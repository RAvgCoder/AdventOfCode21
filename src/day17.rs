use crate::utils::day_setup::Utils;
use std::ops::RangeInclusive;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/17).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 17, Some(6555));
    Utils::run_part_single(part2, 2, 17, Some(4973));
}

fn part1(target_area: TargetArea) -> u32 {
    target_area.max_height()
}

fn part2(target_area: TargetArea) -> u16 {
    target_area.num_of_initial_velocities()
}

#[derive(Debug)]
struct TargetArea {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl TargetArea {
    fn max_height(&self) -> u32 {
        (0..=(*self.y.start()).unsigned_abs() - 1).sum()
    }

    fn num_of_initial_velocities(&self) -> u16 {
        let mut count = 0;

        for y in *self.y.start()..=1 - *self.y.start() {
            for x in 0..=*self.x.end() {
                let mut x_pos = 0;
                let mut y_pos = 0;

                for i in 0..1000 {
                    y_pos += y - i;

                    if x - i > 0 {
                        x_pos += x - i;
                    }
                    if *self.y.start() <= y_pos
                        && y_pos <= *self.y.end()
                        && *self.x.start() <= x_pos
                        && x_pos <= *self.x.end()
                    {
                        count += 1;
                        break;
                    }
                }
            }
        }

        count
    }
}

impl From<Vec<String>> for TargetArea {
    fn from(input: Vec<String>) -> Self {
        let (x_raw, y_raw) = input.first().unwrap()[13..].trim().split_once(',').unwrap();

        let (x1, x2) = x_raw[2..].split_once("..").unwrap();
        let (y1, y2) = y_raw.trim()[2..].split_once("..").unwrap();

        TargetArea {
            x: x1.parse::<i32>().unwrap()..=x2.parse::<i32>().unwrap(),
            y: y1.parse::<i32>().unwrap()..=y2.parse::<i32>().unwrap(),
        }
    }
}
