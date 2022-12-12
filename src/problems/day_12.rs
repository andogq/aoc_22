use std::collections::VecDeque;

use crate::day::Day;

type Position = (usize, usize);
type Map = Vec<Vec<usize>>;

const START_ELEVATION: usize = 0;
const END_ELEVATION: usize = 25;

fn find_shortest_path(points: &[Position], map: &Map, end: Position) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    for p in points {
        queue.push_back((p.to_owned(), 0));

        visited[p.1][p.0] = true;
    }

    while let Some((pos, distance)) = queue.pop_front() {
        if pos == end {
            return distance;
        }

        let current_elevation = map[pos.1][pos.0];

        // Get neighbors
        queue.extend(
            [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .into_iter()
                .filter_map(|(dx, dy): (isize, isize)| {
                    if let (Some(x), Some(y)) =
                        (pos.0.checked_add_signed(dx), pos.1.checked_add_signed(dy))
                    {
                        let next_pos = (x, y);
                        if next_pos.1 < map.len()
                            && next_pos.0 < map[0].len()
                            && !visited[next_pos.1][next_pos.0]
                        {
                            let next_elevation = map[next_pos.1][next_pos.0];

                            // Make sure height difference is valid
                            if current_elevation + 1 >= next_elevation {
                                visited[next_pos.1][next_pos.0] = true;
                                return Some(next_pos);
                            }
                        }
                    }
                    None
                })
                .map(|pos| (pos, distance + 1)),
        );
    }

    unreachable!();
}

pub struct Day12;
impl Day for Day12 {
    type Input = (Position, Position, Map);
    type Output = usize;

    fn part_1((start, end, map): Self::Input) -> Self::Output {
        find_shortest_path(&[start], &map, end)
    }

    fn part_2((_, end, map): Self::Input) -> Self::Output {
        // Find all grid locations of elevation 0
        find_shortest_path(
            &map.iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, &c)| c == START_ELEVATION)
                        .map(move |(x, _)| (x, y))
                })
                .collect::<Vec<_>>(),
            &map,
            end,
        )
    }

    fn parse(raw: &str) -> Self::Input {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let map = raw
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            START_ELEVATION
                        }
                        'E' => {
                            end = (x, y);
                            END_ELEVATION
                        }
                        _ => c as usize - 'a' as usize,
                    })
                    .collect()
            })
            .collect();

        (start, end, map)
    }
}

#[test]
fn test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    assert_eq!(Day12::run(input), (31, 29));
}
