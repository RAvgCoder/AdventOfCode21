use helper_utils::Utils;

use crate::day6::lantern_fish::LanternFishList;
use crate::utils::helper_utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/6).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 6, 396210);
    Utils::run_part(part2, 2, 6, 1770823541496);
}

fn part1(input: Vec<LanternFishList>) -> u64 {
    assert_eq!(input.len(), 1, "Only one list of lantern fishes should be provided");
    const MAX_DAYS_TO_SIMULATE: u16 = 80;

    simulate_days(input.first().unwrap(), MAX_DAYS_TO_SIMULATE)
}

fn part2(input: Vec<LanternFishList>) -> u64 {
    assert_eq!(input.len(), 1, "Only one list of lantern fishes should be provided");

    const MAX_DAYS_TO_SIMULATE: u16 = 256;

    simulate_days(input.first().unwrap(), MAX_DAYS_TO_SIMULATE)
}

fn simulate_days(lantern_fish_list: &LanternFishList, max_days_to_simulate: u16) -> u64 {
    let mut lantern_fishes_index = [0u64; 9];

    lantern_fish_list.fishes.iter()
        .for_each(|lantern_fish| {
            lantern_fishes_index[lantern_fish.days_left_before_birth as usize] += 1;
        });

    for _ in 0..max_days_to_simulate {
        // Find the number of new fishes to be born
        let new_fishes = lantern_fishes_index[0];

        // Decrease the number of days left till birth for all fishes
        lantern_fishes_index.rotate_left(1);

        // Reset timer for all fishes which have given birth
        lantern_fishes_index[6] += new_fishes;

        // Give birth to new fishes
        lantern_fishes_index[8] = new_fishes;
    }

    lantern_fishes_index.iter().sum()
}

mod lantern_fish {
    use std::str::FromStr;

    const DEFAULT_DAYS_TO_SIMULATE: u8 = 8;

    #[derive(Debug)]
    #[repr(transparent)]
    pub struct LanternFish {
        pub days_left_before_birth: u8,
    }

    impl LanternFish {
        pub fn new(days_left_before_birth: &str) -> LanternFish {
            LanternFish {
                days_left_before_birth: days_left_before_birth
                    .parse::<u8>()
                    .expect("Could not parse num of days"),
            }
        }
    }

    impl Default for LanternFish {
        #[inline(always)]
        fn default() -> Self {
            LanternFish {
                days_left_before_birth: DEFAULT_DAYS_TO_SIMULATE,
            }
        }
    }

    pub struct LanternFishList {
        pub fishes: Box<[LanternFish]>,
    }

    impl FromStr for LanternFishList {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok(LanternFishList {
                fishes: input
                    .split(',')
                    .map(LanternFish::new)
                    .collect::<Vec<LanternFish>>()
                    .into_boxed_slice()
            })
        }
    }
}
