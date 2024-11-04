use crate::utils::day_setup::Utils;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/19).
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

fn part1(input: ScannerList) -> u64 {
    println!("Part 1: {:#?}", input);
    0
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}


struct ScannerList {
    scanners: Vec<Scanner>,
}


struct Scanner {
    name: u16,
    beacons: HashSet<Beacon>,
}

#[derive( Hash, Copy, Clone, Eq, PartialEq)]
struct Beacon(i32, i32, i32);


impl Debug  for Beacon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?},{:?},{:?}", self.0, self.1, self.2)
    }
}

impl Debug for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- scanner {} ---", self.name)?;
        for beacon in &self.beacons {
            writeln!(f, "{:?}", beacon)?;
        }
        
        Ok(())
    }
}

impl Debug for ScannerList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for s in &self.scanners {
            writeln!(f, "{:?}", s)?;
        }
        Ok(())
    }
}


impl From<Vec<String>> for ScannerList {
    fn from(value: Vec<String>) -> Self {
        let mut scanners = vec![];

        let mut beacons = HashSet::new();
        let mut count = 0;
        for line in value {
            if line.starts_with("---") {
                // Skip
            } else if line.is_empty() {
                scanners.push(Scanner {
                    name: count,
                    beacons,
                });
                count += 1;
                beacons = HashSet::new();
            } else { // The actual beacon information 
                let mut beacon_info = line.split(',');
                beacons.insert(Beacon(
                    beacon_info.next().unwrap().parse::<i32>().unwrap(),
                    beacon_info.next().unwrap().parse::<i32>().unwrap(),
                    beacon_info.next().unwrap().parse::<i32>().unwrap(),
                ));
            }
        }
        
        if !beacons.is_empty() {
            scanners.push(Scanner {
                name: count,
                beacons,
            });
        }

        ScannerList { scanners }
    }
}