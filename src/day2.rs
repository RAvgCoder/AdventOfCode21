use crate::utils::day_setup;

pub fn run() {
    // run_part(function, part_num, day_num)
    day_setup::Utils::run_part(part1, 1, 2, Some(1714680));
    day_setup::Utils::run_part(part2, 2, 2, Some(1963088820));
}

fn part1(input: Vec<String>) -> u64 {
    let mut horizontal: u32 = 0;
    let mut depth: i16 = 0;
    for line in input {
        let mut info = line.split_whitespace();
        let name = info.next().unwrap();
        let x = info.next().unwrap().parse::<u32>().unwrap();
        match name {
            "forward" => horizontal += x,
            "down" => depth += x as i16,
            "up" => depth -= x as i16,
            _ => unreachable!(),
        }
    }
    (horizontal as i32 * depth as i32) as u64
}

fn part2(input: Vec<String>) -> u64 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: i16 = 0;
    for line in input {
        let mut info = line.split_whitespace();
        let name = info.next().unwrap();
        let x = info.next().unwrap().parse::<u32>().unwrap();
        match name {
            "forward" => {
                horizontal += x;
                depth += aim as u32 * x;
            }
            "down" => aim += x as i16,
            "up" => aim -= x as i16,
            _ => unreachable!(),
        }
    }
    (horizontal * depth) as u64
}
