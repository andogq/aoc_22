use std::{
    collections::{HashSet, VecDeque},
    mem,
};
use tqdm::tqdm;

use crate::day::Day;

type Position = (usize, usize);

const NUM_ROCKS_1: usize = 2022;
const NUM_ROCKS_2: usize = 1000000000000;
const WIDTH: usize = 7;
const Y_BUFFER: usize = 3;
const STARTING_X_OFFSET: usize = 2;
const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            c => {
                dbg!(c);
                unreachable!()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rock {
    points: Vec<Position>,
    height: usize,
    width: usize,
}

impl Rock {
    pub fn new(raw: &str) -> Self {
        let mut width = 0;
        let mut height = 0;

        let points = raw
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(
                    move |(x, c)| {
                        if c == '#' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .inspect(|&(x, y)| {
                width = width.max(x + 1);
                height = height.max(y + 1);
            })
            .collect();

        Self {
            points,
            width,
            height,
        }
    }
}

fn world_to_hashset(world: &VecDeque<[bool; WIDTH]>) -> HashSet<(usize, usize)> {
    world
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &c)| if c { Some((x, y)) } else { None })
        })
        .collect()
}

fn print_tower(world: &VecDeque<[bool; WIDTH]>) {
    world.iter().enumerate().rev().for_each(|(y, row)| {
        println!(
            "{y:>2} | {}",
            row.iter()
                .map(|&c| if c { "#" } else { " " })
                .collect::<Vec<_>>()
                .join(" ")
        );
    });
    println!("   -------------------");
    println!(
        "     {}",
        (0..WIDTH)
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

#[derive(Hash, PartialEq, Eq)]
struct TowerBitMask(u32);
impl From<&Vec<Vec<bool>>> for TowerBitMask {
    fn from(tower: &Vec<Vec<bool>>) -> Self {
        Self(
            tower
                .iter()
                .flatten()
                .fold(0, |mask, &b| (mask << 1) | (b as u32)),
        )
    }
}

fn simulate(rocks: Vec<Rock>, jets: Vec<Direction>, num_rocks: usize) -> usize {
    let largest_rock = rocks.iter().map(|r| r.height.max(r.width)).max().unwrap();
    let mut rocks = rocks.into_iter().cycle();
    let mut jets = jets.into_iter().cycle();

    let mut tower: VecDeque<[bool; WIDTH]> =
        vec![[false; WIDTH]; Y_BUFFER.max(largest_rock) + largest_rock].into();
    let mut tower_height = 0;

    for _ in tqdm(0..num_rocks) {
        // Create next rock
        let rock = rocks.next().unwrap();

        // Clear until top 3 rows are free
        while tower[tower.len() - Y_BUFFER.max(largest_rock)]
            .iter()
            .any(|&c| c)
        {
            // tower.pop_front();
            tower.push_back([false; WIDTH]);

            tower_height += 1;
        }

        // Find the correct place to drop from
        let tower_top = tower
            .iter()
            .enumerate()
            .rev() // start with top of tower
            .skip(Y_BUFFER.max(largest_rock))
            .find(|(_, row)| row.iter().any(|&c| c))
            .map(|(y, _)| y + 1)
            .unwrap_or(0);

        let mut position = (
            STARTING_X_OFFSET
                .saturating_add_signed(
                    (0..Y_BUFFER)
                        .map(|_| jets.next().unwrap())
                        .map(|j| match j {
                            Direction::Left => -1,
                            Direction::Right => 1,
                        })
                        .sum(),
                )
                .min(WIDTH - rock.width),
            tower_top,
        );

        let hashset_world = world_to_hashset(&tower);

        loop {
            let jet = jets.next().unwrap();

            let horizontal_no_collision = match jet {
                Direction::Left => {
                    if position.0 > 0 {
                        Some(-1)
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if position.0 + rock.width < WIDTH {
                        Some(1)
                    } else {
                        None
                    }
                }
            }
            .map(|dx: isize| {
                let moved_rock: HashSet<Position> = rock
                    .points
                    .iter()
                    .map(|&(x, y)| {
                        (
                            (x + position.0).checked_add_signed(dx).unwrap(),
                            y + position.1,
                        )
                    })
                    .collect();

                moved_rock.intersection(&hashset_world).count() == 0
            })
            .unwrap_or(false);

            if horizontal_no_collision {
                // Can move
                position.0 = match jet {
                    Direction::Left => position.0 - 1,
                    Direction::Right => position.0 + 1,
                };
            } else {
                // No movement L/R
            }

            // Check for collision below
            let moved_rock: HashSet<Position> = rock
                .points
                .iter()
                // TODO: This is really bad, should have a better way to check if it's at the
                // bottom
                .map(|&(x, y)| (x + position.0, (y + position.1).saturating_sub(1)))
                .collect();

            let blocked = position.1 == 0 || (moved_rock.intersection(&hashset_world).count() > 0);

            if !blocked {
                position.1 -= 1;
            } else {
                // Land the rock
                let moved_rock: HashSet<Position> = rock
                    .points
                    .iter()
                    .map(|&(x, y)| (x + position.0, y + position.1))
                    .collect();

                for pos in moved_rock {
                    tower[pos.1][pos.0] = true;
                }

                break;
            }
        }
    }

    tower.into_iter().filter(|r| r.iter().any(|&c| c)).count()
}

pub struct Day17;
impl Day for Day17 {
    type Input = (Vec<Rock>, Vec<Direction>);
    type Output = usize;

    fn part_1((rocks, jets): Self::Input) -> Self::Output {
        simulate(rocks, jets, NUM_ROCKS_1)
    }

    fn part_2((rocks, jets): Self::Input) -> Self::Output {
        // simulate(rocks, jets, NUM_ROCKS_2)
        0
        // let mut rocks = rocks.into_iter().enumerate().cycle();
        // let mut jets = jets.into_iter().enumerate().cycle();
        //
        // let mut tower: Vec<Vec<bool>> = VecDeque::new();
        //
        // let mut cache: HashSet<(TowerBitMask, usize, usize)> = HashSet::new();
        //
        // for _ in tqdm(0..NUM_ROCKS_2) {
        //     // Create next rock
        //     let (rock_i, rock) = rocks.next().unwrap();
        //
        //     let world_top = tower
        //         .iter()
        //         .enumerate()
        //         .rev()
        //         .find(|(_, row)| row.iter().any(|&c| c))
        //         .map(|(y, _)| y + 1)
        //         .unwrap_or(0);
        //
        //     // Add in rows so there's always 3 empty ones
        //     while (tower.len() - world_top) <= 3 {
        //         tower.push_back(vec![false; 7]);
        //     }
        //
        //     let mut position = STARTING_POSITION;
        //     position.1 = tower.len() - 1;
        //
        //     let hashset_world = world_to_hashset(&tower);
        //
        //     loop {
        //         let (jet_i, jet) = jets.next().unwrap();
        //
        //         let new = cache.insert((
        //             TowerBitMask::from(
        //                 &tower
        //                     .iter()
        //                     .rev()
        //                     .skip_while(|r| r.iter().all(|c| !c))
        //                     .take(4)
        //                     .cloned()
        //                     .collect(),
        //             ),
        //             rock_i,
        //             jet_i,
        //         ));
        //         if !new {
        //             println!("cache hit");
        //
        //             dbg!((
        //                 &tower
        //                     .iter()
        //                     .rev()
        //                     .skip_while(|r| r.iter().all(|c| !c))
        //                     .take(4)
        //                     .cloned()
        //                     .collect::<Vec<_>>(),
        //                 rock_i,
        //                 jet_i
        //             ));
        //         }
        //
        //         let horizontal_no_collision = match jet {
        //             Direction::Left => {
        //                 if position.0 > 0 {
        //                     Some(-1)
        //                 } else {
        //                     None
        //                 }
        //             }
        //             Direction::Right => {
        //                 if position.0 + rock.width < WIDTH {
        //                     Some(1)
        //                 } else {
        //                     None
        //                 }
        //             }
        //         }
        //         .map(|dx: isize| {
        //             let moved_rock: HashSet<Position> = rock
        //                 .points
        //                 .iter()
        //                 .map(|&(x, y)| {
        //                     (
        //                         (x + position.0).checked_add_signed(dx).unwrap(),
        //                         y + position.1,
        //                     )
        //                 })
        //                 .collect();
        //
        //             moved_rock.intersection(&hashset_world).count() == 0
        //         })
        //         .unwrap_or(false);
        //
        //         if horizontal_no_collision {
        //             // Can move
        //             position.0 = match jet {
        //                 Direction::Left => position.0 - 1,
        //                 Direction::Right => position.0 + 1,
        //             };
        //         } else {
        //             // No movement L/R
        //         }
        //
        //         // Check for collision below
        //         let moved_rock: HashSet<Position> = rock
        //             .points
        //             .iter()
        //             // TODO: This is really bad, should have a better way to check if it's at the
        //             // bottom
        //             .map(|&(x, y)| (x + position.0, (y + position.1).saturating_sub(1)))
        //             .collect();
        //
        //         let blocked =
        //             position.1 == 0 || (moved_rock.intersection(&hashset_world).count() > 0);
        //
        //         if !blocked {
        //             position.1 -= 1;
        //         } else {
        //             // Land the rock
        //             let moved_rock: HashSet<Position> = rock
        //                 .points
        //                 .iter()
        //                 .map(|&(x, y)| (x + position.0, y + position.1))
        //                 .collect();
        //
        //             // Increase world size as requried
        //             tower.extend(
        //                 (tower.len()..=rock.height.saturating_sub(position.1))
        //                     .map(|_| vec![false; 7]),
        //             );
        //
        //             for pos in moved_rock {
        //                 tower[pos.1][pos.0] = true;
        //             }
        //
        //             break;
        //         }
        //     }
        // }
        //
        // print_world(&tower);
        //
        // tower.into_iter().filter(|r| r.iter().any(|&c| c)).count()
    }

    fn parse(raw: &str) -> Self::Input {
        let rocks = ROCKS.split("\n\n").map(Rock::new).collect();

        let jets = raw
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(Direction::from)
            .collect();

        (rocks, jets)
    }
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    assert_eq!(Day17::run(input), (3068, 1514285714288));
}
