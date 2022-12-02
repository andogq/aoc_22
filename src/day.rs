use std::time::{Duration, Instant};

fn bench<F>(n: u32, callback: F) -> Duration
where
    F: Fn(),
{
    let mut total = Duration::ZERO;

    for _ in 0..n {
        let now = Instant::now();

        callback();

        total += now.elapsed();
    }

    total
}

pub trait Day {
    type Input: Clone;
    type Output;

    fn part_1(input: &Self::Input) -> Self::Output;
    fn part_2(input: &Self::Input) -> Self::Output;

    fn parse(raw: &str) -> Self::Input;

    fn run(input: &str) -> (Self::Output, Self::Output) {
        let input = Self::parse(input);

        (Self::part_1(&input), Self::part_2(&input))
    }

    fn bench(input: &str, n: u32) {
        println!("Performing bench mark");

        // Benchmark parse
        let parse_total = bench(n, || {
            Self::parse(input);
        });
        println!("    parse: {}ns", (parse_total / n).as_nanos());

        let input = Self::parse(input);

        // Benchmark part 1
        let part_1_total = bench(n, || {
            Self::part_1(&input);
        });
        println!("    part 1: {}ns", (part_1_total / n).as_nanos());

        // Benchmark part 2
        let part_2_total = bench(n, || {
            Self::part_2(&input);
        });
        println!("    part 2: {}ns", (part_2_total / n).as_nanos());
    }
}
