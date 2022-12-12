use std::collections::HashSet;

use crate::day::Day;

type Position = (isize, isize);

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

impl From<&Direction> for (isize, isize) {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        Rope {
            knots: vec![(0, 0); length],
        }
    }

    pub fn count_tail_locations(length: usize, directions: &[Direction]) -> usize {
        let mut rope = Rope::new(length);

        let mut visited = HashSet::new();

        for direction in directions {
            rope.step(direction);

            visited.insert(rope.tail());
        }

        visited.len()
    }

    pub fn tail(&self) -> Position {
        self.knots.last().unwrap().to_owned()
    }

    pub fn step(&mut self, direction: &Direction) {
        let (dx, dy) = direction.into();

        let mut new_positions = Vec::with_capacity(self.knots.len());

        for (i, knot) in self.knots.iter().enumerate() {
            let mut knot = knot.to_owned();

            let (dx, dy) = if i == 0 {
                (dx, dy)
            } else {
                let head: Position = new_positions[i - 1];

                let dx = head.0 - knot.0;
                let dy = head.1 - knot.1;

                if dx.abs() >= 2 || dy.abs() >= 2 {
                    (dx.signum(), dy.signum())
                } else {
                    (0, 0)
                }
            };

            knot.0 += dx;
            knot.1 += dy;

            new_positions.push(knot);
        }

        self.knots = new_positions;
    }
}

pub struct Day09;
impl Day for Day09 {
    type Input = Vec<Direction>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        Rope::count_tail_locations(2, &input)
    }

    fn part_2(input: Self::Input) -> Self::Output {
        Rope::count_tail_locations(10, &input)
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
    assert_eq!(Day09::run(input), (88, 36));
}
