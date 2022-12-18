use std::{
    collections::{HashSet, VecDeque},
    ops::Deref,
};

use crate::day::Day;

#[derive(Clone)]
pub struct Droplet {
    cubes: HashSet<Point>,
    x_bound: usize,
    y_bound: usize,
    z_bound: usize,
}

impl Deref for Droplet {
    type Target = HashSet<Point>;

    fn deref(&self) -> &Self::Target {
        &self.cubes
    }
}

impl From<&str> for Droplet {
    fn from(value: &str) -> Self {
        let mut x_bound = 0;
        let mut y_bound = 0;
        let mut z_bound = 0;

        Self {
            cubes: value
                .lines()
                .map(|raw| {
                    let cube: Point = raw.into();

                    x_bound = x_bound.max(cube.x);
                    y_bound = y_bound.max(cube.y);
                    z_bound = z_bound.max(cube.z);

                    cube
                })
                .collect(),
            x_bound: x_bound + 1,
            y_bound: y_bound + 1,
            z_bound: z_bound + 1,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn neighbors(&self) -> HashSet<Point> {
        [(0, 0, 1), (0, 1, 0), (1, 0, 0)]
            .into_iter()
            .flat_map(|(dx, dy, dz)| [(dx, dy, dz), (-dx, -dy, -dz)])
            .filter_map(|(dx, dy, dz)| {
                if let (Some(x), Some(y), Some(z)) = (
                    self.x.checked_add_signed(dx),
                    self.y.checked_add_signed(dy),
                    self.z.checked_add_signed(dz),
                ) {
                    Some(Self { x, y, z })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        // Offset points by 1, to allow for edges against the 'border' (0)
        let mut nums = value.split(',').map(|n| n.parse::<usize>().unwrap() + 1);

        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
        }
    }
}

impl From<(usize, usize, usize)> for Point {
    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Self { x, y, z }
    }
}

pub struct Day18;
impl Day for Day18 {
    type Input = Droplet;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        input
            .iter()
            .map(|cube| {
                let neighbors = cube
                    .neighbors()
                    .into_iter()
                    .filter(|c| input.contains(c))
                    .count();

                6 - neighbors
            })
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut visited = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(Point::from((0, 0, 0)));

        let mut touched = 0;

        while let Some(p) = frontier.pop_back() {
            visited.insert(p.clone());

            let (air, mass) = p.neighbors().into_iter().fold(
                (HashSet::new(), HashSet::new()),
                |(mut air, mut mass), n| {
                    if input.contains(&n) {
                        mass.insert(n);
                    } else {
                        air.insert(n);
                    }

                    (air, mass)
                },
            );

            touched += mass.len();

            let air: HashSet<Point> = air
                .into_iter()
                .filter(|n| {
                    !visited.contains(n)
                        && !frontier.contains(n)
                        && n.x <= input.x_bound
                        && n.y <= input.y_bound
                        && n.z <= input.z_bound
                })
                .collect();

            frontier.extend(air);
        }

        touched
    }

    fn parse(raw: &str) -> Self::Input {
        raw.into()
    }
}

#[test]
fn test_single() {
    assert_eq!(Day18::run("1,1,1"), (6, 6));
}
#[test]
fn test_small() {
    let input = "1,1,1
2,1,1";

    assert_eq!(Day18::run(input), (10, 10));
}
#[test]
fn test_large() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    assert_eq!(Day18::run(input), (64, 58));
}
#[test]
fn test_larger() {
    let input = "1,1,1
1,1,2
1,1,3
1,2,1
1,2,2
1,2,3
1,3,1
1,3,2
1,3,3
2,1,1
2,1,2
2,1,3
2,2,1
2,2,3
2,3,1
2,3,2
2,3,3
3,1,1
3,1,2
3,1,3
3,2,1
3,2,3
3,3,1
3,3,2
3,3,3
4,1,1
4,1,2
4,1,3
4,2,1
4,2,2
4,2,3
4,3,1
4,3,2
4,3,3
";

    assert_eq!(Day18::run(input), (76, 66));
}
