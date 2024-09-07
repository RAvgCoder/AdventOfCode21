use std::iter::zip;

use day_setup::Utils;

use crate::day5::diagram::Diagram;
use crate::day5::lines::Line;
use crate::utils::day_setup;

pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 5, 4993);
    Utils::run_part(part2, 2, 5, 21101);
}

/// Part 1 solution: considers only horizontal and vertical lines.
///
/// # Arguments
/// * `input` - A vector of strings representing the line segments.
///
/// # Returns
/// * `u64` - The number of points where at least two lines overlap.
fn part1(input: Vec<String>) -> u64 {
    let mut diagram = Diagram::new();
    for line in input {
        diagram.draw_line(Line::new(extract_ranges(line)), |_, _| {});
    }
    diagram.num_of_overlap()
}

/// Part 2 solution: considers horizontal, vertical, and diagonal lines.
///
/// # Arguments
/// * `input` - A vector of strings representing the line segments.
///
/// # Returns
/// * `u64` - The number of points where at least two lines overlap.
fn part2(input: Vec<String>) -> u64 {
    let mut diagram = Diagram::new();
    for line in input {
        diagram.draw_line(
            Line::new(extract_ranges(line)),
            |diagram: &mut Diagram, line: Line| {
                if line.is_diagonal {
                    for (x, y) in zip(line.x_range(), line.y_range()) {
                        diagram.place_at(x, y);
                    }
                }
            },
        );
    }

    diagram.num_of_overlap()
}

/// Extracts the range coordinates from a string representing a line segment.
///
/// # Arguments
/// * `line` - A string representing a line segment in the format "x1,y1 -> x2,y2".
///
/// # Returns
/// * `[usize; 4]` - An array containing the extracted coordinates [x1, y1, x2, y2].
fn extract_ranges(line: String) -> [usize; 4] {
    let ranges: [usize; 4] = line
        .split(" -> ")
        .flat_map(|x_y| x_y.split(',').map(|e| e.parse::<usize>().unwrap()))
        .collect::<Vec<usize>>()
        .try_into()
        .expect("Not enough / Bad input provided for the line");
    ranges
}

mod lines {
    /// Represents a line segment with x and y ranges and flags for orientation.
    #[derive(Debug)]
    pub struct Line {
        x_range: (usize, usize),
        y_range: (usize, usize),
        pub is_perpendicular: bool,
        pub is_diagonal: bool,
    }

    impl Line {
        /// Creates a new `Line` from the given coordinates.
        ///
        /// # Arguments
        /// * `x_y_ranges` - An array containing the coordinates [x1, y1, x2, y2].
        ///
        /// # Returns
        /// * `Line` - The created line segment.
        pub fn new(x_y_ranges: [usize; 4]) -> Line {
            let (x1, x2) = (x_y_ranges[0], x_y_ranges[2]);
            let (y1, y2) = (x_y_ranges[1], x_y_ranges[3]);

            let is_perpendicular = x1 == x2 || y1 == y2;
            let is_diagonal =
                (x2 as isize - x1 as isize).abs() == (y2 as isize - y1 as isize).abs();

            Line {
                x_range: (x1, x2),
                y_range: (y1, y2),
                is_perpendicular,
                is_diagonal,
            }
        }

        /// Returns the x range as a vector, reversed if necessary.
        ///
        /// # Returns
        /// * `Vec<usize>` - The x range as a vector.
        pub fn x_range(&self) -> Vec<usize> {
            let start = self.x_range.0;
            let end = self.x_range.1;
            if start > end {
                (end..=start).rev().collect()
            } else {
                (start..=end).collect()
            }
        }

        /// Returns the y range as a vector, reversed if necessary.
        ///
        /// # Returns
        /// * `Vec<usize>` - The y range as a vector.
        pub fn y_range(&self) -> Vec<usize> {
            let start = self.y_range.0;
            let end = self.y_range.1;
            if start > end {
                (end..=start).rev().collect()
            } else {
                (start..=end).collect()
            }
        }
    }
}

mod diagram {
    use crate::day5::lines::Line;

    const ARRAY_SIZE: usize = 1000;

    /// Represents the diagram where lines are drawn and overlaps are calculated.
    pub struct Diagram {
        num_of_overlap: u32,
        board: Box<[[u16; ARRAY_SIZE]; ARRAY_SIZE]>,
    }

    impl Diagram {
        /// Creates a new, empty `Diagram`.
        ///
        /// # Returns
        /// * `Diagram` - The created diagram.
        #[inline(always)]
        pub fn new() -> Diagram {
            Diagram {
                num_of_overlap: 0,
                board: (0..ARRAY_SIZE)
                    .map(|_| [0u16; ARRAY_SIZE])
                    .collect::<Vec<[u16; ARRAY_SIZE]>>()
                    .try_into()
                    .unwrap(),
            }
        }

        /// Draws a line on the diagram, with optional extra conditions.
        /// DEFAULT CONDITION: If line `is_perpendicular`
        ///
        /// # Arguments
        /// * `line` - The line to be drawn.
        /// * `extra_draw_conditions` - Additional drawing logic to be applied.
        pub fn draw_line<F>(&mut self, line: Line, mut extra_draw_conditions: F)
        where
            F: FnMut(&mut Diagram, Line),
        {
            if line.is_perpendicular {
                for x in line.x_range() {
                    for y in line.y_range() {
                        self.place_at(x, y);
                    }
                }
            }
            extra_draw_conditions(self, line);
        }

        /// Places an element at the specified (x, y) position on the board.
        ///
        /// This function increments the value at the given position by 1.
        /// If the new value at this position is 2, it increments the `num_of_overlap` counter.
        ///
        /// # Parameters
        ///
        /// - `x`: The x-coordinate (column index) of the position.
        /// - `y`: The y-coordinate (row index) of the position.
        ///
        /// # Panics
        ///
        /// This function will panic if `x` or `y` is out of bounds of the board.
        #[inline(always)]
        pub fn place_at(&mut self, x: usize, y: usize) {
            self.board[y][x] += 1;
            if self.board[y][x] == 2 {
                self.num_of_overlap += 1;
            }
        }

        /// Calculates the number of points where at least two lines overlap.
        ///
        /// # Returns
        /// * `u64` - The number of overlapping points.
        pub fn num_of_overlap(&self) -> u64 {
            self.num_of_overlap as u64
        }
    }
}
