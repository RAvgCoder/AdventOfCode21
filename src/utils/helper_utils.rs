use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::Instant;

/// Utility struct containing various helper functions.
pub struct Utils;

impl Utils {
    const AOC_YEAR: u16 = 21; // 2021

    /// Executes a function with a list of data and measures its execution time.
    ///
    /// # Arguments
    /// * `day_func_part_to_run` - The function to be executed.
    /// * `part_num` - The part number of the puzzle.
    /// * `day_num` - The day number of the puzzle.
    /// * `expected` - The expected result for assertion.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the input vector.
    /// * `F` - The function type that takes a vector of `T` and returns a `u64`.
    ///
    /// # Panics    
    ///  if the expected result does not match the actual result.
    pub fn run_part<T, F>(day_func_part_to_run: F, part_num: i32, day_num: u8, expected: u64)
    where
        F: FnOnce(Vec<T>) -> u64,
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        println!(
            "//------------[Day {} Part {}]------------\\\\",
            day_num, part_num
        );
        let read_file = Self::read_file::<T>(day_num);
        let start_time = Instant::now();
        let result = day_func_part_to_run(read_file);
        let elapsed_time = start_time.elapsed();

        // The assumption is that no advent of code answer is to ever be zero cuz that'll be boring
        if expected != 0 && result != expected {
            println!(
                r#"
Assertion Failed
----------------
Expected: {}
Found: {}
            "#,
                expected, result
            );
            std::process::exit(1);
        }

        // Convert to milliseconds and microseconds
        let millis = elapsed_time.as_millis();
        let micros = elapsed_time.as_micros() % 1_000; // Remaining microseconds after converting to milliseconds

        println!(
            "Result: {}\nTime Taken: {} milli secs and {} micro secs\n",
            result, millis, micros
        );
    }

    /// Reads a file and returns its content as a vector of elements of type `T`.
    ///
    /// # Arguments
    /// * `day_num` - The day number of the puzzle.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the input file.
    ///
    /// # Returns
    /// * `Vec<T>` - A vector containing the parsed elements from the file.
    ///
    /// # Panics
    ///  If the file cannot be opened or if parsing an element fails.
    fn read_file<T>(day_num: u8) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let file_path = Self::get_file_path().join("inputs").join(if day_num == 0 {
            "Example".to_string()
        } else {
            format!("day{}", day_num)
        }).with_extension("txt");

        let file = File::open(&file_path).unwrap_or_else(|_| panic!("Failed to open file at {}", file_path.display()));
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| line.unwrap().parse::<T>().unwrap())
            .collect()
    }

    /// Retrieves the base directory for the project.
    ///
    /// # Returns
    /// * `PathBuf` - The path to the project's base directory.
    fn get_file_path() -> PathBuf {
        let mut current_directory = env::current_dir().unwrap();

        if !current_directory.ends_with("src") {
            current_directory.push("src");
        }

        current_directory
    }

    /// Creates a new Rust file for a specific day with a template.
    ///
    /// # Arguments
    /// * `day_num` - The day number for which to create the new file.
    ///
    /// # Panics
    /// If the file already exists or if it cannot be created.
    pub fn new_day(day_num: i32) {
        let src_file_path = Self::get_file_path().join(format!("day{}", day_num)).with_extension("rs");
        if src_file_path.exists() {
            panic!(
                "Cannot create file as it already exists at {}",
                src_file_path.display()
            );
        }
        let input_file_path = Self::get_file_path().join("inputs").join(format!("day{}.txt", day_num));
        if input_file_path.exists() {
            panic!(
                "Cannot create file as it already exists at {}",
                input_file_path.display()
            );
        }
        println!("NEW_DAY.txt: {}", input_file_path.display());
        println!("    src.rs: {}", src_file_path.display());
        let _ = File::create(&input_file_path).unwrap_or_else(|_| panic!("Failed to create file at {}", input_file_path.display()));
        let mut file = File::create(&src_file_path).unwrap_or_else(|_| panic!("Failed to create file at {}", src_file_path.display()));
        writeln!(
            file,
            r#"use helper_utils::Utils;

use crate::utils::helper_utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/20{}/day/{}).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {{
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 0, 0);
    Utils::run_part(part2, 2, 0, 0);
}}

fn part1(input: Vec<String>) -> u64 {{
    println!("Part 1: {{:?}}", input);
    return 0;
}}

fn part2(input: Vec<String>) -> u64 {{
    println!("Part 2 {{:?}}", input);
    return 0;
}}
                "#,
            Utils::AOC_YEAR,
            day_num
        )
            .expect("Failed to write to file");
        println!(
            "File successfully created at location: {} & {}",
            src_file_path.display(), input_file_path.display()
        );
    }

    /// Prints a 2D array.
    ///
    /// # Arguments
    /// * `arr` - A reference to a 2D array to be printed.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the 2D array.
    pub fn print_2d<T: std::fmt::Debug>(arr: &[Vec<T>]) {
        for row in arr {
            for elem in row {
                print!("{:?}\t", elem);
            }
            println!();
        }
    }
}