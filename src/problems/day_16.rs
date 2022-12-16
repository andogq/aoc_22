use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::day::Day;

pub struct Day16;

type Identifier = usize;
type OpenedHash = u64;

#[derive(Clone, Debug)]
pub struct Valve {
    identifier: String,
    flow_rate: usize,
    connections: HashSet<Identifier>,
}

#[derive(Clone, Debug)]
pub struct CaveSystem {
    valves: HashMap<Identifier, Valve>,
    distance_cache: RefCell<HashMap<(Identifier, Identifier), usize>>,
    flow_cache: RefCell<HashMap<(Identifier, usize, OpenedHash), usize>>,
    flow_2_cache: RefCell<HashMap<(Identifier, usize, Identifier, usize, OpenedHash), usize>>,
}

impl CaveSystem {
    fn new(valves: HashMap<Identifier, Valve>) -> Self {
        Self {
            valves,
            distance_cache: RefCell::new(HashMap::new()),
            flow_cache: RefCell::new(HashMap::new()),
            flow_2_cache: RefCell::new(HashMap::new()),
        }
    }

    fn distance(&self, x: Identifier, y: Identifier) -> usize {
        let d = self
            .distance_cache
            .borrow()
            .get(&{
                if x < y {
                    (x, y)
                } else {
                    (y, x)
                }
            })
            .map(|d| d.to_owned());

        d.unwrap_or_else(|| {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((0, x));

            while let Some((d, valve_identifier)) = queue.pop_front() {
                if valve_identifier == y {
                    // Insert into cache
                    self.distance_cache.borrow_mut().insert((x, y), d);
                    return d;
                }

                visited.insert(valve_identifier);

                let valve = self.valves.get(&valve_identifier).unwrap();
                queue.extend(
                    valve
                        .connections
                        .iter()
                        .filter(|c| !visited.contains(*c))
                        .cloned()
                        .map(|c| (d + 1, c)),
                );
            }
            unreachable!("Should be a continuous graph");
        })
    }

    fn flow(&self, x: Identifier, t: usize, opened: OpenedHash) -> usize {
        let flow = self
            .flow_cache
            .borrow()
            .get(&(x, t, opened))
            .map(|f| f.to_owned());

        flow.unwrap_or_else(|| {
            let valve = self.valves.get(&x).unwrap();

            let flow = usize::max(
                if (opened & (1 << x)) == 0 && valve.flow_rate > 0 {
                    // Flow on
                    let valve_flow = valve.flow_rate * (t - 1);
                    let opened = opened | (1 << x);

                    valve_flow + self.flow(x, t - 1, opened)
                } else {
                    0
                },
                {
                    // Flow off
                    valve
                        .connections
                        .iter()
                        .filter_map(|&y| {
                            let d = self.distance(x, y);

                            if t > d {
                                Some(self.flow(y, t - d, opened))
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap_or(0)
                },
            );

            self.flow_cache.borrow_mut().insert((x, t, opened), flow);

            flow
        })
    }

    fn flow_2(
        &self,
        x1: Identifier,
        t1: usize,
        x2: Identifier,
        t2: usize,
        opened: OpenedHash,
    ) -> usize {
        let (x1, t1, x2, t2) = if x1 < x2 {
            (x1, t1, x2, t2)
        } else {
            (x2, t2, x1, t1)
        };

        let flow = self
            .flow_2_cache
            .borrow()
            .get(&(x1, t1, x2, t2, opened))
            .map(|f| f.to_owned());

        flow.unwrap_or_else(|| {
            let valve_1 = self.valves.get(&x1).unwrap();
            let valve_2 = self.valves.get(&x2).unwrap();

            let can_open =
                |v| (opened & (1u64 << v) == 0) && self.valves.get(&v).unwrap().flow_rate > 0;
            let open = |v, opened| opened | (1u64 << v);
            let flow_rate = |v: &Valve, t| v.flow_rate * (t - 1);

            let flow = [
                if x1 != x2 && can_open(x1) && can_open(x2) {
                    // open/open
                    let opened = open(x1, open(x2, opened));

                    flow_rate(valve_1, t1)
                        + flow_rate(valve_2, t2)
                        + self.flow_2(x1, t1 - 1, x2, t2 - 1, opened)
                } else {
                    0
                },
                if can_open(x1) {
                    // open/step
                    let opened = open(x1, opened);

                    flow_rate(valve_1, t1)
                        + valve_2
                            .connections
                            .iter()
                            .filter_map(|&c| {
                                let d = self.distance(x2, c);

                                if t2 > d {
                                    Some(self.flow_2(x1, t1 - 1, c, t2 - d, opened))
                                } else {
                                    None
                                }
                            })
                            .max()
                            .unwrap_or(0)
                } else {
                    0
                },
                if can_open(x2) {
                    // step/open
                    let opened = open(x2, opened);

                    flow_rate(valve_2, t2)
                        + valve_1
                            .connections
                            .iter()
                            .filter_map(|&c| {
                                let d = self.distance(x1, c);

                                if t1 > d {
                                    Some(self.flow_2(c, t1 - d, x2, t2 - 1, opened))
                                } else {
                                    None
                                }
                            })
                            .max()
                            .unwrap_or(0)
                } else {
                    0
                },
                {
                    // step/step

                    // Generate every possible permutation of steps
                    valve_1
                        .connections
                        .iter()
                        .filter_map(|&c1| {
                            let d1 = self.distance(x1, c1);

                            if t1 > d1 {
                                Some(valve_2.connections.iter().filter_map(move |&c2| {
                                    let d2 = self.distance(x2, c2);

                                    if t2 > d2 {
                                        Some(self.flow_2(c1, t1 - d1, c2, t2 - d2, opened))
                                    } else {
                                        None
                                    }
                                }))
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .max()
                        .unwrap_or(0)
                },
            ]
            .into_iter()
            .max()
            .unwrap();

            self.flow_2_cache
                .borrow_mut()
                .insert((x1, t1, x2, t2, opened), flow);

            flow
        })
    }
}

impl Day for Day16 {
    type Input = CaveSystem;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        let start = input
            .valves
            .iter()
            .find(|(_, v)| v.identifier == "AA")
            .unwrap()
            .0
            .to_owned();

        input.flow(start, 30, 0)
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let start = input
            .valves
            .iter()
            .find(|(_, v)| v.identifier == "AA")
            .unwrap()
            .0
            .to_owned();

        input.flow_2(start, 26, start, 26, 0)
    }

    fn parse(raw: &str) -> Self::Input {
        let mut identifier_map = HashMap::new();
        let mut next_identifier: usize = 0;

        let mut get_identifier = |s: &str| -> usize {
            identifier_map
                .entry(s.to_string())
                .or_insert_with(|| {
                    let id = next_identifier;
                    next_identifier += 1;
                    id
                })
                .to_owned()
        };

        CaveSystem::new(
            raw.lines()
                .map(|line| {
                    let mut line = line.split_whitespace();
                    let identifier_str = line.nth(1).unwrap();
                    let identifier = get_identifier(identifier_str);
                    let flow_rate = line
                        .nth(2)
                        .unwrap()
                        .trim_end_matches(';')
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .parse()
                        .unwrap();

                    let connections = line
                        .skip(4)
                        .map(|connection| get_identifier(connection.trim_end_matches(',')))
                        .collect();

                    (
                        identifier,
                        Valve {
                            identifier: identifier_str.to_string(),
                            flow_rate,
                            connections,
                        },
                    )
                })
                .collect(),
        )
    }
}

#[test]
fn test() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    assert_eq!(Day16::run(input), (1651, 1707));
}
