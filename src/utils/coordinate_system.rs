use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Default, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Coordinate {
    pub i: i32,
    pub j: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { i: x, j: y }
    }

    #[allow(dead_code)]
    pub const fn manhattan_distance(&self) -> i32 {
        self.i.abs() + self.j.abs()
    }

    pub const fn transpose(&self) -> Self {
        Self::new(self.j, self.i)
    }
}

// Implementing the AddAssign trait for += operator
impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        self.i += other.i;
        self.j += other.j;
    }
}

// Implementing the Add trait for + operator with Coordinate
impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

// Implementing the Add trait for + operator with Direction
impl Add<direction::Direction> for Coordinate {
    type Output = Self;

    fn add(self, direction: direction::Direction) -> Self::Output {
        let (dx, dy) = direction.offset();
        Self {
            i: self.i + dx,
            j: self.j + dy,
        }
    }
}

// Implementing the Add trait for + operator with Direction
impl Add<direction::FullDirection> for Coordinate {
    type Output = Self;

    fn add(self, direction: direction::FullDirection) -> Self::Output {
        let (dx, dy) = direction.offset();
        Self {
            i: self.i + dx,
            j: self.j + dy,
        }
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate({}, {})", self.i, self.j)
    }
}

/// Implements the `FromStr` trait for the `Coordinate` struct, allowing it to be created from a string representation.
///
/// # Type
/// * `Err` - The error type returned if parsing fails. In this case, it is a `String`.
///
/// # Arguments
/// * `line` - A string slice that holds the string representation of the coordinate in the format "x,y".
///
/// # Returns
/// * `Result<Self, Self::Err>` - Returns `Ok(Self)` if parsing is successful, otherwise returns an `Err` with a descriptive error message.
///
/// # Errors
/// This function will return an error if:
/// * The input string does not contain a comma.
/// * The x or y values cannot be parsed as integers.
/// * The input string is not in the format "x,y".
impl FromStr for Coordinate {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line.split_once(',') {
            None => Err(format!("Invalid coordinate {}. Format is 'x,y'", line)),
            Some((i, j)) => {
                let x = i.parse().map_err(|err: std::num::ParseIntError| {
                    format!("Cannot parse i axis: {}", err)
                })?;
                let y = j.parse().map_err(|err: std::num::ParseIntError| {
                    format!("Cannot parse j axis: {}", err)
                })?;
                Ok(Self::new(x, y))
            }
        }
    }
}

pub mod direction {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North,
        East,
        South,
        West,
        Current,
    }

    impl Direction {
        pub const fn offset(&self) -> (i32, i32) {
            match self {
                Self::North => (-1, 0),
                Self::East => (0, 1),
                Self::South => (1, 0),
                Self::West => (0, -1),
                Self::Current => (0, 0),
            }
        }

        /// Returns an array containing the four cardinal directions.
        ///
        /// # Returns
        /// An array of `Direction` enums representing the four cardinal directions:
        /// North, East, South, and West.
        ///
        /// # Example
        /// ```
        /// use crate::direction::Direction;
        /// let directions = Direction::direction_list();
        /// assert_eq!(directions, [Direction::North, Direction::East, Direction::South, Direction::West]);
        /// ```
        pub const fn direction_list() -> [Direction; 4] {
            [Self::North, Self::East, Self::South, Self::West]
        }
    }

    impl TryFrom<char> for Direction {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'N' => Ok(Self::North),
                'E' => Ok(Self::East),
                'S' => Ok(Self::South),
                'W' => Ok(Self::West),
                _ => Err("Invalid direction"),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FullDirection {
        North,
        NorthEast,
        East,
        SouthEast,
        South,
        SouthWest,
        West,
        NorthWest,
        Current,
    }

    impl FullDirection {
        #[allow(dead_code)]
        pub const fn offset(&self) -> (i32, i32) {
            match self {
                Self::North => Direction::North.offset(),
                Self::NorthEast => (-1, 1),
                Self::East => Direction::East.offset(),
                Self::SouthEast => (1, 1),
                Self::South => Direction::South.offset(),
                Self::SouthWest => (1, -1),
                Self::West => Direction::West.offset(),
                Self::NorthWest => (-1, -1),
                Self::Current => Direction::Current.offset(),
            }
        }

        /// Returns an array containing the eight full cardinal and intercardinal directions.
        ///
        /// # Returns
        /// An array of `FullDirection` enums representing the eight full directions:
        /// North, NorthEast, East, SouthEast, South, SouthWest, West, and NorthWest.
        ///
        /// # Example
        /// ```
        /// use crate::direction::FullDirection;
        /// let directions = FullDirection::full_direction_list();
        /// assert_eq!(directions, [
        ///     FullDirection::North,
        ///     FullDirection::NorthEast,
        ///     FullDirection::East,
        ///     FullDirection::SouthEast,
        ///     FullDirection::South,
        ///     FullDirection::SouthWest,
        ///     FullDirection::West,
        ///     FullDirection::NorthWest,
        /// ]);
        /// ```
        #[allow(dead_code)]
        pub const fn full_direction_list() -> [FullDirection; 8] {
            [
                Self::North,
                Self::NorthEast,
                Self::East,
                Self::SouthEast,
                Self::South,
                Self::SouthWest,
                Self::West,
                Self::NorthWest,
            ]
        }
    }

    impl TryFrom<&str> for FullDirection {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "N" => Ok(Self::North),
                "NE" => Ok(Self::NorthEast),
                "E" => Ok(Self::East),
                "SE" => Ok(Self::SouthEast),
                "S" => Ok(Self::South),
                "SW" => Ok(Self::SouthWest),
                "W" => Ok(Self::West),
                "NW" => Ok(Self::NorthWest),
                _ => Err("Invalid direction"),
            }
        }
    }
}
