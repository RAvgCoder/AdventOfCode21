use std::{u32, u64};

use helper_utils::Utils;

use crate::utils::helper_utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/7).
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
    let horizontal_positions = input
        .first()
        .expect("Expected input to be of format '16,1,2,0,4,2,7,1,2,14'")
        .split(',')
        .map(|str| str.parse::<u16>().expect("Could not horizontal positions"))
        .collect::<Vec<u16>>();

    find_cheapest_fuel(horizontal_positions) as u64
}

#[inline]
fn find_cheapest_fuel(horizontal_positions: Vec<u16>) -> u32 {
    find_cheapest_fuel_helper(horizontal_positions, 0)
}

fn find_cheapest_fuel_helper(horizontal_positions: Vec<u16>, curr_fuel_used: u32) -> u32 {
    let mut cheapest_fuel = if horizontal_positions.is_empty() { curr_fuel_used } else { u32::MAX };
    for (i, curr) in horizontal_positions.iter().enumerate() {
        let mut new_positions = horizontal_positions.clone();
        new_positions.swap_remove(i);
        cheapest_fuel = cheapest_fuel.min(find_cheapest_fuel_helper(
            new_positions,
            (curr_fuel_used as i32 - *curr as i32).unsigned_abs(),
        ));
    }
    cheapest_fuel
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:?}", input);
    0
}
