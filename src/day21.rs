use crate::day21::board::Board;
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

    Board::new_deterministic(player1, player2).play_till_score(SCORE)
}

fn part2(input: Vec<String>) -> u64 {
    const SCORE: u32 = 21;
    let player1 = input[0].parse::<Pawn>().unwrap();
    let player2 = input[1].parse::<Pawn>().unwrap();
    Board::new_quantum(player1, player2).play_till_score(SCORE)
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

    const RANGE: RangeInclusive<u16> = 1..=100;
    const ROLL_NUM: usize = 3;
    impl Dice<Deterministic> {
        pub fn new_deterministic() -> Self {
            Self {
                side: RANGE.clone().cycle(),
                num_of_rolls: 0,
                _marker: PhantomData,
            }
        }

        pub fn next_roll(&mut self) -> u16 {
            self.num_of_rolls += ROLL_NUM as u16;
            self.side.by_ref().take(ROLL_NUM).sum()
        }
    }

    pub type Possibilities = u16;
    impl Dice<Quantum> {
        pub fn new_quantum() -> Self {
            Self {
                side: RANGE.clone().cycle(),
                num_of_rolls: 0,
                _marker: PhantomData,
            }
        }

        pub fn next_roll(&mut self) -> [Possibilities; 3] {
            self.num_of_rolls += ROLL_NUM as u16;
            self.side
                .by_ref()
                .take(ROLL_NUM)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }
    }
}

mod board {
    use super::die::{Deterministic, Possibilities, Quantum};
    use super::{Dice, Pawn};
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct Board<D> {
        dice: Dice<D>,
        players: [Pawn; 2],
    }

    impl Board<Deterministic> {
        pub fn new_deterministic(player1: Pawn, player2: Pawn) -> Self {
            Self {
                dice: Dice::new_deterministic(),
                players: [player1, player2],
            }
        }

        pub fn play_till_score(self, score: u32) -> u32 {
            let Self {
                mut dice,
                mut players,
                ..
            } = self;

            let mut player_choice = (0..=1).cycle();
            while !players[0].has_won(score) && !players[1].has_won(score) {
                let next_roll = dice.next_roll();
                let current_player = player_choice.next().unwrap();
                let pawn = &mut players[current_player];
                pawn.update_score(next_roll);
            }

            let winner = if players[0].has_won(score) {
                &players[1]
            } else {
                &players[0]
            };
            winner.score() * dice.get_num_rolls() as u32
        }
    }

    impl Board<Quantum> {
        pub fn new_quantum(player1: Pawn, player2: Pawn) -> Self {
            Self {
                dice: Dice::new_quantum(),
                players: [player1, player2],
            }
        }

        pub fn play_till_score(self, score: u32) -> u64 {
            let Self {
                mut dice, players, ..
            } = self;

            let mut number_of_wins = [0, 0];

            let mut players_universe = {
                let mut p = VecDeque::with_capacity((score * 3) as usize);
                p.extend(players);
                p
            };

            while let Some(curr_pawn) = players_universe.pop_front() {
                let next_roll = dice.next_roll();
                let new_pawns = Self::split_piece(next_roll, curr_pawn);
                new_pawns.into_iter().for_each(|pawn| {
                    if !pawn.has_won(score) {
                        players_universe.push_back(pawn);
                    } else {
                        number_of_wins[pawn.player_id() as usize] += 1;
                    }
                });
            }
            number_of_wins.into_iter().max().unwrap()
        }

        fn split_piece(next_roll: [Possibilities; 3], pawn: Pawn) -> [Pawn; 3] {
            next_roll.map(|possibilities| {
                let mut pawn = pawn.clone();
                pawn.update_score(possibilities);
                pawn
            })
        }
    }
}
mod pawn {
    use std::sync::atomic::{AtomicU8, Ordering};

    static CURRENT_ID: AtomicU8 = AtomicU8::new(0);
    #[derive(Debug, Clone)]
    pub struct Pawn {
        player_id: u8,
        curr_position: u8,
        score: u32,
    }
    impl Pawn {
        pub fn new(curr_position: u8) -> Result<Self, &'static str> {
            Ok(Self {
                curr_position,
                score: 0,
                player_id: unsafe { Pawn::give_id()? },
            })
        }

        pub fn score(&self) -> u32 {
            self.score
        }

        pub fn player_id(&self) -> u8 {
            self.player_id
        }

        pub fn has_won(&self, score: u32) -> bool {
            self.score >= score
        }

        /// Generates a unique player ID for each `Pawn` instance.
        ///
        /// This function uses an atomic counter to ensure that each player gets a unique ID.
        /// It is marked as `unsafe` because it relies on a global mutable state.
        ///
        /// # Returns
        /// - `Ok(u8)`: The unique player ID.
        /// - `Err(&'static str)`: An error if more than 2 players are created.
        ///
        /// # Safety
        /// This function is `unsafe` because it manipulates a global atomic counter, which can lead to
        /// undefined behavior if not used correctly. It should only be called in a controlled manner
        /// where the number of players does not exceed 2.
        unsafe fn give_id() -> Result<u8, &'static str> {
            let curr_id = CURRENT_ID.load(Ordering::Relaxed);
            let result = if curr_id > 1 {
                Err("Can only hand out to 2 players")?
            } else {
                Ok(curr_id)
            };
            CURRENT_ID.fetch_add(1, Ordering::Relaxed);
            result
        }

        pub fn update_score(&mut self, score: u16) {
            self.curr_position = match (self.curr_position as u16 + score) % 10 {
                0 => 10,
                n => n as u8,
            };
            self.score += self.curr_position as u32;
        }
    }

    impl Drop for Pawn {
        /// Decrements the global player ID counter when a `Pawn` instance is dropped.
        ///
        /// This function is called automatically when a `Pawn` instance goes out of scope.
        /// It uses an atomic operation to safely decrement the global `CURRENT_ID` counter.
        fn drop(&mut self) {
            CURRENT_ID.fetch_sub(1, Ordering::Relaxed);
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
