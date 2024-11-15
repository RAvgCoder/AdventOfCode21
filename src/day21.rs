use crate::day21::board::Board;
use crate::day21::board::PlayMode;
use crate::day21::die::Dice;
use crate::day21::pawn::Pawn;
use crate::utils::day_setup::Utils;
use std::str::FromStr;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/21).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 21, Some(428736));
    Utils::run_part(part2, 2, 0, Some(444356092776315));
}

fn part1(input: Vec<String>) -> u32 {
    const SCORE: u32 = 1000;
    let player1 = input[0].parse::<Pawn>().unwrap();
    let player2 = input[1].parse::<Pawn>().unwrap();

    Board::new_deterministic(player1, player2, SCORE).play()
}

fn part2(input: Vec<String>) -> u64 {
    const SCORE: u32 = 21;
    let player1 = input[0].parse::<Pawn>().unwrap();
    let player2 = input[1].parse::<Pawn>().unwrap();
    Board::new_quantum(player1, player2, SCORE).play(PlayMode::Recursive)
}

mod die {
    use std::iter::Cycle;
    use std::marker::PhantomData;
    use std::ops::RangeInclusive;

    #[derive(Debug)]
    pub struct Deterministic;
    #[derive(Debug)]
    pub struct Quantum;

    #[derive(Debug)]
    pub struct Dice<T> {
        side: Cycle<RangeInclusive<u16>>,
        num_of_rolls: u16,
        _marker: PhantomData<T>,
    }

    impl<T> Dice<T> {
        pub fn get_num_rolls(&self) -> u16 {
            self.num_of_rolls
        }
    }

    impl Dice<Deterministic> {
        const ROLL_NUM: usize = 3;
        const RANGE: RangeInclusive<u16> = 1..=100;
        pub fn new_deterministic() -> Self {
            Self {
                side: Self::RANGE.clone().cycle(),
                num_of_rolls: 0,
                _marker: PhantomData,
            }
        }

        pub fn next_roll(&mut self) -> u16 {
            self.num_of_rolls += Self::ROLL_NUM as u16;
            self.side.by_ref().take(Self::ROLL_NUM).sum()
        }
    }

    pub type Possibilities = u16;
    impl Dice<Quantum> {
        const ROLL_NUM: usize = 27;

        pub const fn quantum_rolls() -> [Possibilities; Self::ROLL_NUM] {
            let mut rolls = [0; 27];
            let mut index = 0;
            let mut i = 1;
            while i <= 3 {
                let mut j = 1;
                while j <= 3 {
                    let mut k = 1;
                    while k <= 3 {
                        rolls[index] = i + j + k;
                        index += 1;
                        k += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            rolls
        }

        pub fn new_quantum() -> Self {
            const RANGE: RangeInclusive<u16> = 1..=3;
            Self {
                side: RANGE.clone().cycle(),
                num_of_rolls: 0,
                _marker: PhantomData,
            }
        }
    }
}

mod board {
    use super::die::{Deterministic, Quantum};
    use super::{Dice, Pawn};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Board<D> {
        dice: Dice<D>,
        players: [Pawn; 2],
        winning_score: u32,
    }

    pub enum PlayMode {
        Recursive,
        Iterative,
    }

    impl Board<Deterministic> {
        pub fn new_deterministic(player1: Pawn, player2: Pawn, winning_score: u32) -> Self {
            Self {
                dice: Dice::new_deterministic(),
                players: [player1, player2],
                winning_score,
            }
        }

        pub fn play(self) -> u32 {
            let Self {
                mut dice,
                mut players,
                winning_score,
                ..
            } = self;

            let mut player_choice = (0..=1).cycle();
            while !players[0].has_won(winning_score) && !players[1].has_won(winning_score) {
                let next_roll = dice.next_roll();
                let current_player = player_choice.next().unwrap();
                let pawn = &mut players[current_player];
                pawn.update_score(next_roll);
            }

            let winner = if players[0].has_won(winning_score) {
                &players[1]
            } else {
                &players[0]
            };

            winner.score() * dice.get_num_rolls() as u32
        }
    }

    impl Board<Quantum> {
        pub fn new_quantum(player1: Pawn, player2: Pawn, winning_score: u32) -> Self {
            Self {
                dice: Dice::new_quantum(),
                players: [player1, player2],
                winning_score,
            }
        }

        pub fn play(self, play_mode: PlayMode) -> u64 {
            let mut number_of_wins = [0, 0];

            let Self { players, .. } = self;

            match play_mode {
                PlayMode::Recursive => {
                    let mut memo = HashMap::new();
                    number_of_wins = Self::play_recursively(players, self.winning_score, &mut memo)
                }
                PlayMode::Iterative => {}
            }

            number_of_wins.into_iter().max().unwrap()
        }

        fn play_recursively(
            [player1, player2]: [Pawn; 2],
            score: u32,
            memo: &mut HashMap<(Pawn, Pawn), [u64; 2]>,
        ) -> [u64; 2] {
            if player1.has_won(score) {
                return [1, 0];
            } else if player2.has_won(score) {
                return [0, 1];
            }

            let state = (player1.clone(), player2.clone());
            if let Some(&result) = memo.get(&state) {
                return result;
            }

            let mut player1_wins = 0;
            let mut player2_wins = 0;

            for rolls in Dice::quantum_rolls() {
                let mut new_player1 = player1.clone();
                new_player1.update_score(rolls);

                let [p2_wins, p1_wins] =
                    Self::play_recursively([player2.clone(), new_player1], score, memo);

                player1_wins += p1_wins;
                player2_wins += p2_wins;
            }

            let result = [player1_wins, player2_wins];
            memo.insert(state, result);
            result
        }
    }
}
mod pawn {
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    pub struct Pawn {
        curr_position: u8,
        score: u32,
    }
    impl Pawn {
        pub fn new(curr_position: u8) -> Result<Self, &'static str> {
            Ok(Self {
                curr_position,
                score: 0,
            })
        }

        pub fn score(&self) -> u32 {
            self.score
        }

        pub fn has_won(&self, score: u32) -> bool {
            self.score >= score
        }

        pub fn update_score(&mut self, roll: u16) {
            self.curr_position = match (self.curr_position as u16 + roll) % 10 {
                0 => 10,
                n => n as u8,
            };
            self.score += self.curr_position as u32;
        }
    }
}
impl FromStr for Pawn {
    type Err = &'static str;

    fn from_str(player: &str) -> Result<Self, Self::Err> {
        // Player 1 starting position: 4
        const SKIP_LEN: usize = "Player 1 starting position: ".len();
        let (_, num) = player.split_at(SKIP_LEN);
        Pawn::new(num.parse().map_err(|_| {
            "\
            Format did not match format:
                 Player 1 starting position: 4\
            "
        })?)
    }
}
