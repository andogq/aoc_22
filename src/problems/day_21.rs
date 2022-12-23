use crate::day::Day;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
pub enum Monkey {
    Number(usize),
    Operation(Operation),
}

#[derive(Clone, Debug)]
pub struct Operation {
    lhs: String,
    rhs: String,
    operation: Op,
}

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl From<char> for Op {
    fn from(value: char) -> Self {
        use Op::*;
        match value {
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Polynomial {
    parts: HashMap<isize, f64>,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial {
            parts: HashMap::new(),
        }
    }

    pub fn derive(&self) -> Self {
        let mut derivative = Self::new();

        for (&n, &x) in {
            let mut powers = self.parts.iter().collect::<Vec<_>>();
            powers.sort_by_key(|(&p, _)| p);

            powers
        } {
            if n == 0 {
                continue;
            }

            let next_n = n - 1;
            let next_x = x * (n as f64);
            derivative.parts.insert(next_n, next_x);
        }

        derivative
    }

    pub fn solve(&self, x: f64) -> f64 {
        self.parts
            .iter()
            .fold(0f64, |amount, (&n, b)| amount + (x.powf(n as f64) * b))
    }

    pub fn roots(&self) -> Option<f64> {
        const MAX_ITERATIONS: usize = 10;
        const EPSILON: f64 = 0.000001;

        let derivative = self.derive();
        let mut x0: f64 = 1.0;

        for _ in 0..MAX_ITERATIONS {
            let y = self.solve(x0);
            let y_prime = derivative.solve(x0);

            if y_prime.abs() < EPSILON {
                break;
            }

            let x1 = x0 - y / y_prime;

            if (x1 - x0).abs() <= EPSILON {
                return Some(x1);
            }

            x0 = x1;
        }

        None
    }
}

impl Add<&Self> for Polynomial {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        for (&power, n) in rhs.parts.iter() {
            *self.parts.entry(power).or_default() += n;
        }

        self
    }
}

impl Sub<&Self> for Polynomial {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        for (&power, n) in rhs.parts.iter() {
            *self.parts.entry(power).or_default() -= n;
        }

        self
    }
}

impl Mul<&Self> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut output = Polynomial::new();

        for (power, n) in &self.parts {
            for (sub_power, sub_n) in &rhs.parts {
                *output.parts.entry(power + sub_power).or_default() += n * sub_n;
            }
        }

        // for (power, n) in &rhs.parts {
        //     for (sub_power, sub_n) in &self.parts {
        //         *output.parts.entry(power + sub_power).or_default() += n * sub_n;
        //     }
        // }

        output
    }
}

impl Div<&Self> for Polynomial {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        let mut output = Polynomial::new();

        for (power, n) in &self.parts {
            for (sub_power, sub_n) in &rhs.parts {
                *output.parts.entry(power - sub_power).or_default() += n / sub_n;
            }
        }

        // for (power, n) in &rhs.parts {
        //     for (sub_power, sub_n) in &self.parts {
        //         *output.parts.entry(power + sub_power).or_default() += n / sub_n;
        //     }
        // }

        output
    }
}

impl Debug for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = self.parts.iter().collect::<Vec<_>>();
        parts.sort_by_key(|(&p, _)| p);
        parts.reverse();

        write!(
            f,
            "{}",
            parts
                .into_iter()
                .map(|(p, x)| {
                    match p {
                        0 => format!("{x}"),
                        1 => format!("{x}x"),
                        p => format!("{x}x^{p}"),
                    }
                })
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

pub struct Day21;
impl Day for Day21 {
    type Input = Vec<(String, Monkey)>;
    type Output = usize;

    fn part_1(monkeys: Self::Input) -> Self::Output {
        let mut known = HashMap::new();
        let mut queue = VecDeque::new();

        for (id, monkey) in monkeys {
            match monkey {
                Monkey::Number(n) => {
                    known.insert(id, n);
                }
                Monkey::Operation(op) => {
                    queue.push_back((id, op));
                }
            }
        }

        while let Some((id, op)) = queue.pop_front() {
            // Attempt to find lhs and rhs
            if let (Some(lhs), Some(rhs)) = (known.get(&op.lhs), known.get(&op.rhs)) {
                known.insert(
                    id,
                    match op.operation {
                        Op::Add => lhs + rhs,
                        Op::Sub => lhs - rhs,
                        Op::Mul => lhs * rhs,
                        Op::Div => lhs / rhs,
                    },
                );
            } else {
                // Add to end to solve later
                queue.push_back((id, op));
            }
        }

        *known.get("root").unwrap()
    }

    fn part_2(monkeys: Self::Input) -> Self::Output {
        let mut known: HashMap<String, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        for (id, monkey) in monkeys {
            if id == "humn" {
                // Don't insert the human at all
                continue;
            }

            match monkey {
                Monkey::Number(n) => {
                    known.insert(id, n);
                }
                Monkey::Operation(op) => {
                    if id == "root" {
                        // Add root value
                        known.insert(id.clone(), 0);

                        // Add relation between root operands
                        queue.push_back((
                            op.rhs.clone(),
                            Operation {
                                lhs: op.lhs.clone(),
                                operation: Op::Sub,
                                rhs: id.clone(),
                            },
                        ));
                        queue.push_back((
                            op.lhs,
                            Operation {
                                lhs: op.rhs,
                                operation: Op::Sub,
                                rhs: id,
                            },
                        ));
                    } else {
                        queue.push_back((id.clone(), op.clone()));
                        match op.operation {
                            Op::Add => [
                                (
                                    op.lhs.clone(),
                                    Operation {
                                        lhs: id.clone(),
                                        rhs: op.rhs.clone(),
                                        operation: Op::Sub,
                                    },
                                ),
                                (
                                    op.rhs,
                                    Operation {
                                        lhs: id,
                                        rhs: op.lhs,
                                        operation: Op::Sub,
                                    },
                                ),
                            ],
                            Op::Sub => [
                                (
                                    op.lhs.clone(),
                                    Operation {
                                        lhs: id.clone(),
                                        rhs: op.rhs.clone(),
                                        operation: Op::Add,
                                    },
                                ),
                                (
                                    op.rhs,
                                    Operation {
                                        lhs: op.lhs,
                                        rhs: id,
                                        operation: Op::Sub,
                                    },
                                ),
                            ],
                            Op::Mul => [
                                (
                                    op.lhs.clone(),
                                    Operation {
                                        lhs: id.clone(),
                                        rhs: op.rhs.clone(),
                                        operation: Op::Div,
                                    },
                                ),
                                (
                                    op.rhs,
                                    Operation {
                                        lhs: id,
                                        rhs: op.lhs,
                                        operation: Op::Div,
                                    },
                                ),
                            ],
                            Op::Div => [
                                (
                                    op.lhs.clone(),
                                    Operation {
                                        lhs: id.clone(),
                                        rhs: op.rhs.clone(),
                                        operation: Op::Mul,
                                    },
                                ),
                                (
                                    op.rhs,
                                    Operation {
                                        lhs: op.lhs,
                                        rhs: id,
                                        operation: Op::Div,
                                    },
                                ),
                            ],
                        }
                        .into_iter()
                        .for_each(|o| queue.push_back(o));
                    }
                }
            }
        }

        while let Some((id, op)) = queue.pop_front() {
            if let (Some(lhs), Some(rhs)) = (known.get(&op.lhs), known.get(&op.rhs)) {
                dbg!(&id, &lhs, &rhs, &op);
                let result = match op.operation {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                if id == "humn" {
                    return result;
                }

                known.insert(id, result);
            } else {
                queue.push_back(dbg!(id, op));
            }
        }

        *known.get("humn").unwrap()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|line| {
                let (id, rhs) = line.split_once(": ").unwrap();

                let rhs = rhs.split_whitespace().collect::<Vec<_>>();

                (
                    id.to_string(),
                    if rhs.len() == 3 {
                        // Operation
                        Monkey::Operation(Operation {
                            lhs: rhs[0].to_string(),
                            rhs: rhs[2].to_string(),
                            operation: Op::from(rhs[1].chars().next().unwrap()),
                        })
                    } else {
                        // Number
                        Monkey::Number(rhs[0].parse().unwrap())
                    },
                )
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    assert_eq!(Day21::run(input), (152, 301));
}
