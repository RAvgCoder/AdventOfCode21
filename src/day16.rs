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
    Utils::run_part(part2, 2, 0, None);
}

// 199075848 Too High

type PacketResult<'rest> = (u64, &'rest str);

fn part1(mut input: Vec<String>) -> u64 {
    let binary_strings = hex_to_binary_strings(input.pop().unwrap());
    let packet = Packet { bits: &binary_strings };

    packet.decode_version_number().0
}

fn part2(_input: Vec<String>) -> u64 {
    // let binary_strings = hex_to_binary_strings(input.pop().unwrap());
    // let packet = Packet { bits: &binary_strings };
    // // println!("Part 1: {:#?}", packet);
    // packet.decode_version_number();
    0
}

#[derive(Debug)]
struct Packet<'a> {
    bits: &'a str,
}

impl<'a> Packet<'a> {
    const VERSION_NUMBER: RangeInclusive<usize> = 0..=2;
    const TYPE_ID_RANGE: RangeInclusive<usize> = 3..=5;

    ///110100101111111000101000
    /// VVVTTTAAAAABBBBBCCCCC
    /// 
    /// 001 110 0 000000000011011 11010001010 0101001000100100 0000000
    /// VVV TTT I LLLLLLLLLLLLLLL AAAAAAAAAAA BBBBBBBBBBBBBBBB
    fn decode_version_number(&self) -> PacketResult<'a> {
        let bits = self.bits;
        assert!(bits.len() >= 6, "Bits too short: {}", bits);
        let version_number = Self::binary_str_to_int(&bits[Self::VERSION_NUMBER]);
        let type_id = Self::binary_str_to_int(&bits[Self::TYPE_ID_RANGE]);

        println!("Version Number: {} Type ID: {} | {}", version_number, type_id, "bits");

        if type_id == 4 { // base case
            let (_, bits) = Self::decode_literal(&bits[6..]);
            println!("BASE4: {:?}", version_number);
            (version_number, bits)
        } else if bits.as_bytes()[6] as char == '0' {
            let offset = 7 + 15;
            let sub_packet_length = Self::binary_str_to_int(&bits[7..offset]);

            let mut acc_version_number = version_number;
            let bits = &bits[offset..];
            let mut new_bits = &bits[..sub_packet_length as usize];

            while !new_bits.is_empty() {
                let (v_num, rest) = Packet { bits: new_bits }.decode_version_number();
                println!("0: {:?}", v_num);
                new_bits = rest;
                acc_version_number += v_num;
            }

            // Return the other packets that were not consumed in the fixed range 
            (acc_version_number, &bits[sub_packet_length as usize..])
        } else {
            let offset = 7 + 11;

            let sub_packet_length = Self::binary_str_to_int(&bits[7..offset]);
            let mut bits = &bits[offset..];
            let mut acc_version_number = version_number;

            for _ in 0..sub_packet_length {
                let (v_num, rest) = Packet { bits }.decode_version_number();
                println!("1: {:?}", v_num);
                acc_version_number += v_num;
                bits = rest;
            }

            (acc_version_number, bits)
        }
    }

    fn decode_literal(mut sub_bits: &str) -> PacketResult {
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

fn hex_to_binary_strings(hex: String) -> String {
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
