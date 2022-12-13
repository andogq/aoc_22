use std::cmp::Ordering;

use crate::day::Day;

#[derive(Clone)]
pub struct PacketList {
    list: Vec<Packet>,
    divider_packet: bool,
}

#[derive(Clone)]
pub enum Packet {
    Integer(usize),
    List(PacketList),
}

impl Eq for Packet {}

impl Packet {
    pub fn divider_packet(signal: usize) -> Self {
        Self::List(PacketList {
            list: vec![vec![Packet::Integer(signal)].into()],
            divider_packet: true,
        })
    }

    pub fn as_list(&self) -> Self {
        match self {
            Packet::Integer(integer) => vec![Packet::Integer(*integer)].into(),
            // TODO: not super happy with this clone
            list => list.clone(),
        }
    }
}

impl From<Vec<Packet>> for Packet {
    fn from(v: Vec<Packet>) -> Self {
        Self::List(PacketList {
            list: v,
            divider_packet: false,
        })
    }
}

impl From<&str> for Packet {
    fn from(raw: &str) -> Self {
        if raw.chars().all(|c| c.is_ascii_digit()) {
            Self::Integer(raw.parse().unwrap())
        } else {
            let mut packet = Vec::new();

            let mut chars = raw.chars();

            // Skip the first open bracket
            chars.next();

            while let Some(c) = chars.next() {
                match c {
                    '0'..='9' => {
                        let mut n = vec![c];
                        n.extend(chars.clone().take_while(|c| c.is_ascii_digit()).to_owned());

                        packet.push(Packet::Integer(
                            n.iter().collect::<String>().parse().unwrap(),
                        ));
                    }
                    '[' => {
                        // Starting new packet part
                        let mut part_raw = c.to_string();
                        let mut depth = 0;

                        loop {
                            let next_c = chars.next().unwrap();
                            part_raw.push(next_c);

                            if next_c == '[' {
                                depth += 1;
                            } else if next_c == ']' {
                                if depth == 0 {
                                    // End of packet part
                                    packet.push(part_raw.as_str().into());
                                    break;
                                } else {
                                    depth -= 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            packet.into()
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.cmp(right),
            (Packet::List(ref left), Packet::List(ref right)) => {
                // Compare items manually
                let mut left = left.list.iter();
                let mut right = right.list.iter();
                loop {
                    let l = left.next();
                    let r = right.next();

                    match (l, r) {
                        (Some(l), Some(r)) => {
                            // Compare items, and short circuit if out of order
                            match l.cmp(r) {
                                Ordering::Equal => continue,
                                o => return o,
                            }
                        }
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (left, right) => left.as_list().cmp(&right.as_list()),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day13;
impl Day for Day13 {
    type Input = Vec<(Packet, Packet)>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .enumerate()
            .filter(|(_, group)| matches!(group.0.cmp(&group.1), Ordering::Less))
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut ordered = input
            .into_iter()
            .flat_map(|(left, right)| [left, right])
            .chain([Packet::divider_packet(2), Packet::divider_packet(6)])
            .collect::<Vec<_>>();

        ordered.sort_unstable();

        ordered
            .into_iter()
            .enumerate()
            .filter(|(_, p)| {
                matches!(
                    p,
                    Packet::List(PacketList {
                        divider_packet: true,
                        ..
                    })
                )
            })
            .map(|(i, _)| i + 1)
            .product()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.split("\n\n")
            .map(|raw_group| {
                let mut groups = raw_group.lines().map(|packet| packet.into());

                (groups.next().unwrap(), groups.next().unwrap())
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    assert_eq!(Day13::run(input), (13, 140));
}
