use crate::utils::day_setup::Utils;
use std::ops::RangeInclusive;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/16).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 16, Some(977));
    Utils::run_part(part2, 2, 16, Some(101501020883));
}

struct PacketResult<'rest> {
    version_number: u64,
    rest: &'rest str,
}

fn part1(input: Vec<String>) -> u64 {
    let binary_strings = hex_to_binary_strings(input.first().unwrap());
    let mut packet = Packet {
        bits: &binary_strings,
        evaluated_expression: None,
    };

    packet.decode_version_number().version_number
}

fn part2(input: Vec<String>) -> u64 {
    let binary_strings = hex_to_binary_strings(input.first().unwrap());
    let mut packet = Packet {
        bits: &binary_strings,
        evaluated_expression: None,
    };

    let _ = packet.decode_version_number();

    packet.evaluated_expression.unwrap()
}

#[derive(Debug)]
struct Packet<'a> {
    /// The bits representing the packet.
    bits: &'a str,
    /// The evaluated expression value of the packet, if any.
    evaluated_expression: Option<u64>,
}

impl<'a> Packet<'a> {
    /// The range of bits representing the version number.
    const VERSION_NUMBER: RangeInclusive<usize> = 0..=2;
    /// The range of bits representing the type ID.
    const TYPE_ID_RANGE: RangeInclusive<usize> = 3..=5;

    /// Decodes the version number from the packet's bits.
    ///
    /// The version number is located in the first three bits of the packet.
    ///
    /// # Returns
    /// A `PacketResult` containing the version number and the remaining bits.
    ///
    /// # Panics
    /// Panics if the bits length is less than 6.
    fn decode_version_number(&mut self) -> PacketResult<'a> {
        let bits = self.bits;
        assert!(bits.len() >= 6, "Bits too short: {}", bits);
        let version_number = Self::binary_str_to_int(&bits[Self::VERSION_NUMBER]);
        let type_id = Self::binary_str_to_int(&bits[Self::TYPE_ID_RANGE]);

        if type_id == 4 {
            // base case
            let (evaluated_expression, rest) = Self::decode_literal(&bits[6..]);

            self.evaluated_expression = Some(evaluated_expression);

            PacketResult {
                version_number,
                rest,
            }
        } else {
            let mut compute_fn =
                Self::compute_from_type_id(type_id as usize, &mut self.evaluated_expression);

            if bits.as_bytes()[6] as char == '0' {
                let offset = 7 + 15;
                let sub_packet_length = Self::binary_str_to_int(&bits[7..offset]);

                let mut acc_version_number = version_number;
                let bits = &bits[offset..];
                let mut new_bits = &bits[..sub_packet_length as usize];

                while !new_bits.is_empty() {
                    let mut new_packet = Packet {
                        bits: new_bits,
                        evaluated_expression: None,
                    };

                    let PacketResult {
                        version_number,
                        rest,
                    } = new_packet.decode_version_number();

                    compute_fn(new_packet.evaluated_expression.unwrap());

                    new_bits = rest;
                    acc_version_number += version_number;
                }

                // Return the other packets that were not consumed in the fixed range
                PacketResult {
                    version_number: acc_version_number,
                    rest: &bits[sub_packet_length as usize..],
                }
            } else {
                let offset = 7 + 11;

                let sub_packet_length = Self::binary_str_to_int(&bits[7..offset]);
                let mut bits = &bits[offset..];
                let mut acc_version_number = version_number;

                for _ in 0..sub_packet_length {
                    let mut new_packet = Packet {
                        bits,
                        evaluated_expression: None,
                    };

                    let PacketResult {
                        version_number,
                        rest,
                    } = new_packet.decode_version_number();

                    compute_fn(new_packet.evaluated_expression.unwrap());

                    acc_version_number += version_number;
                    bits = rest;
                }

                PacketResult {
                    version_number: acc_version_number,
                    rest: bits,
                }
            }
        }
    }

    /// Returns a closure that modifies the accumulator based on the `type_id`.
    ///
    /// # Arguments
    /// * `type_id` - The type ID of the packet.
    /// * `acc` - A mutable reference to an optional accumulator value.
    ///
    /// # Panics
    /// Panics if the accumulator is not `None`.
    ///
    /// # Returns
    /// A closure that takes a `u64` value and modifies the accumulator.
    fn compute_from_type_id(type_id: usize, acc: &mut Option<u64>) -> Box<dyn FnMut(u64) + '_> {
        assert!(acc.is_none(), "Accumulator should be None");
        match type_id {
            0 => {
                *acc = Some(0);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    *acc_ref += b;
                })
            }
            1 => {
                *acc = Some(1);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    *acc_ref *= b;
                })
            }
            2 => {
                *acc = Some(u64::MAX);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    *acc_ref = (*acc_ref).min(b);
                })
            }
            3 => {
                *acc = Some(0);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    *acc_ref = (*acc_ref).max(b);
                })
            }
            5 => {
                *acc = Some(u64::MAX);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    if *acc_ref == u64::MAX {
                        *acc_ref = b;
                    } else if *acc_ref > b {
                        *acc_ref = 1;
                    } else {
                        *acc_ref = 0;
                    }
                })
            }
            6 => {
                *acc = Some(u64::MAX);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    if *acc_ref == u64::MAX {
                        *acc_ref = b;
                    } else if *acc_ref < b {
                        *acc_ref = 1;
                    } else {
                        *acc_ref = 0;
                    }
                })
            }
            7 => {
                *acc = Some(u64::MAX);
                Box::new(|b: u64| {
                    let acc_ref = acc.as_mut().unwrap();
                    if *acc_ref == u64::MAX {
                        *acc_ref = b;
                    } else if *acc_ref == b {
                        *acc_ref = 1;
                    } else {
                        *acc_ref = 0;
                    }
                })
            }
            _ => unreachable!("Invalid type_id: {}", type_id),
        }
    }

    /// Decodes a literal value from the packet's bits.
    ///
    /// # Arguments
    /// * `sub_bits` - A string slice representing the bits to decode.
    ///
    /// # Returns
    /// A tuple containing the decoded literal value and the remaining bits.
    fn decode_literal(mut sub_bits: &str) -> (u64, &str) {
        let mut acc = String::with_capacity(sub_bits.len());
        loop {
            acc.push_str(&sub_bits[1..=4]);

            if sub_bits.as_bytes()[0] == b'0' {
                return (Self::binary_str_to_int(&acc), &sub_bits[5..]);
            }

            sub_bits = &sub_bits[5..];
        }
    }

    /// Converts a binary string to an integer.
    ///
    /// # Arguments
    /// * `binary_string` - A string slice representing the binary string.
    ///
    /// # Returns
    /// A `u64` value representing the integer.
    fn binary_str_to_int(binary_string: &str) -> u64 {
        u64::from_str_radix(binary_string, 2).unwrap()
    }
}

fn hex_to_binary_strings(hex: &str) -> String {
    let mut bits = String::with_capacity(hex.len() * 4);
    for char in hex.chars() {
        bits.push_str(match char {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            other => panic!("Unknown char {}", other),
        });
    }

    bits
}
