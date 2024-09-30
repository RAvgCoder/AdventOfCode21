use crate::utils::day_setup::Utils;
use crate::utils::grid::unsized_grid::UnsizedGrid;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/15).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 0, None);
    Utils::run_part(part2, 2, 0, None);
}

fn part1(input: RiskMap) -> u64 {
    println!("Part 1: {:#?}", input);
    0
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

#[derive(Debug)]
struct RiskMap {
    grid: UnsizedGrid<u8>,
}

impl From<Vec<String>> for RiskMap {
    fn from(input: Vec<String>) -> Self {
        let grid = UnsizedGrid::from_box(
            input
                .iter()
                .map(|row| {
                    row.chars()
                        .map(|c| c as u8 - b'0')
                        .collect::<Vec<u8>>()
                        .into_boxed_slice()
                })
                .collect::<Vec<Box<[u8]>>>()
                .into_boxed_slice(),
        );
        Self { grid }
    }
}
