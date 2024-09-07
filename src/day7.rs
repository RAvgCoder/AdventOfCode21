use day_setup::Utils;
use std::str::FromStr;

use crate::utils::day_setup;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/7).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 7, 356992);
    Utils::run_part(part2, 2, 7, 101268110);
}

/// Solves part 1 of the puzzle.
///
/// # Arguments
/// * `input` - A vector of `CrabPositions`.
///
/// # Returns
/// * `u64` - The minimum fuel cost.
fn part1(input: Vec<CrabPositions>) -> u64 {
    let adjustment_function = |distance: u64| distance;
    assert_eq!(input.len(), 1, "Expected only one crab position");
    min_fuel_cost(input.first().unwrap(), adjustment_function)
}

/// Solves part 2 of the puzzle.
///
/// # Arguments
/// * `input` - A vector of `CrabPositions`.
///
/// # Returns
/// * `u64` - The minimum fuel cost.
fn part2(input: Vec<CrabPositions>) -> u64 {
    let adjustment_function = |distance: u64| {
        let n = distance + 1;
        // Σ n Whole Numbers formula:
        (n * (n - 1)) / 2
    };
    assert_eq!(input.len(), 1, "Expected only one crab position");
    min_fuel_cost(input.first().unwrap(), adjustment_function)
}

/// Calculates the minimum fuel cost to align all crab positions.
///
/// # Arguments
/// * `horizontal_positions` - A reference to `CrabPositions`.
/// * `adjustment_function` - A function that adjusts the distance.
///
/// # Returns
/// * `u64` - The minimum fuel cost.
fn min_fuel_cost(horizontal_positions: &CrabPositions, adjustment_function: fn(u64) -> u64) -> u64 {
    (0..horizontal_positions.positions.len())
        .map(|idx| horizontal_positions.find_distance(idx + 1, adjustment_function))
        .min()
        .unwrap_or(0)
}

#[repr(transparent)]
#[derive(Debug)]
/// Represents the positions of crabs.
struct CrabPositions {
    positions: Box<[u32]>,
}

impl CrabPositions {
    /// Finds the total distance for aligning all crab positions to a specific position.
    ///
    /// # Arguments
    /// * `pos` - The index of the position to align to.
    /// * `distance_alignment_function` - A function that calculates the distance between two positions.
    ///
    /// # Returns
    /// * `u64` - The total distance for aligning all crab positions to the specified position.
    pub fn find_distance<F>(&self, pos: usize, distance_alignment_function: F) -> u64
    where
        F: Fn(u64) -> u64,
    {
        self.positions
            .iter()
            .map(|&e| distance_alignment_function((e as i64 - pos as i64).unsigned_abs()))
            .sum()
    }
}

impl FromStr for CrabPositions {
    type Err = &'static str;

    /// Parses a string into `CrabPositions`.
    ///
    /// # Arguments
    /// * `s` - A string slice containing comma-separated crab positions.
    ///
    /// # Returns
    /// * `Result<CrabPositions, Self::Err>` - A result containing `CrabPositions` or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CrabPositions {
            positions: s
                .split(',')
                .map(|str| str.parse::<u32>().expect("Could not parse crab positions"))
                .collect::<Vec<u32>>()
                .into_boxed_slice(),
        })
    }
}
