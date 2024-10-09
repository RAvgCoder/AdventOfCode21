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

fn part2(input: Vec<String>) -> u128 {
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
    bits: &'a str,
    evaluated_expression: Option<u128>,
}


impl<'a> Packet<'a> {
    const VERSION_NUMBER: RangeInclusive<usize> = 0..=2;
    const TYPE_ID_RANGE: RangeInclusive<usize> = 3..=5;

    ///110100101111111000101000
    /// VVVTTTAAAAABBBBBCCCCC
    ///
    /// 001 110 0 000000000011011 11010001010 0101001000100100 0000000
    /// VVV TTT I LLLLLLLLLLLLLLL AAAAAAAAAAA BBBBBBBBBBBBBBBB
    fn decode_version_number(&mut self) -> PacketResult<'a> {
        let bits = self.bits;
        assert!(bits.len() >= 6, "Bits too short: {}", bits);
        let version_number = Self::binary_str_to_int(&bits[Self::VERSION_NUMBER]);
        let type_id = Self::binary_str_to_int(&bits[Self::TYPE_ID_RANGE]);

        if type_id == 4 {
            // base case
            let (evaluated_expression, rest) = Self::decode_literal(&bits[6..]);

            self.evaluated_expression = Some(evaluated_expression as u128);

            PacketResult {
                version_number,
                rest,
            }
        } else {

            assert!(self.evaluated_expression.is_none(), "Evaluated expression should be None");
            
            // Initialize the evaluated expression to act as the accumulator and return the compute function
            let compute_fn = Self::compute_from_type_id(type_id as usize, &mut self.evaluated_expression);

            let mut acc = self.evaluated_expression.unwrap();

            if bits.as_bytes()[6] as char == '0' {
                let offset = 7 + 15;
                let sub_packet_length = Self::binary_str_to_int(&bits[7..offset]);

                let mut acc_version_number = version_number;
                let bits = &bits[offset..];
                let mut new_bits = &bits[..sub_packet_length as usize];

                while !new_bits.is_empty() {
                    let mut new_packet = Packet { bits: new_bits, evaluated_expression: None };
                    let PacketResult { version_number, rest } = new_packet.decode_version_number();

                    acc = compute_fn(acc, new_packet.evaluated_expression.unwrap());

                    new_bits = rest;
                    acc_version_number += version_number;
                }

                self.evaluated_expression = Some(acc);

                // Return the other packets that were not consumed in the fixed range
                // (acc_version_number, &bits[sub_packet_length as usize..])
                PacketResult {
                    version_number: acc_version_number,
                    // evaluated_expression: acc_version_number,
                    rest: &bits[sub_packet_length as usize..],
                }
            } else {
                let offset = 7 + 11;

                let sub_packet_length = Self::binary_str_to_int(&bits[7..offset]);
                let mut bits = &bits[offset..];
                let mut acc_version_number = version_number;

                for _ in 0..sub_packet_length {
                    let mut new_packet = Packet { bits, evaluated_expression: None };
                    let PacketResult { version_number, rest } = new_packet.decode_version_number();

                    acc = compute_fn(acc, new_packet.evaluated_expression.unwrap());

                    acc_version_number += version_number;
                    bits = rest;
                }

                self.evaluated_expression = Some(acc);

                // (acc_version_number, bits)
                PacketResult {
                    version_number: acc_version_number,
                    // evaluated_expression: acc_version_number,
                    rest: bits,
                }
            }
        }
    }

    fn compute_from_type_id(type_id: usize, acc: &mut Option<u128>) -> impl Fn(u128, u128) -> u128 {
        type ComputeTool = (u128, fn(u128, u128) -> u128);
        const FUNCTIONS: [ComputeTool; 7] = [
            (0, |a: u128, b| a + b),          // type_id 0
            (1, |a: u128, b| a * b),          // type_id 1
            (u128::MAX, |a: u128, b| a.min(b)),       // type_id 2
            (0, |a: u128, b| a.max(b)),       // type_id 3
            (u128::MAX, |a, b| {
                if a == u128::MAX { return b; }
                if a > b { 1_u128 } else { 0 }
            }),  // type_id 5
            (u128::MAX, |a, b| {
                if a == u128::MAX { return b; }
                if a < b { 1_u128 } else { 0 }
            }),  // type_id 6
            (u128::MAX, |a, b| {
                if a == u128::MAX { return b; }
                if a == b { 1_u128 } else { 0 }
            }), // type_id 7
        ];

        let result = match type_id {
            0..=3 => FUNCTIONS[type_id],
            5..=7 => FUNCTIONS[type_id - 1],
            _ => unreachable!("Invalid type_id: {}", type_id),
        };

        *acc = Some(result.0);

        result.1
    }

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
