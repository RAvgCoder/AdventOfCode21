mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    // utils::day_setup::Utils::new_day(21);
    [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        day10::run,
        day11::run,
        day14::run,
        day16::run,
        day17::run,
        day18::run,
        day12::run, // Incomplete
        day13::run,
        day15::run,
        day19::run, // Incomplete
        day20::run, // Incomplete
        day21::run, // Incomplete
    ]
    // .iter().for_each(|day| { day(); println!() });
    .last()
    .unwrap()();
}
