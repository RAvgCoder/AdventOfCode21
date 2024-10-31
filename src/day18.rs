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
    Utils::run_part(part1, 1, 18, Some(3051));
    Utils::run_part(part2, 2, 18, Some(4812));
}

fn part1(mut input: Vec<SnailFish>) -> u64 {
    let first = input.remove(0);
    input
        .into_iter()
        .fold(first, |mut acc_snail_fish, snail_fish| {
            acc_snail_fish += snail_fish;
            acc_snail_fish
        })
        .magnitude()
}

fn part2(input: Vec<SnailFish>) -> u64 {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::scope(|s| {
        for i in 0..input.len() {
            let tx = tx.clone();
            let input = &input;

            s.spawn(move || {
                let mut max_magnitude = 0;

                for j in 0..input.len() {
                    if i != j {
                        let mut lhs = input[i].clone();
                        lhs += input[j].clone();
                        let mut rhs = input[j].clone();
                        rhs += input[i].clone();

                        max_magnitude = max_magnitude.max(lhs.magnitude().max(rhs.magnitude()));
                    }
                }

                tx.send(max_magnitude).unwrap();
            });
        }
    });

    drop(tx);
    rx.into_iter().max().unwrap()
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

impl Clone for SnailFish {
    fn clone(&self) -> Self {
        Self {
            tokens: self.tokens.clone(),
        }
    }
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
        fn replace_pair(tokens: &mut Vec<SnailToken>, index: usize) -> (SnailToken, SnailToken) {
            assert_eq!(tokens[index], SnailToken::OpenParen);
            *tokens.get_mut(index).unwrap() = SnailToken::Number(0); // replace [ with 0
            let num1 = tokens.remove(index + 1); // Num1
            assert!(matches!(num1, SnailToken::Number(_)));
            let num2 = tokens.remove(index + 1); // Num2
            assert!(matches!(num2, SnailToken::Number(_)));
            assert_eq!(tokens.remove(index + 1), SnailToken::CloseParen); // ]

            (num1, num2)
        }

        loop {
            let mut exploded = false;
            let mut depth = 0_u8;
            let mut index = 0;

            while index < self.tokens.len() {
                match self.tokens[index] {
                    SnailToken::OpenParen => {
                        depth += 1;

                        if depth >= 5 {
                            let (num1, num2) = replace_pair(&mut self.tokens, index);

                            if let Some(SnailToken::Number(n)) = self.tokens[..index]
                                .iter_mut()
                                .rfind(|tok| matches!(tok, SnailToken::Number(_)))
                            {
                                *n += match num1 {
                                    SnailToken::Number(num1) => num1,
                                    _ => {
                                        unreachable!("Should never be anything other than a number")
                                    }
                                }
                            }

                            if let Some(SnailToken::Number(n)) = self.tokens[index + 1..]
                                .iter_mut()
                                .find(|tok| matches!(tok, SnailToken::Number(_)))
                            {
                                *n += match num2 {
                                    SnailToken::Number(num2) => num2,
                                    _ => {
                                        unreachable!("Should never be anything other than a number")
                                    }
                                }
                            }

                            depth -= 1; // We have remove the current pair so we are no longer at that depth
                            exploded = true;
                        }
                    }
                    SnailToken::CloseParen => depth -= 1,
                    SnailToken::Number(_) => (),
                }

                index += 1;
            }

            if !exploded {
                break;
            }
        }
    }

    fn split(&mut self) -> bool {
        let mut index = 0;
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

                    return true;
                }
            }

            index += 1;
        }

        false
    }

    fn merge(&mut self, other: Self) {
        self.tokens.insert(0, SnailToken::OpenParen);
        self.tokens.extend(other.tokens);
        self.tokens.push(SnailToken::CloseParen);
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
        self.merge(rhs);
        loop {
            self.explode();
            if !self.split() {
                break;
            }
        }
    }
}

impl fmt::Debug for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.tokens.iter().peekable();
        let mut i = 0;
        while let Some(tok) = iter.next() {
            match tok {
                SnailToken::OpenParen => {
                    i += 1;
                    write!(f, "[")?
                }
                SnailToken::CloseParen => {
                    i -= 1;
                    match iter.peek() {
                        Some(SnailToken::Number(n)) => {
                            write!(f, "],{}", n)?;
                            iter.next();
                        }
                        Some(SnailToken::OpenParen) => {
                            write!(f, "],[")?;
                            i += 1;
                            iter.next();
                        }
                        _ => write!(f, "]")?,
                    }
                }
                SnailToken::Number(n) => match iter.peek() {
                    Some(SnailToken::Number(n2)) => {
                        write!(f, "{},{}", n, n2)?;
                        iter.next();
                    }
                    Some(SnailToken::OpenParen) => {
                        write!(f, "{},[", n)?;
                        i += 1;
                        iter.next();
                    }
                    None => {
                        debug_assert!(false, "Cannot end list with a number: {:?}", self.tokens);
                        unreachable!("Numbers should never end the list")
                    }
                    _ => write!(f, "{}", n)?,
                },
            }
        }
        assert_eq!(i, 0);
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
        [
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ]
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
            (
                "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
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
                    "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                ),
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                ),
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                    "[7,[5,[[3,8],[1,4]]]]",
                ),
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                    "[[2,[2,2]],[8,[8,1]]]",
                ),
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                    "[2,9]",
                ),
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            ),
            AddTest(
                AddInput(
                    "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                    "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                ),
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                    "[[[5,[7,4]],7],1]",
                ),
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            AddTest(
                AddInput(
                    "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                    "[[[[4,2],2],6],[8,7]]",
                ),
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
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
                a += b;
                assert_eq!(a, expected, "Failed to add SnailFish for test {}", idx);
            });
    }
}
