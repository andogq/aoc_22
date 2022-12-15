use std::collections::{HashMap, VecDeque};

use crate::day::Day;

#[derive(Clone, Debug)]
pub enum Block {
    Rock,
    Sand,
}

const SAND_ORIGIN: (usize, usize) = (500, 0);

fn print_map(map: &HashMap<(usize, usize), Block>) {
    let mut origin = map.keys().next().unwrap().to_owned().0;
    let mut max = origin;
    let mut output = {
        let mut v = VecDeque::new();
        v.push_back(VecDeque::new());

        v
    };

    for (k, v) in map {
        // Resize to the left
        if k.0 < origin {
            // Need to shift origin
            let difference = origin - k.0;
            for row in output.iter_mut() {
                for _ in 0..difference {
                    row.push_front(None);
                }
            }

            origin = k.0;
        }

        // Resize to the right
        if k.0 >= max {
            for row in output.iter_mut() {
                row.extend((max..k.0).map(|_| None));
            }

            max = k.0;
        }

        // Add required rows
        while k.1 >= output.len() {
            output.push_back(VecDeque::from_iter((0..=output[0].len()).map(|_| None)));
        }

        output[k.1][k.0 - origin] = Some(v);
    }

    for row in output {
        for c in row {
            print!(
                "{}",
                match c {
                    None => ' ',
                    Some(Block::Rock) => '█',
                    Some(Block::Sand) => '●',
                }
            );
        }

        println!();
    }
}

pub struct Day14;
impl Day for Day14 {
    type Input = HashMap<(usize, usize), Block>;
    type Output = usize;

    fn part_1(mut map: Self::Input) -> Self::Output {
        let abyss = map.keys().max_by_key(|k| k.1).unwrap().1;
        let mut sand_counter = 0;

        let mut drop_sand = || -> bool {
            let mut next_pos = SAND_ORIGIN;

            loop {
                if next_pos.1 >= abyss {
                    return false;
                }

                let (down, left, right) = {
                    let mut nodes = [0, -1, 1]
                        .into_iter()
                        .map(|dx| (next_pos.0.checked_add_signed(dx).unwrap(), next_pos.1 + 1))
                        .map(|pos| (map.get(&pos), pos));

                    (
                        nodes.next().unwrap(),
                        nodes.next().unwrap(),
                        nodes.next().unwrap(),
                    )
                };

                next_pos = match (down.0, left.0, right.0) {
                    (None, _, _) => down.1,
                    (Some(_), None, _) => left.1,
                    (Some(_), Some(_), None) => right.1,
                    (Some(_), Some(_), Some(_)) => {
                        // Come to rest
                        break;
                    }
                }
            }

            // Insert sand position
            map.insert(next_pos, Block::Sand);
            sand_counter += 1;

            true
        };

        // wtf lol
        while drop_sand() {}

        sand_counter
    }

    fn part_2(mut map: Self::Input) -> Self::Output {
        let floor = map.keys().max_by_key(|k| k.1).unwrap().1 + 1;

        let mut sand_counter = 0;

        let mut drop_sand = || -> bool {
            let mut next_pos = SAND_ORIGIN;

            loop {
                if next_pos.1 == floor {
                    // Hit the floor
                    break;
                }

                let (down, left, right) = {
                    let mut nodes = [0, -1, 1]
                        .into_iter()
                        .map(|dx| (next_pos.0.checked_add_signed(dx).unwrap(), next_pos.1 + 1))
                        .map(|pos| (map.get(&pos), pos));

                    (
                        nodes.next().unwrap(),
                        nodes.next().unwrap(),
                        nodes.next().unwrap(),
                    )
                };

                next_pos = match (down.0, left.0, right.0) {
                    (None, _, _) => down.1,
                    (Some(_), None, _) => left.1,
                    (Some(_), Some(_), None) => right.1,
                    (Some(_), Some(_), Some(_)) => {
                        // Come to rest
                        break;
                    }
                }
            }

            // Insert sand position
            map.insert(next_pos, Block::Sand);
            sand_counter += 1;

            next_pos != SAND_ORIGIN
        };

        // wtf lol
        while drop_sand() {}

        print_map(&map);

        sand_counter
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines().fold(HashMap::new(), |mut map, rock| {
            rock.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>()[..]
                .windows(2)
                .flat_map(|points| {
                    let start = points[0];
                    let end = points[1];

                    if start.0 == end.0 {
                        let (low, high) = if start.1 > end.1 {
                            (end.1, start.1)
                        } else {
                            (start.1, end.1)
                        };
                        (low..=high).map(|y| (start.0, y)).collect::<Vec<_>>()
                    } else {
                        let (low, high) = if start.0 > end.0 {
                            (end.0, start.0)
                        } else {
                            (start.0, end.0)
                        };
                        (low..=high).map(|x| (x, start.1)).collect::<Vec<_>>()
                    }
                })
                .for_each(|pos| {
                    map.insert(pos, Block::Rock);
                });

            map
        })
    }
}

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(Day14::run(input), (24, 93));
}
