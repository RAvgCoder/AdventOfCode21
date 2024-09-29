use std::ops::Not;

use day_setup::Utils;

use crate::utils::day_setup;

pub fn run() {
    // run_part(function, part_num, day_num)
    Utils::run_part(part1, 1, 3, Some(1997414));
    Utils::run_part(part2, 2, 3, Some(1032597));
}
const BIT_SIZE: usize = 12;

fn part1(read_file: Vec<String>) -> u64 {
    let gamma = find_frequency(&read_file);
    const MASK: i32 = !(-1 << BIT_SIZE);
    let epsilon = gamma.not() & MASK;

    (epsilon * gamma) as u64
}

fn find_frequency(read_file: &[String]) -> i32 {
    let mut counter = [0i16; BIT_SIZE];

    for l in read_file.iter() {
        for (idx, c) in l.chars().enumerate() {
            counter[idx] += match c {
                '0' => 1,
                '1' => -1,
                _ => {
                    unreachable!("{}", c)
                }
            }
        }
    }

    let mut gamma = 0;
    for i in &counter {
        gamma <<= 1;
        gamma |= if *i <= 0 { 1 } else { 0 }
    }
    gamma
}

fn part2(read_file: Vec<String>) -> u64 {
    let mut oxygen = vec![];
    let mut co2 = vec![];
    for s in &read_file {
        oxygen.push(s.clone());
        co2.push(s.clone());
    }

    // oxygen generator rating
    for i in (0..BIT_SIZE).rev() {
        let oxygen_len = oxygen.len();
        let mut acc_oxygen: Vec<String> = Vec::with_capacity(oxygen_len);
        let co2_len = co2.len();
        let mut acc_o2: Vec<String> = Vec::with_capacity(co2_len);
        let freq = find_frequency(&oxygen);
        let bit = (freq >> i) & 1;
        if oxygen_len > 1 {
            for x in oxygen {
                let bytes = x.as_bytes();
                if bit == (bytes[(BIT_SIZE - 1) - i] - b'0') as i32 {
                    acc_oxygen.push(x);
                }
            }
            oxygen = acc_oxygen;
        }

        let freq = find_frequency(&co2);
        let bit = (freq >> i) & 1;
        if co2_len > 1 {
            for o in co2 {
                let bytes = o.as_bytes();
                if bit != (bytes[(BIT_SIZE - 1) - i] - b'0') as i32 {
                    acc_o2.push(o);
                }
            }
            co2 = acc_o2;
        }
    }

    let oxygen_rating = i32::from_str_radix(oxygen.first().unwrap(), 2).unwrap();
    let co2_rating = i32::from_str_radix(co2.first().unwrap(), 2).unwrap();

    co2_rating as u64 * oxygen_rating as u64
}
