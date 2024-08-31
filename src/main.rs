mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

const DAYS_COMPLETED: [fn(); 7] = [
    day1::run,
    day2::run,
    day3::run,
    day4::run,
    day5::run,
    day6::run,
    day7::run,
];

fn main() {
    // utils::helper_utils::Utils::new_day(8);
    DAYS_COMPLETED.last().unwrap()();
}
