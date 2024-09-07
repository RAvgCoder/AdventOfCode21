mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

const DAYS_COMPLETED: [fn(); 11] = [
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
];

fn main() {
    // utils::helper_utils::Utils::new_day(11);
    DAYS_COMPLETED.iter().for_each(|day| {
        day();
        println!()
    });
    // DAYS_COMPLETED.last().unwrap()();
}
