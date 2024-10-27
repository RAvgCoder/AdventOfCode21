use crate::utils::day_setup::Utils;
use std::fmt;
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::slice::Iter;
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

    let mut input1 = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,5]],[[5,[[5,5],0]],[5,6]]]]"
        .parse::<SnailFish>().unwrap();
    input1.explode();
    
    // let first = input.remove(0);
    // input
    //     .into_iter()
    //     .enumerate()
    //     .fold(first, |mut acc_snail_fish, (idx, snail_fish)| {
    //         acc_snail_fish += snail_fish;
    //         println!("\n\nCalc {idx}  | Res: {:?}\n\n", acc_snail_fish);
    //         acc_snail_fish
    //     })
    //     .magnitude();

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
    fn magnitude(&self) -> u64 {
        Self::magnitude_helper(&mut self.tokens.iter())
    }

    fn magnitude_helper(snail_token: &mut Iter<SnailToken>) -> u64 {
        if let Some(token) = snail_token.next() {
            match token {
                SnailToken::OpenParen => {
                    let a = Self::magnitude_helper(snail_token);
                    let b = Self::magnitude_helper(snail_token);
                    let _ = snail_token.next(); // Consume the CloseParen token
                    return (3 * a) + (2 * b);
                }
                SnailToken::CloseParen => unreachable!("Close Paren should never be the first token as you should always leave at a number"),
                SnailToken::Number(n) => {
                    return *n as u64;
                }
            }
        }

        unreachable!("Should never reach here as lists should always be in pairs")
    }

    fn explode(&mut self) {
        println!("Exploding: {:?}", self);
        fn remove_snail_fish(tokens: &mut Vec<SnailToken>, i: usize) {
            println!("{:?}", tokens[i - 1..i + 3].to_vec());
            tokens.remove(i); // Num1
            tokens.remove(i); // Num2
            tokens.remove(i); // ]
            *tokens.get_mut(i - 1).unwrap() = SnailToken::Number(0); // replace [ with 0
        }

        let mut depth = 0_u8;
        let mut index = 0;
        let mut exploded = false;
        while index < self.tokens.len() {
            match self.tokens[index] {
                SnailToken::OpenParen => depth += 1,
                SnailToken::CloseParen => depth -= 1,
                SnailToken::Number(first_pair) if depth >= 5 => {
                    // Find the last number token before the current number token
                    let _ = self.tokens[..index]
                        .iter_mut()
                        .rfind(|tok| matches!(tok, SnailToken::Number(_)))
                        .map(|num_tok| match num_tok {
                            SnailToken::Number(num) => *num += first_pair,
                            _ => unreachable!("Filtered value should only be a number token"),
                        });

                    println!();
                    println!("{:?}", self.tokens[index - 2]);
                    println!("{:?}", self.tokens[index - 1]);
                    println!("{:?}", self.tokens[index]);
                    println!("{:?}", self.tokens[index + 1]);
  
                    let second_pair = match self.tokens[index + 1] {
                        SnailToken::Number(num) => num,
                        SnailToken::CloseParen => {
                            println!(
                                "\n \
                            Idx: {}\n \
                            CurrNum {:?}\n \
                            First Pair: {:?}\n \
                            Second Pair: {:?}",
                                index,
                                first_pair,
                                &self.tokens[..index],
                                &self.tokens[index..]
                            );
                            debug_assert!(
                                false,
                                "Numbers should always come in pairs Saw=SnailToken::CloseParen\n List:{:?}",
                                self
                            );
                            unreachable!("Numbers should always come in pairs")
                        }
                        SnailToken::OpenParen => {
                            panic!("Inner values are found")
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
                    println!("+{:?}+", self.tokens[index-1]);

                    // We know that we are now past the removed tokens so we decrement the index
                    index -= 2;
                    depth -= 1;
                    exploded = true;

                    println!("Exploded into: {:?}", self);
                }
                // SnailToken::Number(n) if depth > 5 => {
                //     debug_assert!(
                //         false,
                //         "Cannot have a number outside of a depth of 5 or any depth other than 5\n\
                //         Saw={} IDX={}\n List:{:?}",
                //         n, index, self
                //     );
                //     unreachable!(
                //         "Cannot have a number outside of a depth of 5 or any depth other than 5"
                //     )
                // }
                SnailToken::Number(_) => {
                    /* Do nothing for numbers that are not above critical depth */
                }
            }

            index += 1;
        }

        if exploded && self.split() {
            self.explode();
        }
    }

    pub fn split(&mut self) -> bool {
        let mut index = 0;
        let mut was_split = false;
        while index < self.tokens.len() {
            if let SnailToken::Number(n) = self.tokens[index] {
                if n > 9 {
                    // Split 2-digit numbers into two single digit numbers
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
            }

            index += 1;
        }

        println!("Split into: {:?}", self);
        was_split
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
    type Err = ParseIntError;

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
                    tokens.push(SnailToken::Number(buff.parse::<u8>()?));
                }
                _comma => (),
            }
        }
        Ok(SnailFish { tokens })
    }
}

#[cfg(test)]
mod snail_fish_tests {
    use super::*;

    #[test]
    fn test_snail_fish_from_str() {
        let input = "[9,[8,7]]".parse::<SnailFish>().unwrap();
        let expected = SnailFish {
            tokens: vec![
                SnailToken::OpenParen,
                SnailToken::Number(9),
                SnailToken::OpenParen,
                SnailToken::Number(8),
                SnailToken::Number(7),
                SnailToken::CloseParen,
                SnailToken::CloseParen,
            ],
        };
        assert_eq!(input, expected, "Failed to parse SnailFish");

        let input = "[9,[8,7],[6,5]]".parse::<SnailFish>().unwrap();
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
                SnailToken::CloseParen,
            ],
        };

        assert_eq!(input, expected, "Failed to parse SnailFish");
    }

    #[test]
    fn test_snail_fish_split() {
        [(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        )]
        .map(|(i, e)| {
            (
                i.parse::<SnailFish>().unwrap(),
                e.parse::<SnailFish>().unwrap(),
            )
        })
        .into_iter()
        .for_each(|(mut input, expected)| {
            input.split();
            assert_eq!(input, expected, "Failed to split SnailFish");
        })
    }

    #[test]
    fn test_snail_fish_split_multiple() {
        let mut input = "[20,1]".parse::<SnailFish>().unwrap();

        let expected = "[[[5,5],[5,5]],1]".parse::<SnailFish>().unwrap();

        input.split();
        assert_eq!(input, expected, "Failed to split SnailFish");
    }

    #[test]
    fn test_snail_fish_explode() {
        [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ]
        .map(|(input, expected)| {
            (
                input.parse::<SnailFish>().unwrap(),
                expected.parse::<SnailFish>().unwrap(),
            )
        })
        .into_iter()
        .for_each(|(mut input, expected)| {
            input.explode();
            assert_eq!(input, expected, "Failed to explode SnailFish");
        });
    }

    #[test]
    fn test_explode_split_combo() {
        [(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        )]
        .map(|(i, e)| {
            (
                i.parse::<SnailFish>().unwrap(),
                e.parse::<SnailFish>().unwrap(),
            )
        })
        .into_iter()
        .for_each(|(mut input, expected)| {
            input.explode();
            assert_eq!(input, expected, "Failed to explode SnailFish");
        });
    }

    #[test]
    fn test_magnitude() {
        [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
            (
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
                4140,
            ),
        ]
        .map(|(input, expected)| (input.parse::<SnailFish>().unwrap(), expected))
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(input.magnitude(), expected, "Failed to calculate magnitude");
        });
    }
    
    #[test]
    fn test() {
        let mut input = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,5]],[[10,[0,5]],[5,6]]]]"
            .parse::<SnailFish>().unwrap();
        input.explode();
        println!("1:{:?}", input);
        println!("{}","~".repeat(50));
        let mut input = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,5]],[[5,[[5,5],0]],[5,6]]]]"
            .parse::<SnailFish>().unwrap();
        input.explode();
        println!("2:{:?}", input);
        // assert!(false);
    }

    #[test]
    fn test_addition() {
        struct AddTest<'input>(AddInput<'input>, &'input str);
        struct AddInput<'input>(&'input str, &'input str);
        [
            AddTest(
                AddInput("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"),
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
            AddTest(
                AddInput(
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                ),
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]",
                    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                ),
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]",
                    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                ),
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]",
                    "[7,[5,[[3,8],[1,4]]]]",
                ),
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]",
                    "[[2,[2,2]],[8,[8,1]]]",
                ),
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]",
                    "[2,9]",
                ),
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]",
            ),
            AddTest(
                AddInput(
                    "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]",
                    "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                ),
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]",
                    "[[[5,[7,4]],7],1]",
                ),
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]",
                    "[[[[4,2],2],6],[8,7]]",
                ),
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]",
            ),
        ]
        .map(|AddTest(AddInput(a, b), e)| {
            (
                (
                    a.parse::<SnailFish>().unwrap(),
                    b.parse::<SnailFish>().unwrap(),
                ),
                e.parse::<SnailFish>().unwrap(),
            )
        })
        .into_iter()
        .enumerate()
        .for_each(|(idx, ((mut a, b), expected))| {
            println!("Test: {}\n\n", idx);
            a += b;
            assert_eq!(a, expected, "Failed to add SnailFish for test {}", idx);
        });
    }
}
