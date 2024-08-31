use helper_utils::Utils;

use crate::utils::helper_utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/8).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 0, 0);
    Utils::run_part(part2, 2, 0, 0);
}

fn part1(input: Vec<String>) -> u64 {
    println!("Part 1: {:?}", input);
    return 0;
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:?}", input);
    return 0;
}
                
