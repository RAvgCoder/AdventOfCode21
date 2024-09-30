use crate::utils::coordinate_system::direction::Direction;
use crate::utils::coordinate_system::Coordinate;
use crate::utils::day_setup;
use crate::utils::grid::unsized_grid::UnsizedGrid;
use crate::utils::grid::Grid;
use day_setup::Utils;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/9).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 9, Some(486));
    Utils::run_part_single(part2, 2, 9, Some(1059300));
}

/// Part 1 of the puzzle, which finds the smallest points in the height map.
///
/// # Arguments
/// * `height_map` - A reference to the `HeightMap` containing the height data.
///
/// # Returns
/// The sum of the risk levels of all the smallest points.
fn part1(height_map: HeightMap) -> u64 {
    height_map.grid.foreach(|pos, e: &u8, sum: &mut u64| {
        if height_map.is_lowest_point(pos) {
            *sum += *e as u64 + 1
        }
    })
}

/// Part 2 of the puzzle, which finds the largest basins in the height map.
///
/// # Arguments
/// * `height_map` - A reference to the `HeightMap` containing the height data.
///
/// # Returns
/// The product of the sizes of the three largest basins.
fn part2(height_map: HeightMap) -> u64 {
    height_map
        .grid
        .foreach(|pos, _, acc: &mut [u64; 3]| {
            if height_map.is_lowest_point(pos) {
                let mut queue = VecDeque::new();
                let mut has_visited = HashSet::new();
                queue.push_back(pos);

                while let Some(pos) = queue.pop_front() {
                    if !has_visited.insert(pos) {
                        continue;
                    }
                    has_visited.insert(pos);
                    for dir in Direction::direction_list() {
                        let position = pos + dir;
                        if let Some(&new_height) = height_map.get(&position) {
                            if new_height < HeightMap::HIGHEST_POINT {
                                queue.push_back(position);
                            }
                        }
                    }
                }

                if let Some(min) = acc.iter_mut().min() {
                    let new_val = has_visited.len() as u64;
                    if *min < new_val {
                        *min = new_val;
                    }
                } else {
                    panic!("Could not find min value")
                }
            }
        })
        .iter()
        .product::<u64>()
}

/// Represents a height map for the puzzle.
#[derive(Debug)]
struct HeightMap {
    grid: UnsizedGrid<u8>,
}

impl HeightMap {
    pub const HIGHEST_POINT: u8 = 9;

    /// Gets the height at a specific position.
    ///
    /// # Arguments
    /// * `position` - The position to get the height from.
    ///
    /// # Returns
    /// An `Option` containing a reference to the height value, or `None` if the position is invalid.
    #[inline(always)]
    fn get(&self, position: &Coordinate) -> Option<&u8> {
        self.grid.get(position)
    }

    /// Checks if a position is the lowest point in its neighborhood.
    ///
    /// # Arguments
    /// * `position` - The position to check.
    ///
    /// # Returns
    /// `true` if the position is the lowest point, `false` otherwise.
    fn is_lowest_point(&self, position: Coordinate) -> bool {
        let curr_height = *self.get(&position).unwrap();
        for dir in Direction::direction_list() {
            let new_pos = position + dir;
            if let Some(new_height) = self.get(&new_pos) {
                if *new_height <= curr_height {
                    return false;
                }
            }
        }

        true
    }
}

impl From<Vec<String>> for HeightMap {
    /// Creates a `HeightMap` from a vector of strings.
    ///
    /// # Arguments
    /// * `value` - A vector of strings representing the height map.
    ///
    /// # Returns
    /// A `HeightMap` created from the input strings.
    fn from(value: Vec<String>) -> Self {
        let grid = value
            .iter()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
                    .into_boxed_slice()
            })
            .collect::<Vec<Box<[u8]>>>()
            .into_boxed_slice();

        Self {
            grid: UnsizedGrid::from_box(grid),
        }
    }
}
