use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::day::Day;

#[derive(Clone, Debug)]
pub enum Block {
    Rock,
    Sand,
}

type Position = (usize, usize);

#[derive(Clone)]
pub struct Map {
    map: HashMap<Position, Block>,
    sand_test: Option<Rc<dyn Fn(Position) -> bool>>,
    floor: Option<usize>,

    y_bound: usize,
    x_bound: (usize, usize),
}

impl Map {
    pub fn drop_sand(&mut self, start: Position) -> Option<Position> {
        let mut next_pos = start;

        loop {
            // Test the condition to stop dropping sand
            if let Some(ref sand_test) = self.sand_test {
                if sand_test(next_pos) {
                    return None;
                }
            }

            if let Some(floor) = self.floor {
                if next_pos.1 == floor {
                    break;
                }
            }

            let (down, left, right) = {
                let mut nodes = [0, -1, 1]
                    .into_iter()
                    .map(|dx| (next_pos.0.checked_add_signed(dx).unwrap(), next_pos.1 + 1))
                    .map(|pos| (self.map.get(&pos), pos));

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
        self.map.insert(next_pos, Block::Sand);

        // Update bounds
        self.y_bound = self.y_bound.max(next_pos.1);
        self.x_bound.0 = self.x_bound.0.min(next_pos.0);
        self.x_bound.1 = self.x_bound.1.max(next_pos.0);

        Some(next_pos)
    }
}

impl From<&str> for Map {
    fn from(raw: &str) -> Self {
        let (map, y_bound, x_bound) = raw
            .lines()
            .flat_map(|rock| {
                rock.to_owned()
                    .split(" -> ")
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
                    // TODO: Don't like that this has to be allocated, but &str references are
                    // causing problems
                    .collect::<Vec<_>>()
            })
            .fold(
                (HashMap::new(), 0, None::<(usize, usize)>),
                |(mut map, y_bound, x_bound), point: Position| {
                    map.insert(point, Block::Rock);

                    (
                        map,
                        y_bound.max(point.1),
                        if let Some(x_bound) = x_bound {
                            Some((x_bound.0.min(point.0), x_bound.1.max(point.0)))
                        } else {
                            Some(point)
                        },
                    )
                },
            );

        Self {
            map,
            sand_test: None,
            floor: None,

            x_bound: x_bound.unwrap(),
            y_bound,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .map
            .iter()
            .fold(
                vec![vec![' '; self.x_bound.1 - self.x_bound.0 + 1]; self.y_bound + 1],
                |mut output, (pos, c)| {
                    output[pos.1][pos.0 - self.x_bound.0] = match c {
                        Block::Rock => '█',
                        Block::Sand => '●',
                    };

                    output
                },
            )
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{output}")
    }
}

const SAND_ORIGIN: (usize, usize) = (500, 0);

pub struct Day14;
impl Day for Day14 {
    type Input = Map;
    type Output = usize;

    fn part_1(mut map: Self::Input) -> Self::Output {
        map.sand_test = Some(Rc::new(move |pos| pos.1 >= map.y_bound));

        let mut sand_counter = 0;
        loop {
            if map.drop_sand(SAND_ORIGIN).is_none() {
                break;
            } else {
                sand_counter += 1;
            }
        }

        sand_counter
    }

    fn part_2(mut map: Self::Input) -> Self::Output {
        map.floor = Some(map.y_bound + 1);

        let mut sand_counter = 0;
        loop {
            let dropped = map.drop_sand(SAND_ORIGIN);
            sand_counter += 1;

            if let Some(SAND_ORIGIN) = dropped {
                break;
            }
        }

        println!("{map}");

        sand_counter
    }

    fn parse(raw: &str) -> Self::Input {
        raw.into()
    }
}

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(Day14::run(input), (24, 93));
}
