use day_setup::Utils;
use std::str::FromStr;

use crate::utils::day_setup;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/10).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 10, 318081);
    Utils::run_part(part2, 2, 10, 4361305341);
}

fn part1(program: Vec<NavSubSystem>) -> u64 {
    let mut map_count = [
        (Instruction::CloseAngle, 0),
        (Instruction::CloseCurly, 0),
        (Instruction::CloseParen, 0),
        (Instruction::CloseSquare, 0),
    ];

    for nav_system in program {
        if let (Some(instruction), _) = nav_system.is_corrupted() {
            match instruction {
                Instruction::CloseAngle => map_count[0].1 += 1,
                Instruction::CloseCurly => map_count[1].1 += 1,
                Instruction::CloseParen => map_count[2].1 += 1,
                Instruction::CloseSquare => map_count[3].1 += 1,
                _ => panic!("Invalid error instruction: {:?}", instruction),
            }
        }
    }

    map_count
        .iter()
        .map(|(k, v)| k.get_closing_points() as u64 * *v)
        .sum()
}

fn part2(program: Vec<NavSubSystem>) -> u64 {
    let mut values = program
        .iter()
        .filter_map(|nav_system| {
            if let (None, stack) = nav_system.is_corrupted() {
                const MULTIPLIER: u64 = 5;
                Some(NavSubSystem::fix_corrupted(&stack).into_iter().fold(
                    0u64,
                    |mut acc, instruction| {
                        acc *= MULTIPLIER;
                        acc += match instruction {
                            Instruction::CloseParen => 1,
                            Instruction::CloseSquare => 2,
                            Instruction::CloseCurly => 3,
                            Instruction::CloseAngle => 4,
                            _ => panic!("Invalid instruction: {:?}", instruction),
                        };
                        acc
                    },
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    values.sort();

    values[values.len() / 2]
}

#[derive(Debug)]
struct NavSubSystem {
    instructions: Box<[Instruction]>,
}

impl NavSubSystem {
    fn is_corrupted(&self) -> (Option<Instruction>, Vec<Instruction>) {
        let mut stack = Vec::new();

        for instruction in self.instructions.iter() {
            if instruction.is_open() {
                stack.push(*instruction);
            } else if let Some(open) = stack.pop() {
                if !open.is_closing_for(instruction) {
                    return (Some(*instruction), stack);
                }
            } else {
                panic!("Stack Empty failed to pop instruction: {:?}", instruction);
            }
        }

        (None, stack)
    }

    fn fix_corrupted(instruction_stack: &[Instruction]) -> Vec<Instruction> {
        instruction_stack
            .iter()
            .rev()
            .map(|instruction| instruction.generate_closing())
            .collect()
    }
}

impl FromStr for NavSubSystem {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::with_capacity(line.len());
        for (idx, c) in line.chars().enumerate() {
            instructions.push(Instruction::try_from(c).map_err(|err| {
                format!("Failed to parse instruction: {} at line: {}", err, idx + 1)
            })?);
        }

        Ok(Self {
            instructions: instructions.into_boxed_slice(),
        })
    }
}

/// [ ( { < > } ) ]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    CloseAngle,
    OpenAngle,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
}

impl Instruction {
    fn is_open(&self) -> bool {
        matches!(
            self,
            Instruction::OpenParen
                | Instruction::OpenCurly
                | Instruction::OpenSquare
                | Instruction::OpenAngle
        )
    }

    fn is_closing_for(&self, other: &Self) -> bool {
        match self {
            Instruction::OpenParen => matches!(other, Instruction::CloseParen),
            Instruction::OpenCurly => matches!(other, Instruction::CloseCurly),
            Instruction::OpenSquare => matches!(other, Instruction::CloseSquare),
            Instruction::OpenAngle => matches!(other, Instruction::CloseAngle),
            _ => false,
        }
    }

    fn generate_closing(&self) -> Instruction {
        match self {
            Instruction::OpenParen => Instruction::CloseParen,
            Instruction::OpenCurly => Instruction::CloseCurly,
            Instruction::OpenSquare => Instruction::CloseSquare,
            Instruction::OpenAngle => Instruction::CloseAngle,
            _ => panic!("Invalid instruction: {:?}", self),
        }
    }

    fn get_closing_points(&self) -> u16 {
        match self {
            Instruction::CloseParen => 3,
            Instruction::CloseSquare => 57,
            Instruction::CloseCurly => 1197,
            Instruction::CloseAngle => 25137,
            _ => panic!("Invalid instruction: {:?}", self),
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::CloseAngle),
            '<' => Ok(Self::OpenAngle),
            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::CloseParen),
            '{' => Ok(Self::OpenCurly),
            '}' => Ok(Self::CloseCurly),
            '[' => Ok(Self::OpenSquare),
            ']' => Ok(Self::CloseSquare),
            invalid => Err(format!("Invalid instruction: {}", invalid)),
        }
    }
}
