use crate::utils::coordinate_system::direction::FullDirection;
use crate::utils::coordinate_system::Coordinate;
use crate::utils::day_setup;
use crate::utils::grid::sized_grid::SizedGrid;
use day_setup::Utils;
use std::fmt::Debug;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/11).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 11, 1729);
    Utils::run_part_single(part2, 2, 11, 237);
}
const GRID_SIZE: usize = 10;
fn part1(mut octopus_grid: OctopusGrid) -> u64 {
    for _ in 0..100 {
        octopus_grid.raise_energy_levels();
        octopus_grid.process_flashes();
    }

    octopus_grid.num_flashes
}

fn part2(mut octopus_grid: OctopusGrid) -> u64 {
    for i in 0.. {
        octopus_grid.raise_energy_levels();
        let all_flashing = octopus_grid.process_flashes();
        if all_flashing {
            return i + 1;
        }
    }
    unreachable!("The octopus grid should have all grids flashing by now.")
}

#[derive(Debug)]
struct OctopusGrid {
    grid: SizedGrid<EnergyLevel, GRID_SIZE, GRID_SIZE>,
    curr_flashes: Vec<Coordinate>,
    num_flashes: u64,
}

impl OctopusGrid {
    /// Handles the flashes of the octopuses in the grid.
    ///
    /// This function processes the current flashes in the grid by iterating through
    /// the positions in `curr_flashes`. For each position, it checks all adjacent
    /// positions in all directions. If an adjacent position contains an octopus that
    /// is not already flashing and its energy level is raised to the flash level,
    /// it is added to the `curr_flashes` queue. The total number of flashes is updated
    /// accordingly.
    ///
    /// # Returns
    /// `true` if all octopuses are flashing, otherwise `false`.
    pub(crate) fn process_flashes(&mut self) -> bool {
        let mut num_flashes = self.curr_flashes.len();
        while let Some(curr_position) = self.curr_flashes.pop() {
            for dir in FullDirection::full_direction_list() {
                let new_position = curr_position + dir;
                if let Some(e) = self.grid.get_mut(new_position) {
                    if *e != EnergyLevel::Flash && e.raise_energy() {
                        self.curr_flashes.push(new_position);
                        self.num_flashes += 1;
                        num_flashes += 1;
                    }
                }
            }
        }
        num_flashes == GRID_SIZE * GRID_SIZE
    }

    /// Raises the energy levels of all octopuses in the grid.
    ///
    /// This function iterates through each octopus in the grid and raises its energy level.
    /// If an octopus's energy level reaches the flash level, its position is added to the
    /// `curr_flashes` queue, and the total number of flashes is incremented.
    pub(crate) fn raise_energy_levels(&mut self) {
        self.curr_flashes.clear();
        for row in self.grid.iter_mut() {
            for (position, energy) in row {
                if energy.raise_energy() {
                    self.curr_flashes.push(position);
                    self.num_flashes += 1;
                }
            }
        }
    }
}

impl From<Vec<String>> for OctopusGrid {
    fn from(value: Vec<String>) -> Self {
        assert_eq!(
            value.len(),
            GRID_SIZE,
            "Invalid number of rows: {}",
            value.len()
        );
        assert!(
            value.iter().all(|row| row.len() == GRID_SIZE),
            "Invalid number of columns"
        );

        let mut grid = [[EnergyLevel::Flash; GRID_SIZE]; GRID_SIZE];
        for (i, row) in value.iter().enumerate() {
            for (j, e) in row.chars().enumerate() {
                grid[i][j] = EnergyLevel::from(e);
            }
        }
        Self {
            grid: SizedGrid::new(grid),
            num_flashes: 0,
            curr_flashes: Vec::with_capacity(100),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EnergyLevel {
    /// FLASH MODE
    Flash = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl EnergyLevel {
    /// Raises the energy level of the octopus by one.
    ///
    /// This function increments the energy level of the octopus by one and updates
    /// the current energy level. If the new energy level reaches the `FLASH` level,
    /// the function returns `true`.
    ///
    /// # Returns
    /// `true` if the new energy level is `FLASH`, otherwise `false`.
    pub(crate) fn raise_energy(&mut self) -> bool {
        let new_level = EnergyLevel::from(*self as u8 + 1);
        *self = new_level;
        new_level == EnergyLevel::Flash
    }
}

impl From<char> for EnergyLevel {
    fn from(value: char) -> Self {
        match value {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => panic!("Invalid energy level: {}", value),
        }
    }
}

impl From<u8> for EnergyLevel {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Flash,
            _ => panic!("Invalid energy level: {}", value),
        }
    }
}

impl Debug for EnergyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
