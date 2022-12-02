use crate::day::Day;

type Input = Vec<Vec<usize>>;

pub struct Day01;
impl Day01 {
    fn sum_elves(elves: &Input) -> Vec<usize> {
        elves.iter().map(|elf| elf.iter().sum()).collect()
    }
}
impl Day for Day01 {
    type Input = Input;
    type Output = usize;

    fn part_1(input: &Self::Input) -> Self::Output {
        *Self::sum_elves(&input).iter().max().unwrap()
    }

    fn part_2(input: &Self::Input) -> Self::Output {
        let mut elves = Self::sum_elves(&input);
        elves.sort_unstable();
        elves[elves.len() - 3..elves.len()].iter().sum()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines().fold(vec![vec![]], |mut elves, line| {
            if line.is_empty() {
                // Start a new elf
                elves.push(Vec::new());
            } else {
                // Add the value to the last elf
                elves.last_mut().unwrap().push(line.parse().unwrap());
            }

            elves
        })
    }
}

#[test]
fn test() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    assert_eq!(Day01::run(input), (24000, 45000));
}
