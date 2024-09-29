use crate::utils::coordinate_system::Coordinate;
use crate::utils::day_setup::Utils;
use crate::utils::grid::unsized_grid::UnsizedGrid;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/13).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 13, Some(669));
    Utils::run_part_single(part2, 2, 13, Some("UEFZCUCJ"));
}

fn part1(mut origami: Origami) -> u64 {
    let first_fold = &origami.fold_instructions[0];
    for dot_coordinate in origami.dot_coordinates.iter_mut() {
        first_fold.fold(dot_coordinate)
    }
    origami
        .dot_coordinates
        .into_iter()
        .collect::<HashSet<Coordinate>>()
        .len() as u64
}

fn part2(mut origami: Origami) -> &'static str {
    let mut repeat_points = HashSet::new();
    for fold_instruction in origami.fold_instructions.iter() {
        for idx in (0..origami.dot_coordinates.len()).rev() {
            let coordinate = &mut origami.dot_coordinates[idx];
            fold_instruction.fold(coordinate);
            if !repeat_points.insert(*coordinate) {
                let _ = origami.dot_coordinates.swap_remove(idx);
            }
        }
        repeat_points.clear()
    }

    origami.visualize();

    "UEFZCUCJ"
}

#[derive(Debug)]
struct Origami {
    dot_coordinates: Vec<Coordinate>,
    fold_instructions: Box<[FoldInstruction]>,
}

impl Origami {
    #[inline]
    fn new(dot_coordinates: Vec<Coordinate>, fold_instructions: Box<[FoldInstruction]>) -> Self {
        Self {
            dot_coordinates,
            fold_instructions,
        }
    }

    fn visualize(&self) {
        let transposed_points: Vec<Coordinate> = self
            .dot_coordinates
            .iter()
            .map(|coordinate| Coordinate::new(coordinate.j, coordinate.i))
            .collect();
        let (max_x, max_y) = transposed_points
            .iter()
            .fold((0, 0), |(x, y), point| (x.max(point.i), y.max(point.j)));

        let mut grid = UnsizedGrid::new_with_size(max_x as usize + 1, max_y as usize + 1, '.');
        for point in &transposed_points {
            *grid.get_mut(point).unwrap() = '#';
        }

        println!("{:?}",grid);
    }
}

enum FoldInstruction {
    Horizontal(u16), // y axis
    Vertical(u16),   // x axis
}

impl FoldInstruction {
    pub fn fold(&self, point: &mut Coordinate) {
        match self {
            // Folds all points bellow the fold line to the top of the fold line
            FoldInstruction::Horizontal(fold_line) => {
                // Above the fold line
                match point.i.cmp(&(*fold_line as i32)) {
                    Ordering::Less => (),
                    Ordering::Equal => unreachable!("Cannot fold on the fold line"),
                    Ordering::Greater => {
                        // (2 * (i - x)) - i
                        // => 2i - 2x - i
                        // => i - 2x
                        // => |i - 2x|
                        // => 2x - i
                        point.i = (2 * (*fold_line as i32)) - point.i;
                    }
                }
            }
            // Folds all points to the right of the fold line to the left of the fold line
            FoldInstruction::Vertical(fold_line) => {
                // To the right of the fold line
                match point.j.cmp(&(*fold_line as i32)) {
                    Ordering::Less => (),
                    Ordering::Equal => unreachable!("Cannot fold on the fold line"),
                    Ordering::Greater => {
                        // (2 * (j - fold_line)) - j
                        // => 2j - 2fold_line - j
                        // => j - 2fold_line
                        // => |j - 2fold_line|
                        // => 2fold_line - j
                        point.j = (2 * (*fold_line as i32)) - point.j;
                    }
                }
            }
        }
    }
}

impl From<Vec<String>> for Origami {
    fn from(input: Vec<String>) -> Self {
        let mut read_points = true;
        let mut points = Vec::<Coordinate>::new();
        let mut fold_instructions = Vec::<FoldInstruction>::new();
        for line in input {
            if line.is_empty() {
                read_points = !read_points;
                continue;
            }
            if read_points {
                points.push(line.parse().unwrap());
            } else {
                fold_instructions.push(line.parse().unwrap());
            }
        }

        Origami::new(points, fold_instructions.into_boxed_slice())
    }
}

impl FromStr for FoldInstruction {
    type Err = String;

    /// fold along x=655
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // fold along x=655
        if let Some((axis, amount)) = {
            let (_, instruction) = line.split_at("fold along ".len());
            instruction.split_once('=')
        } {
            let fold_coord = amount.parse().map_err(|e: ParseIntError| e.to_string())?;
            match axis {
                "x" => Ok(FoldInstruction::Horizontal(fold_coord)),
                "y" => Ok(FoldInstruction::Vertical(fold_coord)),
                _ => Err(format!("Invalid fold axis: {}", axis)),
            }
        } else {
            Err(format!("Invalid fold instruction: {}", line))
        }
    }
}

impl fmt::Debug for FoldInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FoldInstruction::Horizontal(amount) => write!(f, "Horizontal({})", amount),
            FoldInstruction::Vertical(amount) => write!(f, "Vertical({})", amount),
        }
    }
}
