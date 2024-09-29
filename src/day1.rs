use day_setup::Utils;

use crate::utils::day_setup;

/// Runs the Advent of Code puzzles.
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///  If the result of any part does not match the expected value.
pub fn run() {
    // run_part(function, part_num, day_num)
    Utils::run_part(part1, 1, 1, Some(1462));
    Utils::run_part(part2, 2, 1, Some(1497));
}

/// Solves part 1 of the Day 1 puzzle.
///
/// This function counts the number of times a depth measurement increases from the previous measurement.
///
/// # Arguments
/// * `read_file` - A vector of depth measurements.
///
/// # Returns
/// The count of measurements that are larger than the previous measurement.
fn part1(read_file: Vec<u16>) -> u64 {
    let mut depth = u16::MAX;
    let mut count = 0;
    for i in read_file {
        if depth < i {
            count += 1;
        }
        depth = i;
    }

    count
}

/// Solves part 2 of the Day 1 puzzle.
///
/// This function counts the number of times the sum of measurements in a three-measurement sliding window increases from the previous sum.
///
/// # Arguments
/// * `read_file` - A vector of depth measurements.
///
/// # Returns
/// The count of sums that are larger than the previous sum.
fn part2(read_file: Vec<i16>) -> u64 {
    let mut depth = i16::MAX;
    let mut count = 0;
    for window in read_file.windows(3) {
        let n: i16 = window.iter().sum();
        if depth < n {
            count += 1;
        }
        depth = n;
    }
    count
}
