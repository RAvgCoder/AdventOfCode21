use crate::utils::coordinate_system::direction::FullDirection;
use crate::utils::coordinate_system::Coordinate;
use crate::utils::day_setup::Utils;
use std::collections::HashSet;
use std::fmt::Debug;
use std::mem;
use std::ops::RangeInclusive;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/20).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    // 5479 too Low
    // 5525 X
    // 5539 X
    // 5971 too high
    Utils::run_part_single(part1, 1, 0, None);
    Utils::run_part_single(part2, 2, 0, None);
}

fn part1(mut image_enhancer: ImageEnhancer) -> usize {
    println!("{:#?}", image_enhancer.image);
    image_enhancer.enhance::<2>();
    image_enhancer.image.pixel_count()
}

fn part2(mut image_enhancer: ImageEnhancer) -> usize {
    image_enhancer.enhance::<0>();
    image_enhancer.image.pixel_count()
}

type Pixel = Option<()>;
struct ImageEnhancer {
    enhancement_algorithm: [Pixel; 512],
    image: Image,
}

impl ImageEnhancer {
    /// Enhances the image `N` times.
    fn enhance<const N: usize>(&mut self) {
        for _ in 0..N {
            self.enhance_once();
            println!("{:#?}", self.image);
        }
    }

    fn decode_number(&self, coordinate: &Coordinate) -> Pixel {
        const DIRECTION: [FullDirection; 9] = [
            FullDirection::NorthWest,
            FullDirection::North,
            FullDirection::NorthEast,
            FullDirection::West,
            FullDirection::Current,
            FullDirection::East,
            FullDirection::SouthWest,
            FullDirection::South,
            FullDirection::SouthEast,
        ];

        let enhancement_number = DIRECTION
            .map(|d| {
                let coord = *coordinate + d;
                self.image.get_pixel(&coord)
            })
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, pixel)| {
                acc | match pixel {
                    Some(_) => 1 << i,
                    None => 0,
                }
            });
        self.enhancement_algorithm[enhancement_number]
    }

    fn enhance_once(&mut self) {
        let (row_range, column_range) = self.image.loop_range();

        for i in row_range {
            for j in column_range.clone() {
                let curr_coord = Coordinate::new(i, j);
                let pixel = self.decode_number(&curr_coord);
                if pixel.is_some() {
                    self.image.write(&curr_coord);
                }
            }
        }

        self.image.flush();
    }
}

struct Image {
    width_range: RangeInclusive<i32>,
    height_range: RangeInclusive<i32>,
    front_buffer: HashSet<Coordinate>,
    back_buffer: HashSet<Coordinate>,

    // (AllOff, AllOn)
    infinity: (Pixel, Pixel),
}

type RowRange = RangeInclusive<i32>;

type ColumnRange = RangeInclusive<i32>;
impl Image {
    fn pixel_count(&self) -> usize {
        self.front_buffer.len()
    }
    fn loop_range(&self) -> (RowRange, ColumnRange) {
        const OFFSET: i32 = 1;
        (
            *self.height_range.start() - OFFSET..=*self.height_range.end() + OFFSET,
            *self.width_range.start() - OFFSET..=*self.width_range.end() + OFFSET,
        )
    }

    pub fn write(&mut self, coord: &Coordinate) {
        self.back_buffer.insert(*coord);
    }

    fn flush(&mut self) {
        // Swap the pixels storage
        mem::swap(&mut self.front_buffer, &mut self.back_buffer);
        self.back_buffer.clear();

        if self.infinity.0.is_some() {
            // Swap the infinity pixels
            mem::swap(&mut self.infinity.0, &mut self.infinity.1);
        }

        // Redefine the range
        let mut min_width = i32::MAX;
        let mut max_width = 0;
        let mut min_height = i32::MAX;
        let mut max_height = 0;

        for pixels in &self.front_buffer {
            min_width = min_width.min(pixels.j);
            max_width = max_width.max(pixels.j);

            min_height = min_height.min(pixels.i);
            max_height = max_height.max(pixels.i);
        }

        self.width_range = min_width..=max_width;
        self.height_range = min_height..=max_height;
    }

    fn at_infinity(&self, coordinate: &Coordinate) -> bool {
        let width = self.width_range.clone();
        let height = self.height_range.clone();
        !(width.contains(&coordinate.j) && height.contains(&coordinate.i))
    }

    /// Gets the pixel at the given coordinate.
    fn get_pixel(&self, coord: &Coordinate) -> Pixel {
        if self.at_infinity(coord) {
            if self.infinity.0.is_some() {
                Some(())
            } else {
                None
            }
        } else {
            self.front_buffer.get(coord).map(|_| ())
        }
    }
}

impl Debug for ImageEnhancer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enhancement Algorithm: [ ")?;
        for (i, e) in self.enhancement_algorithm.iter().enumerate() {
            match e {
                Some(_) => write!(f, "{i}, ")?,
                None => (),
            }
        }
        writeln!(f, "]")?;
        writeln!(f, "Image: {:#?}", self.image)
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Pixels: ({}, {})",
            self.infinity.0.is_some(),
            self.infinity.1.is_some()
        )?;
        writeln!(f, "Width Range: {:#?}", self.width_range)?;
        writeln!(f, "Height Range: {:#?}", self.height_range)?;
        writeln!(f, "Pixel Count: {}", self.pixel_count())?;
        // for x in self.height_range.clone() {
        //     for y in self.width_range.clone() {
        //         if self.get_pixel(&Coordinate::new(x, y)).is_some() {
        //             write!(f, "# ")?;
        //         } else {
        //             write!(f, ". ")?;
        //         }
        //     }
        //     writeln!(f)?;
        // }
        Ok(())
    }
}

impl From<Vec<String>> for ImageEnhancer {
    fn from(value: Vec<String>) -> Self {
        let mut input = value.iter();

        let enhancement_algorithm: [Pixel; 512] = input
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => None,
                '#' => Some(()),
                _ => panic!("Invalid character in enhancement algorithm"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let _ = input.next(); // Skip empty line

        let mut max_width = 0;
        let mut max_height = 0;
        let mut pixels = HashSet::new();
        for (i, line) in input.enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    max_width = max_width.max(j as i32);
                    max_height = max_height.max(i as i32);
                    pixels.insert(Coordinate::new(i as i32, j as i32));
                }
            }
        }

        assert!(!pixels.is_empty(), "Image must contain at least one pixel");

        Self {
            image: Image {
                front_buffer: pixels,
                width_range: 0..=max_width,
                height_range: 0..=max_height,
                back_buffer: HashSet::new(),
                infinity: (
                    *enhancement_algorithm.first().unwrap(),
                    *enhancement_algorithm.last().unwrap(),
                ),
            },
            enhancement_algorithm,
        }
    }
}
