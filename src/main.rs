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
    run_day!(9, problems::day_09::Day09);
    run_day!(10, problems::day_10::Day10);
    run_day!(11, problems::day_11::Day11);
    run_day!(12, problems::day_12::Day12);
    run_day!(13, problems::day_13::Day13);
    run_day!(14, problems::day_14::Day14);
    run_day!(15, problems::day_15::Day15);
    run_day!(16, problems::day_16::Day16);
    run_day!(17, problems::day_17::Day17);
}
