use std::{collections::HashSet, ops::Deref};
use tqdm::tqdm;

use crate::day::Day;

const NUM_ROCKS_1: usize = 2022;
const NUM_ROCKS_2: usize = 1000000000000;
const WIDTH: usize = 7;
const Y_BUFFER: usize = 3;
const STARTING_X_OFFSET: usize = 2;
const ROCK_SIZE: usize = 4;
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
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rock {
    points: [[bool; ROCK_SIZE]; ROCK_SIZE],
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
            .chain((raw.lines().count()..ROCK_SIZE).map(|_| ""))
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .chain((row.chars().count()..ROCK_SIZE).map(|_| '.'))
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '#' {
                            width = width.max(x + 1);
                            height = height.max(y + 1);

                            true
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            points,
            width,
            height,
        }
    }
}

impl Deref for Rock {
    type Target = [[bool; ROCK_SIZE]; ROCK_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

fn world_to_hashset(world: &[Row]) -> HashSet<(usize, usize)> {
    world
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.as_parts()
                .into_iter()
                .enumerate()
                .filter_map(move |(x, c)| if c { Some((x, y)) } else { None })
        })
        .collect()
}

fn print_tower(world: &[Row]) {
    world.iter().enumerate().rev().for_each(|(y, row)| {
        println!(
            "{y:>2} | {}",
            row.as_parts()
                .into_iter()
                .map(|c| if c { "#" } else { " " })
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

#[derive(Clone)]
pub struct Row(u8);
impl Row {
    pub fn new() -> Self {
        Row(0)
    }

    pub fn set(&mut self, i: usize) {
        self.0 |= 1 << i as u8;
    }

    pub fn overlap(&self, points: &[usize]) -> bool {
        points.iter().any(|&i| (self.0 >> i as u8) & 1 == 1)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn as_parts(&self) -> [bool; WIDTH] {
        (0..WIDTH).into_iter().fold([false; WIDTH], |mut arr, i| {
            arr[i] = (self.0 >> i) & 1 == 1;

            arr
        })
    }
}

fn simulate(rocks: Vec<Rock>, jets: Vec<Direction>, num_rocks: usize) -> usize {
    let largest_rock = rocks.iter().map(|r| r.height.max(r.width)).max().unwrap();
    let mut rocks = rocks.into_iter().cycle();
    let mut jets = jets.into_iter().cycle();

    let mut tower: Vec<Row> = vec![Row::new(); Y_BUFFER + ROCK_SIZE];

    for _ in tqdm(0..num_rocks) {
        // Create next rock
        let rock = rocks.next().unwrap();

        // Clear until top 3 rows are free
        while !tower[tower.len() - Y_BUFFER.max(largest_rock)].is_empty() {
            // tower.pop_front();
            tower.push(Row::new());
        }

        // Find the correct place to drop from
        let tower_top = tower
            .iter()
            .enumerate()
            .rev() // start with top of tower
            .skip(Y_BUFFER.max(largest_rock))
            .find(|(_, row)| !row.is_empty())
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

        loop {
            let jet = jets.next().unwrap();

            let horizontal_collision = match jet {
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
                // TODO: Check with wall
                let rock_x = position.0.saturating_add_signed(dx).min(7);

                // Check if new direction will collide with another rock
                rock.iter().enumerate().any(|(y, row)| {
                    let world_y = position.1 + y;

                    if world_y < tower.len() {
                        let row = row
                            .iter()
                            .enumerate()
                            .filter(|(_, &c)| c)
                            .map(|(x, _)| x + rock_x)
                            .collect::<Vec<_>>();

                        tower[position.1 + y].overlap(&row)
                    } else {
                        // Row is above world, so won't collide
                        false
                    }
                })
            })
            .unwrap_or(true);

            if !horizontal_collision {
                // Can move
                position.0 = match jet {
                    Direction::Left => position.0 - 1,
                    Direction::Right => position.0 + 1,
                };
            }

            // Check for collision below
            let vertical_collision = position.1 == 0
                || rock.iter().enumerate().any(|(y, row)| {
                    let row = row
                        .iter()
                        .enumerate()
                        .filter(|(_, &c)| c)
                        .map(|(x, _)| x + position.0)
                        .collect::<Vec<_>>();

                    tower[(position.1 - 1 + y)].overlap(&row)
                });

            if !vertical_collision {
                position.1 -= 1;
            } else {
                // Land the rock
                for (y, row) in rock[0..rock.height].iter().enumerate() {
                    let tower_row = &mut tower[position.1 + y];

                    for x in row.iter().enumerate().filter(|(_, &c)| c).map(|(x, _)| x) {
                        tower_row.set(position.0 + x);
                    }
                }

                break;
            }
        }
    }

    print_tower(&tower);

    tower.into_iter().filter(|r| !r.is_empty()).count()
}

pub struct Day17;
impl Day for Day17 {
    type Input = (Vec<Rock>, Vec<Direction>);
    type Output = usize;

    fn part_1((rocks, jets): Self::Input) -> Self::Output {
        simulate(rocks, jets, NUM_ROCKS_1)
    }

    fn part_2((rocks, jets): Self::Input) -> Self::Output {
        0
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
