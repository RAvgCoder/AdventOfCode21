use helper_utils::Utils;
use std::str::FromStr;

use crate::utils::helper_utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/8).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 8, 0);
    Utils::run_part(part2, 2, 0, 0);
}

fn part1(segment_list: Vec<SignalSegment>) -> u64 {
    const SEGMENTS_COUNT: [u8; 4] = [
        ClockNumber::One.count_segments(),
        ClockNumber::Four.count_segments(),
        ClockNumber::Seven.count_segments(),
        ClockNumber::Eight.count_segments(),
    ];

    segment_list
        .iter()
        .map(|signal_segment: &SignalSegment| {
            signal_segment
                .output_value
                .iter()
                .filter(|digit| {
                    let digit_len = digit.len() as u8;
                    SEGMENTS_COUNT.iter().any(|&count| count == digit_len)
                })
                .count() as u64
        })
        .sum()
}

fn part2(input: Vec<SignalSegment>) -> u64 {
    // println!("Part 2 {:#?}", input);
    0
}

/// Represents the numbers on a 7-segment clock display.
///
/// Each variant corresponds to a digit from 0 to 9, with the value being a bitmask
/// that indicates which segments are turned on or off.
///
/// # Variants
/// * `Zero` - Represents the digit 0 (segments: a, b, c, e, f, g).
/// * `One` - Represents the digit 1 (segments: c, f).
/// * `Two` - Represents the digit 2 (segments: a, c, d, e, g).
/// * `Three` - Represents the digit 3 (segments: a, c, d, f, g).
/// * `Four` - Represents the digit 4 (segments: b, c, d, f).
/// * `Five` - Represents the digit 5 (segments: a, b, d, f, g).
/// * `Six` - Represents the digit 6 (segments: a, b, d, e, f, g).
/// * `Seven` - Represents the digit 7 (segments: a, c, f).
/// * `Eight` - Represents the digit 8 (all segments).
/// * `Nine` - Represents the digit 9 (segments: a, b, c, d, f, g).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ClockNumber {
    Zero = 0b1110111,
    One = 0b0010010,
    Two = 0b1011101,
    Three = 0b1011011,
    Four = 0b0111010,
    Five = 0b1101011,
    Six = 0b1101111,
    Seven = 0b1010010,
    Eight = 0b1111111,
    Nine = 0b1111011,
}

impl ClockNumber {
    /// Returns the number of segments that are turned on for the given digit.
    ///
    /// # Arguments
    /// * `digit` - The digit to check.
    ///
    /// # Returns
    /// The number of segments that are turned on for the given digit.
    #[inline(always)]
    const fn count_segments(&self) -> u8 {
        (*self as u8).count_ones() as u8
    }
}

struct SignalSegment {
    unique_signal_patterns: [String; 10],
    output_value: [String; 4],
}

impl FromStr for SignalSegment {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut signal_patterns: [String; 10] = core::array::from_fn(|_| String::new());
        let mut output: [String; 4] = core::array::from_fn(|_| String::new());
        let (unique_signal_patterns, output_value) =
            input.split_once('|').expect("Invalid input format");

        unique_signal_patterns
            .split_whitespace()
            .enumerate()
            .for_each(|(idx, input)| {
                signal_patterns[idx] = input.to_string();
            });
        
        output_value
            .split_whitespace()
            .enumerate()
            .for_each(|(idx, input)| {
                output[idx] = input.to_string();
            });

        Ok(SignalSegment {
            unique_signal_patterns: signal_patterns,
            output_value: output,
        })
    }
}
