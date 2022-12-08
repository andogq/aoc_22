use std::fs::read_to_string;

use day::Day;

mod day;
mod problems;

macro_rules! run_day {
    ($day_number:literal, $day:path) => {{
        let input = read_to_string(format!("inputs/day_{:02}.txt", $day_number)).unwrap();
        type Day = $day;
        println!("Day {:02}: {:?}", $day_number, Day::run(&input));
    }};
}

fn main() {
    run_day!(1, problems::day_01::Day01);
    run_day!(2, problems::day_02::Day02);
    run_day!(3, problems::day_03::Day03);
    run_day!(4, problems::day_04::Day04);
    run_day!(5, problems::day_05::Day05);
    run_day!(6, problems::day_06::Day06);
    run_day!(7, problems::day_07::Day07);
    run_day!(8, problems::day_08::Day08);
}
