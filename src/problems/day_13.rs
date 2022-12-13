use std::cmp::Ordering;

use crate::day::Day;

#[derive(Clone)]
pub enum PacketPart {
    Integer(usize),
    List(Vec<PacketPart>),
}

fn parse_packet_part(raw: &[char]) -> Vec<PacketPart> {
    let mut packet = Vec::new();

    let mut chars = raw[1..].iter();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut n = vec![*c];
                n.extend(chars.clone().take_while(|c| c.is_ascii_digit()).to_owned());

                packet.push(PacketPart::Integer(
                    n.iter().collect::<String>().parse().unwrap(),
                ));
            }
            '[' => {
                // Starting new packet part
                let mut part_raw = vec![*c];
                let mut depth = 0;

                loop {
                    let next_c = *chars.next().unwrap();
                    part_raw.push(next_c);

                    if next_c == '[' {
                        depth += 1;
                    } else if next_c == ']' {
                        if depth == 0 {
                            // End of packet part
                            packet.push(PacketPart::List(parse_packet_part(&part_raw)));
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

    packet
}

fn compare(left: &PacketPart, right: &PacketPart) -> Ordering {
    match (left, right) {
        (PacketPart::Integer(left), PacketPart::Integer(right)) => left.cmp(right),
        (PacketPart::List(ref left), PacketPart::List(ref right)) => {
            // Compare items manually
            let mut left = left.iter();
            let mut right = right.iter();
            loop {
                let l = left.next();
                let r = right.next();

                match (l, r) {
                    (Some(l), Some(r)) => {
                        // Compare items, and short circuit if out of order
                        match compare(l, r) {
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
        (PacketPart::Integer(left), PacketPart::List(right)) => compare(
            &PacketPart::List(vec![PacketPart::Integer(*left)]),
            &PacketPart::List(right.to_vec()),
        ),
        (PacketPart::List(left), PacketPart::Integer(right)) => compare(
            &PacketPart::List(left.to_vec()),
            &PacketPart::List(vec![PacketPart::Integer(*right)]),
        ),
    }
}

pub struct Day13;
impl Day for Day13 {
    type Input = Vec<(PacketPart, PacketPart)>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .enumerate()
            .filter(|(_, group)| matches!(compare(&group.0, &group.1), Ordering::Less))
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut ordered = input
            .into_iter()
            .flat_map(|(left, right)| [left, right])
            .chain([
                PacketPart::List(parse_packet_part(
                    &"[[2]]".chars().to_owned().collect::<Vec<_>>(),
                )),
                PacketPart::List(parse_packet_part(
                    &"[[6]]".chars().to_owned().collect::<Vec<_>>(),
                )),
            ])
            .collect::<Vec<_>>();

        ordered.sort_unstable_by(compare);

        ordered
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if let PacketPart::List(l) = p {
                    if l.len() == 1 {
                        if let PacketPart::List(ref j) = l[0] {
                            if j.len() == 1 {
                                if let PacketPart::Integer(j) = j[0] {
                                    if j == 2 || j == 6 {
                                        return Some(i + 1);
                                    }
                                }
                            }
                        }
                    }
                }
                None
            })
            .product()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.split("\n\n")
            .map(|raw_group| {
                let mut groups = raw_group.lines().map(|packet| {
                    parse_packet_part(&packet.chars().to_owned().collect::<Vec<_>>())
                });

                (
                    PacketPart::List(groups.next().unwrap()),
                    PacketPart::List(groups.next().unwrap()),
                )
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
