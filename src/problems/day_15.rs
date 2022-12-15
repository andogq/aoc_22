use std::collections::HashSet;

use crate::day::Day;

type Position = (isize, isize);

const MULTIPLIER: usize = 4000000;

#[derive(Clone, Debug)]
pub struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    pub fn known_distance(&self) -> usize {
        (self.position.0.abs_diff(self.beacon.0)) + (self.position.1.abs_diff(self.beacon.1))
    }
}

pub struct Day15;
impl Day for Day15 {
    type Input = (usize, usize, Vec<Sensor>);
    type Output = usize;

    fn part_1((goal_row, _, sensors): Self::Input) -> Self::Output {
        sensors
            .iter()
            .fold(HashSet::new(), |mut squares, sensor| {
                let y_spread = sensor.position.1.abs_diff(goal_row as isize);
                let d = sensor.known_distance();

                if y_spread <= d {
                    let x_spread = (d - y_spread) as isize;

                    squares.extend(sensor.position.0 - x_spread..=sensor.position.0 + x_spread);
                }

                squares
            })
            .len()
            - sensors
                .iter()
                .filter(|s| s.beacon.1 == goal_row as isize)
                .map(|s| s.beacon.0)
                .collect::<HashSet<_>>()
                .len()
    }

    fn part_2((_, search_max, sensors): Self::Input) -> Self::Output {
        for search_row in 0isize..=search_max as isize {
            let row_squares = sensors
                .iter()
                .try_fold(None::<HashSet<isize>>, |mut row_squares, sensor| {
                    let y_spread = sensor.position.1.abs_diff(search_row);
                    let d = sensor.known_distance();

                    if y_spread <= d {
                        let x_spread = (d - y_spread) as isize;

                        let sensor_squares = (0..sensor.position.0 - x_spread)
                            .chain(sensor.position.0 + x_spread + 1..search_max as isize + 1)
                            .collect::<HashSet<_>>();

                        row_squares = Some(if let Some(row_squares) = row_squares {
                            if row_squares.is_empty() {
                                return None;
                            }

                            row_squares.intersection(&sensor_squares).cloned().collect()
                        } else {
                            sensor_squares
                        });
                    }

                    Some(row_squares)
                })
            .map(|s| s.unwrap());

            if let Some(row_squares) = row_squares {
                match row_squares.len() {
                    0 => (),
                    1 => {
                        let x = row_squares.into_iter().next().unwrap();

                        return (x as usize * MULTIPLIER) + search_row as usize;
                    }
                    _ => {
                        println!("Found row with {} available spaces", row_squares.len());
                    }
                }
            }
        }

        0
    }

    fn parse(raw: &str) -> Self::Input {
        let mut raw = raw.lines();

        (
            raw.next().unwrap().parse().unwrap(),
            raw.next().unwrap().parse().unwrap(),
            raw.map(|raw| {
                let (sensor, beacon) = raw.split_once(':').unwrap();
                let (position, beacon) = {
                    let mut iter = [sensor, beacon].into_iter().map(|section| {
                        let mut parts = section.split(' ').rev().take(2).map(|part| {
                            part.split_once('=')
                                .unwrap()
                                .1
                                .replace(',', "")
                                .parse()
                                .unwrap()
                        });

                        // Reversed
                        let y = parts.next().unwrap();
                        let x = parts.next().unwrap();

                        (x, y)
                    });
                    (iter.next().unwrap(), iter.next().unwrap())
                };

                Sensor { position, beacon }
            })
            .collect(),
        )
    }
}

#[test]
fn test() {
    let input = "10
20
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(Day15::run(input), (26, 56000011));
}
