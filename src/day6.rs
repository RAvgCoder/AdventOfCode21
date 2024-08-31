use helper_utils::Utils;

use crate::day6::lantern_fish::LanternFish;
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

fn part1(input: Vec<String>) -> u64 {
    const MAX_DAYS_TO_SIMULATE: u16 = 80;
    let mut lantern_fishes = input
        .first()
        .expect("Value should be provided for Lantern Fishes in the format <3,4,3,1,2>")
        .split(',')
        .map(LanternFish::new)
        .collect::<Vec<LanternFish>>();

    for _ in 0..MAX_DAYS_TO_SIMULATE {
        let new_fishes = lantern_fishes
            .iter_mut()
            .filter_map(LanternFish::spawn_lantern_fish_if_ready)
            .collect::<Vec<LanternFish>>();
        lantern_fishes.extend(new_fishes);
    }

    lantern_fishes.len() as u64
}

fn part2(input: Vec<String>) -> u64 {
    const MAX_DAYS_TO_SIMULATE: u16 = 256;

    let mut lantern_fishes_index = [0u64; 9];

    input
        .first()
        .expect("Value should be provided for Lantern Fishes in the format <3,4,3,1,2>")
        .split(',')
        .map(|days_left_before_birth| {
            days_left_before_birth
                .parse::<usize>()
                .expect("Could not parse num of days")
        })
        .for_each(|days_left_before_birth| {
            lantern_fishes_index[days_left_before_birth] += 1;
        });

    for _ in 0..MAX_DAYS_TO_SIMULATE {
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
    const DEFAULT_DAYS_TO_SIMULATE: u8 = 8;
    const DEFAULT_RESTART_DAYS_TO_SIMULATE: u8 = 6;

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

        pub fn spawn_lantern_fish_if_ready(&mut self) -> Option<LanternFish> {
            if self.days_left_before_birth == 0 {
                self.days_left_before_birth = DEFAULT_RESTART_DAYS_TO_SIMULATE;
                Some(LanternFish::default())
            } else {
                self.days_left_before_birth -= 1;
                None
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
}
