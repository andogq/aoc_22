use std::{cmp::Ordering, collections::HashSet};

use crate::day::Day;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

fn next_position(head: (isize, isize), mut tail: (isize, isize)) -> (isize, isize) {
    // Update tail position
    fn find_next(d: isize) -> isize {
        match d.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Less => -1,
            Ordering::Greater => 1,
        }
    }

    if (tail.0 - head.0).abs() >= 2 || (tail.1 - head.1).abs() >= 2 {
        tail.0 = tail.0 + find_next(head.0 - tail.0);
        tail.1 = tail.1 + find_next(head.1 - tail.1);
    }

    tail
}

pub struct Day09;
impl Day for Day09 {
    type Input = Vec<Direction>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        let mut visited = HashSet::new();

        let mut head_position = (0, 0);
        let mut tail_position = (0, 0);

        for direction in input {
            let (dx, dy) = match direction {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            // Update head position
            head_position.0 += dx;
            head_position.1 += dy;

            // Update tail position
            tail_position = next_position(head_position, tail_position);

            visited.insert(tail_position);
        }

        visited.len()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut positions = vec![(0, 0); 10];
        let mut visited = HashSet::new();

        for direction in input {
            let mut next_positions: Vec<(isize, isize)> = Vec::with_capacity(10);

            for (i, knot) in positions.iter().enumerate() {
                let mut knot = knot.to_owned();

                if i == 0 {
                    let (dx, dy) = match direction {
                        Direction::Up => (0, 1),
                        Direction::Down => (0, -1),
                        Direction::Left => (-1, 0),
                        Direction::Right => (1, 0),
                    };
                    knot.0 += dx;
                    knot.1 += dy;
                } else {
                    knot = next_position(next_positions[i - 1].to_owned(), knot.to_owned());
                }

                next_positions.push(knot);
            }

            visited.insert(next_positions[9]);

            positions = next_positions;
        }

        visited.len()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .flat_map(|line| {
                let (direction, distance) = line.split_once(' ').unwrap();
                let distance = distance.parse().unwrap();
                let direction = Direction::from(direction.chars().next().unwrap());

                vec![direction; distance]
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(Day09::run(input), (13, 1));

    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(Day09::run(input), (0, 36));
}
