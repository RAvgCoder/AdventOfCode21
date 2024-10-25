use crate::utils::day_setup::Utils;
use std::fmt;
use std::ops::AddAssign;
use std::str::FromStr;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/18).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 0, None);
    Utils::run_part(part2, 2, 0, None);
}

fn part1(mut input: Vec<SnailFish>) -> u64 {
    let first = input.remove(0);
    input
        .into_iter()
        .enumerate()
        .fold(first, |mut acc, (i, e)| {
            acc += e;
            println!("\n\nCalc {i}  | Res: {:?}\n\n", acc);
            acc
        });

    0
}

fn part2(input: Vec<SnailFish>) -> u64 {
    input.iter().for_each(|e| println!("{:?}\n", e));
    0
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SnailToken {
    OpenParen,
    CloseParen,
    Number(u8),
}

#[derive(Eq)]
struct SnailFish {
    tokens: Vec<SnailToken>,
}

impl SnailFish {
    pub fn explode(&mut self) {
        fn remove_snail_fish(tokens: &mut Vec<SnailToken>, i: usize) {
            println!("{:?}", tokens[i - 1..i + 3].to_vec());
            tokens.remove(i); // Num1
            tokens.remove(i); // Num2
            tokens.remove(i); // ]
            *tokens.get_mut(i - 1).unwrap() = SnailToken::Number(0); // replace [ with 0
        }

        let mut depth = 0_u8;
        let mut index = 0;
        loop {
            if index >= self.tokens.len() {
                break;
            }

            match self.tokens[index] {
                SnailToken::OpenParen => depth += 1,
                SnailToken::CloseParen => depth -= 1,
                SnailToken::Number(first_pair) if depth == 5 => {
                    // Find the last number token before the current number token
                    let _ = self.tokens[..index]
                        .iter_mut()
                        .rev()
                        .find(|tok| matches!(tok, SnailToken::Number(_)))
                        .map(|num_tok| match num_tok {
                            SnailToken::Number(num) => *num += first_pair,
                            _ => unreachable!("Filtered value should only be a number token"),
                        });

                    let second_pair = match self.tokens[index + 1] {
                        SnailToken::Number(num) => num,
                        e => {
                            debug_assert!(
                                false,
                                "Numbers should always come in pairs Saw={:?}\n List:{:?}",
                                e, self.tokens
                            );
                            unreachable!("Numbers should always come in pairs")
                        }
                    };

                    // Find the next number token after the current number token
                    let _ = self.tokens[index + 2..]
                        .iter_mut()
                        .find(|tok| matches!(tok, SnailToken::Number(_)))
                        .map(|num_tok| match num_tok {
                            SnailToken::Number(num) => *num += second_pair,
                            _ => unreachable!("Filtered value should only be a number token"),
                        });

                    remove_snail_fish(&mut self.tokens, index);

                    // We know that we are now past the removed tokens so we decrement the index
                    index -= 1;
                    depth -= 1;

                    println!("Exploded into: {:?}", self);
                }
                SnailToken::Number(n) if depth > 5 => {
                    debug_assert!(
                        false,
                        "Cannot have a number outside of a depth of 5 or any depth other than 5\n\
                        Saw={n} IDX={index}\n List:{:?}",
                        self
                    );
                    unreachable!(
                        "Cannot have a number outside of a depth of 5 or any depth other than 5"
                    )
                }
                SnailToken::Number(_) => (),
            }

            index += 1;
        }

        self.split();
    }

    pub fn split(&mut self) {
        let mut index = 0;
        let mut was_split = false;
        loop {
            if index >= self.tokens.len() {
                break;
            }

            match self.tokens[index] {
                SnailToken::Number(n) if n > 9 => {
                    let (first, second) = (n / 2, n - (n / 2));

                    // Remove the number token currently there
                    self.tokens.remove(index);

                    // Insert the new number pair in its spot
                    self.tokens.insert(index, SnailToken::CloseParen);
                    self.tokens.insert(index, SnailToken::Number(second));
                    self.tokens.insert(index, SnailToken::Number(first));
                    self.tokens.insert(index, SnailToken::OpenParen);

                    was_split = true;
                }
                _ => {}
            }

            index += 1;
        }

        println!("Split into: {:?}", self);

        if was_split {
            self.explode();
        }
    }
}

impl PartialEq for SnailFish {
    fn eq(&self, other: &Self) -> bool {
        if self.tokens.len() == other.tokens.len() {
            for (a, b) in self.tokens.iter().zip(other.tokens.iter()) {
                if a != b {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl AddAssign for SnailFish {
    fn add_assign(&mut self, rhs: Self) {
        self.tokens.insert(0, SnailToken::OpenParen);
        self.tokens.extend(rhs.tokens);
        self.tokens.push(SnailToken::CloseParen);
        self.explode();
    }
}

impl fmt::Debug for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.tokens.iter().peekable();
        while let Some(tok) = iter.next() {
            match tok {
                SnailToken::OpenParen => write!(f, "[")?,
                SnailToken::CloseParen => match iter.peek() {
                    Some(SnailToken::Number(n)) => {
                        write!(f, "],{}", n)?;
                        iter.next();
                    }
                    Some(SnailToken::OpenParen) => {
                        write!(f, "],[")?;
                        iter.next();
                    }
                    _ => write!(f, "]")?,
                },
                SnailToken::Number(n) => {
                    match iter.peek() {
                        Some(SnailToken::Number(n2)) => {
                            write!(f, "{},{}", n, n2)?;
                            iter.next();
                        }
                        Some(SnailToken::OpenParen) => {
                            write!(f, "{},[", n)?;
                            iter.next();
                        }
                        None => {
                            debug_assert!(
                                false,
                                "Cannot end list with a number: {:?}",
                                self.tokens
                            );
                            unreachable!("Numbers should never end the list")
                        }
                        _ => write!(f, "{}", n)?,
                    };
                }
            }
        }
        Ok(())
    }
}

impl FromStr for SnailFish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(curr) = chars.next() {
            match curr {
                '[' => tokens.push(SnailToken::OpenParen),
                ']' => tokens.push(SnailToken::CloseParen),
                '0'..='9' => {
                    let mut buff = String::from(curr);
                    while let Some(&e) = chars.peek() {
                        match e {
                            '0'..='9' => {
                                buff.push(e);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    tokens.push(SnailToken::Number(buff.parse::<u8>().unwrap()));
                }
                _comma => (),
            }
        }
        Ok(SnailFish { tokens })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snail_fish_from_str() {
        let input = "[9,[8,7]]";
        let expected = SnailFish {
            tokens: vec![
                SnailToken::OpenParen,
                SnailToken::Number(9),
                SnailToken::OpenParen,
                SnailToken::Number(8),
                SnailToken::Number(7),
                SnailToken::CloseParen,
                SnailToken::CloseParen
            ],
        };
        assert_eq!(input.parse::<SnailFish>().unwrap(), expected, "Failed to parse SnailFish");


        let input = "[9,[8,7],[6,5]]";
        let expected = SnailFish {
            tokens: vec![
                SnailToken::OpenParen,
                SnailToken::Number(9),
                SnailToken::OpenParen,
                SnailToken::Number(8),
                SnailToken::Number(7),
                SnailToken::CloseParen,
                SnailToken::OpenParen,
                SnailToken::Number(6),
                SnailToken::Number(5),
                SnailToken::CloseParen,
                SnailToken::CloseParen
            ],
        };
        assert_eq!(input.parse::<SnailFish>().unwrap(), expected, "Failed to parse SnailFish");
    }

    #[test]
    fn test_snail_fish_split() {
        let mut input = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse::<SnailFish>().unwrap();
        // [7,8] give me the enum

        let expected = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".parse::<SnailFish>().unwrap();
        
        input.split();
        assert_eq!(input, expected, "Failed to split SnailFish");
    }
}
