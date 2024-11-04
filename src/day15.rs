// Import necessary modules and types from the crate
use crate::utils::coordinate_system::direction::Direction;
use crate::utils::coordinate_system::Coordinate;
use crate::utils::day_setup::Utils;
use crate::utils::grid::unsized_grid::UnsizedGrid;
use crate::utils::grid::Grid;
use std::cmp::Reverse; // For using Reverse in the BinaryHeap
use std::collections::BinaryHeap; // For the priority queue implementation

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/15).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    Utils::run_part_single(part1, 1, 15, Some(498));
    Utils::run_part_single(part2, 2, 15, Some(2901));
}

// Define type aliases for clarity
type Risk = u8; // Type representing the risk value of a position
type MinRisk = u16; // Type representing the minimum risk encountered to reach a position

// Function for part 1, calculating the lowest risk path
fn part1(mut risk_map: RiskMap) -> MinRisk {
    risk_map.lowest_risk() // Calls the method to calculate the lowest risk
}

// Function for part 2, expanding the grid and calculating the lowest risk path
fn part2(risk_map: RiskMap) -> MinRisk {
    risk_map.expand_5x().lowest_risk() // Expands the grid and calculates lowest risk
}

// Struct representing the risk map, which contains the grid and the end coordinate
struct RiskMap {
    grid: UnsizedGrid<(Risk, MinRisk)>, // The grid storing risk values and minimum risk values
    end_coord: Coordinate,              // The end coordinate for the pathfinding
}

impl RiskMap {
    // Creates a new RiskMap instance, initializing the starting risk to 0
    fn new(mut grid: UnsizedGrid<(Risk, MinRisk)>) -> Self {
        // Set the minimum risk at the starting coordinate to 0
        grid.get_mut(&Coordinate::new(0, 0)).unwrap().1 = 0;
        let end_coord = grid.last_coordinate(); // Get the coordinate for the bottom-right corner
        Self { grid, end_coord } // Return the new RiskMap instance
    }

    // Expands the risk map 5 times in both dimensions
    fn expand_5x(self) -> Self {
        // Create a new grid that is 5 times the size of the original
        let mut new_grid = UnsizedGrid::new_with_size(
            self.grid.num_rows() * 5,
            self.grid.num_cols() * 5,
            (0, MinRisk::MAX), // Initialize with default risk and maximum minimum risk
        );

        // Get original grid dimensions
        let original_width = self.grid.num_cols();
        let original_height = self.grid.num_rows();

        // Iterate over the new grid to populate risks
        for row in new_grid.iter_mut() {
            for (position, (risk, _)) in row {
                // Calculate base position in the original grid
                let base_x = position.i % original_width as i32; // Horizontal index
                let base_y = position.j % original_height as i32; // Vertical index
                let target_x = position.i / original_width as i32; // Horizontal expansion index
                let target_y = position.j / original_height as i32; // Vertical expansion index

                // Get the risk from the original grid
                let base_risk = self.grid.get(&Coordinate::new(base_x, base_y)).unwrap().0;

                // Calculate new risk value considering expansion
                *risk = base_risk + target_x as u8 + target_y as u8;

                // Wrap risk value if it exceeds 9
                if *risk > 9 {
                    *risk -= 9; // Ensures risk values remain between 1 and 9
                }
            }
        }

        // Return the new expanded RiskMap
        RiskMap::new(new_grid)
    }

    // Calculates the lowest risk path using Dijkstra's algorithm
    fn lowest_risk(&mut self) -> MinRisk {
        let mut heap = BinaryHeap::<Reverse<(MinRisk, Coordinate)>>::new(); // Priority queue
        heap.push(Reverse((0, Coordinate::new(0, 0)))); // Start with the initial coordinate and risk of 0

        // Process the heap until it is empty
        while let Some(Reverse((acc_risk, coord))) = heap.pop() {
            // Check if the current coordinate is the end coordinate
            if coord == self.end_coord {
                return acc_risk; // Return the accumulated risk if reached the end
            }

            // Iterate through possible directions from the current coordinate
            for direction in Direction::direction_list() {
                let new_coord = coord + direction; // Calculate new coordinate
                if let Some((risk, min_risk)) = self.grid.get_mut(&new_coord) {
                    // Calculate new risk by adding the current risk value
                    let new_risk = acc_risk + *risk as u16;

                    // Update minimum risk if the new risk is lower
                    if new_risk < *min_risk {
                        *min_risk = new_risk; // Update minimum risk at new coordinate
                        heap.push(Reverse((new_risk, new_coord))); // Add new state to the heap
                    }
                }
            }
        }

        unreachable!("There is always a path to the bottom-right corner"); // Safety guarantee
    }
}

// Implementing conversion from a vector of strings to a RiskMap
impl From<Vec<String>> for RiskMap {
    fn from(input: Vec<String>) -> Self {
        // Create a grid from input strings
        let grid = UnsizedGrid::from_box(
            input
                .iter()
                .map(|row| {
                    // Convert each character to its risk value and initialize min_risk
                    row.chars()
                        .map(|c| (c as u8 - b'0', MinRisk::MAX))
                        .collect::<Vec<_>>() // Collect into a Vec
                        .into_boxed_slice() // Convert to boxed slice
                })
                .collect::<Vec<Box<[_]>>>()
                .into_boxed_slice(), // Collect into boxed slice of boxed slices
        );  

        RiskMap::new(grid) // Return a new RiskMap instance
    }
}
