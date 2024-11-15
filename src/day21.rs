use crate::day21::board::Board;
use crate::day21::die::Die;
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
    Utils::run_part(part2, 2, 0, None);
}

fn part1(input: Vec<String>) -> u32 {
    const SCORE: u32 = 1000;
    let player1 = input[0].parse::<Pawn>().unwrap();
    let player2 = input[1].parse::<Pawn>().unwrap();

    Board::new_deterministic(player1, player2)
        .play_till_score(SCORE)
        .result()
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

mod die {
    use std::iter::Cycle;
    use std::marker::PhantomData;
    use std::ops::RangeInclusive;

    #[derive(Debug)]
    pub struct Deterministic;

    #[derive(Debug)]
    pub struct Die<T> {
        side: Cycle<RangeInclusive<u16>>,
        num_of_rolls: u16,
        _marker: PhantomData<T>,
    }

    impl<T> Die<T> {
        pub fn get_num_rolls(&self) -> u16 {
            self.num_of_rolls
        }
    }

    const RANGE: RangeInclusive<u16> = 1..=100;
    const ROLL_NUM: usize = 3;
    impl Die<Deterministic> {
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
}

mod board {
    use super::die::Deterministic;
    use super::{Die, Pawn};

    pub struct InPlay;
    pub struct GameOver;
    #[derive(Debug)]
    pub struct Board<T, D> {
        die: Die<D>,
        player: [Pawn; 2],
        // Track who wins
        winner: u8,
        _marker: std::marker::PhantomData<T>,
    }

    impl Board<InPlay, Deterministic> {
        pub fn new_deterministic(player1: Pawn, player2: Pawn) -> Self {
            Self {
                die: Die::new_deterministic(),
                player: [player1, player2],
                winner: 0,
                _marker: std::marker::PhantomData,
            }
        }

        pub fn play_till_score(self, score: u32) -> Board<GameOver, Deterministic> {
            let Self {
                mut die,
                mut player,
                ..
            } = self;

            let mut player_choice = (0..=1).cycle();
            while !player[0].has_won(score) && !player[1].has_won(score) {
                let next_roll = die.next_roll();
                let i = player_choice.next().unwrap();
                let pawn = &mut player[i];
                pawn.update_score(next_roll);
            }

            // Create a new `Board` in the `GameOver` state
            Board {
                winner: if player[0].has_won(score) { 1 } else { 0 },
                die,
                player,
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl Board<GameOver, Deterministic> {
        pub fn result(&self) -> u32 {
            let pawn = &self.player[self.winner as usize];
            pawn.score * self.die.get_num_rolls() as u32
        }
    }
}
#[derive(Debug, Clone)]
struct Pawn {
    curr_pos: u8,
    score: u32,
}

impl Pawn {
    fn has_won(&self, score: u32) -> bool {
        self.score >= score
    }

    fn update_score(&mut self, score: u16) {
        self.curr_pos = match (self.curr_pos as u16 + score) % 10 {
            0 => 10,
            n => n as u8,
        };
        self.score += self.curr_pos as u32;
    }
}

impl FromStr for Pawn {
    type Err = &'static str;

    fn from_str(player: &str) -> Result<Self, Self::Err> {
        // Player 1 starting position: 4
        const SKIP_LEN: usize = "Player 1 starting position: ".len();
        let (_, num) = player.split_at(SKIP_LEN);
        Ok(Self {
            curr_pos: num.parse().map_err(|_| {
                "\
            Format did not match format:
                 Player 1 starting position: 4\
            "
            })?,
            score: 0,
        })
    }
}
