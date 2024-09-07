use crate::utils::coordinate::direction::FullDirection;
use crate::utils::coordinate::Position;
use crate::utils::day_setup;
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
    Utils::run_part_single(part1, 1, 0, 0);
    Utils::run_part(part2, 2, 0, 0);
}

fn part1(mut octopus_grid: OctopusGrid) -> u64 {
    println!("Part 1: {:#?}", octopus_grid);
    let mut falshes = 0;
    for _ in 0..100 {
        octopus_grid.raise_energy_levels();
        if octopus_grid.has_flashes() {
            // flashses += octopus_grid.handle_flashes();
        }
    }
    0
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

struct OctopusGrid {
    grid: [[EnergyLevel; 10]; 10],
    curr_flashes: Vec<Position>,
}

impl OctopusGrid {
    pub(crate) fn handle_flashes(&mut self) -> u64 {
        while let Some(curr_position) = self.curr_flashes.pop() {
            for dir in FullDirection::full_direction_list() {
                // let new_position = curr_position + dir;
                // if let Some(e) = self.grid[new_position.i as usize][new_position.j as usize] {
                //     e.raise_energy();
                // }
            }
        }
        0
    }

    fn raise_energy_levels(&mut self) {
        self.curr_flashes.clear();
        for (i, row) in self.grid.iter_mut().enumerate() {
            for (j, energy) in row.iter_mut().enumerate() {
                if energy.raise_energy() {
                    self.curr_flashes.push(Position::new(i as i32, j as i32));
                }
            }
        }
    }

    fn has_flashes(&self) -> bool {
        !self.curr_flashes.is_empty()
    }
}

impl From<Vec<String>> for OctopusGrid {
    fn from(value: Vec<String>) -> Self {
        assert_eq!(value.len(), 10, "Invalid number of rows: {}", value.len());
        assert!(
            value.iter().all(|row| row.len() == 10),
            "Invalid number of columns"
        );

        let mut grid = [[EnergyLevel::ZERO; 10]; 10];
        for (i, row) in value.iter().enumerate() {
            for (j, e) in row.chars().enumerate() {
                grid[i][j] = EnergyLevel::from(e);
            }
        }
        Self {
            grid,
            curr_flashes: Vec::with_capacity(100),
        }
    }
}

impl Debug for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.grid.iter() {
            for e in row.iter() {
                write!(f, "{:?} ", *e as u8)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnergyLevel {
    /// FLASH MODE
    ZERO = 0,
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
        new_level == EnergyLevel::ZERO
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
            10 => Self::ZERO,
            _ => panic!("Invalid energy level: {}", value),
        }
    }
}
