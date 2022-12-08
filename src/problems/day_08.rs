use std::collections::HashSet;

use crate::day::Day;

pub struct Day08;
impl Day for Day08 {
    type Input = Vec<Vec<usize>>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        let mut remaining = (input.len() * 2) + (input[0].len() * 2) - 4;
        let mut added = HashSet::new();

        for y in 0..input.len() {
            let mut largest = None;
            for x in 0..input[y].len() {
                let this = input[y][x];

                if let Some(l) = largest {
                    if x > 0 {
                        let prev = input[y][x - 1];

                        if this > prev && this > l {
                            largest = Some(this);
                            added.insert((x, y));
                        }

                        continue;
                    }
                }

                largest = Some(this);
                added.insert((x, y));
            }

            let mut largest = None;
            for x in (0..input[y].len()).rev() {
                let this = input[y][x];

                if let Some(l) = largest {
                    if x < input[y].len() - 1 {
                        let prev = input[y][x + 1];

                        if this > prev && this > l {
                            largest = Some(this);
                            added.insert((x, y));
                        }

                        continue;
                    }
                }

                largest = Some(this);
                added.insert((x, y));
            }
        }
        for x in 0..input[0].len() {
            let mut largest = None;
            for y in 0..input.len() {
                let this = input[y][x];

                if let Some(l) = largest {
                    if y > 0 {
                        let prev = input[y - 1][x];

                        if this > prev && this > l {
                            largest = Some(this);
                            added.insert((x, y));
                        }

                        continue;
                    }
                }

                largest = Some(this);
                added.insert((x, y));
            }

            let mut largest = None;
            for y in (0..input.len()).rev() {
                let this = input[y][x];

                if let Some(l) = largest {
                    if y < input.len() - 1 {
                        let prev = input[y + 1][x];

                        if this > prev && this > l {
                            largest = Some(this);
                            added.insert((x, y));
                        }

                        continue;
                    }
                }

                largest = Some(this);
                added.insert((x, y));
            }
        }

        added.len()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut largest = 0;

        for y in 0..input.len() {
            for x in 0..input[y].len() {
                let mut score = 1;
                let tree = input[y][x];

                let mut counter = 0;
                for new_y in (0..y).rev() {
                    counter += 1;
                    if input[new_y][x] >= tree {
                        break;
                    }
                }
                score *= counter;

                let mut counter = 0;
                for new_y in y + 1..input.len() {
                    counter += 1;
                    if input[new_y][x] >= tree {
                        break;
                    }
                }
                score *= counter;

                let mut counter = 0;
                for new_x in (0..x).rev() {
                    counter += 1;
                    if input[y][new_x] >= tree {
                        break;
                    }
                }
                score *= counter;

                let mut counter = 0;
                for new_x in x + 1..input[y].len() {
                    counter += 1;
                    if input[y][new_x] >= tree {
                        break;
                    }
                }
                score *= counter;

                if score > largest {
                    largest = score;
                }
            }
        }

        largest
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|line| {
                line.chars()
                    .map(|c| (c as usize) - ('0' as usize))
                    .collect()
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "30373
25512
65332
33549
35390";

    assert_eq!(Day08::run(input), (21, 8));
}
