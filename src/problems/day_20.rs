use crate::day::Day;

pub struct Day20;

impl Day for Day20 {
    type Input = Vec<isize>;
    type Output = isize;

    fn part_1(input: Self::Input) -> Self::Output {
        let mut input: Vec<_> = input.into_iter().map(|n| (false, n)).collect();
        let input_size = input.len() - 1;

        while let Some(next_i) = {
            let mut next_index = None;

            for (i, (checked, _)) in input.iter().enumerate() {
                if !checked {
                    next_index = Some(i);
                    break;
                }
            }

            next_index
        } {
            // Get number
            let (_, offset) = input.remove(next_i);

            let mut new_index = ((input_size + next_i) as isize + (offset % input_size as isize))
                as usize
                % input_size;

            if new_index == 0 {
                new_index = input_size;
            }

            input.insert(new_index, (true, offset));

            // dbg!(offset, next_i, new_index, &input);
        }

        let base_i = input
            .iter()
            .enumerate()
            .find(|(_, &(_, n))| n == 0)
            .map(|(i, _)| i)
            .unwrap();

        (1..=3)
            .map(|n| input[(base_i + (n * 1000)) % input.len()].1)
            .inspect(|n| {
                // dbg!(n);
            })
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        0
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines().map(|n| n.parse().unwrap()).collect()
    }
}

#[test]
fn test() {
    let input = "1
2
-3
3
-2
0
4
";
    assert_eq!(Day20::run(input), (3, 0));
}
