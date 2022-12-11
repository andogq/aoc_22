use std::collections::HashMap;

use crate::day::Day;

const WORRY_REDUCE: usize = 3;

#[derive(Clone)]
pub enum Operation {
    Mult(usize),
    Add(usize),
    Square,
}

#[derive(Clone)]
pub struct Monkey {
    pub operation: Operation,
    pub divisor: usize,
    pub next_monkey_true: usize,
    pub next_monkey_false: usize,
    pub inspected_count: usize,
}

impl Monkey {
    pub fn inspect(&mut self, item: &u64) -> u64 {
        self.inspected_count += 1;

        match self.operation {
            Operation::Square => item.pow(2),
            Operation::Add(amount) => item + amount as u64,
            Operation::Mult(amount) => item * amount as u64,
        }
    }

    pub fn test(&self, item: u64) -> usize {
        if item % self.divisor as u64 == 0 {
            self.next_monkey_true
        } else {
            self.next_monkey_false
        }
    }
}

pub struct Day11;
impl Day for Day11 {
    type Input = (Vec<Monkey>, HashMap<usize, Vec<u64>>);
    type Output = usize;

    fn part_1((mut monkeys, mut items): Self::Input) -> Self::Output {
        for _ in 0..20 {
            for (i, monkey) in monkeys.iter_mut().enumerate() {
                // TODO: Need a better way to empty an iterator
                let monkey_items = items.get_mut(&i).unwrap().drain(0..).collect::<Vec<_>>();

                for item in monkey_items {
                    // Inspect the item
                    let mut new_item = monkey.inspect(&item);

                    // Reduce worry
                    new_item /= WORRY_REDUCE as u64;

                    // Test item
                    let next_monkey = monkey.test(new_item);

                    // Throw item
                    items.entry(next_monkey).or_default().push(new_item);
                }
            }
        }

        monkeys.sort_unstable_by_key(|monkey| monkey.inspected_count);

        monkeys
            .into_iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.inspected_count)
            .product()
    }

    fn part_2((mut monkeys, mut items): Self::Input) -> Self::Output {
        // Find common divisor
        let div: u64 = monkeys.iter().map(|m| m.divisor as u64).product();

        for _ in 0..10000 {
            for (i, monkey) in monkeys.iter_mut().enumerate() {
                // TODO: Need a better way to empty an iterator
                let monkey_items = items.get_mut(&i).unwrap().drain(0..).collect::<Vec<_>>();

                for item in monkey_items {
                    // Inspect the item
                    let new_item = monkey.inspect(&item) % div;

                    // Test item
                    let next_monkey = monkey.test(new_item);

                    // Throw item
                    items.entry(next_monkey).or_default().push(new_item);
                }
            }
        }

        monkeys.sort_unstable_by_key(|monkey| monkey.inspected_count);

        monkeys
            .into_iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.inspected_count)
            .product()
    }

    fn parse(raw: &str) -> Self::Input {
        let mut items = HashMap::new();

        let monkeys = raw
            .split("\n\n")
            .enumerate()
            .map(|(i, raw_monkey)| {
                let mut raw_monkey = raw_monkey.lines().skip(1);

                items.insert(
                    i,
                    raw_monkey
                        .next()
                        .unwrap()
                        .split_once(": ")
                        .map(|(_, rhs)| rhs.split(", "))
                        .unwrap()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                );

                let operation = raw_monkey
                    .next()
                    .unwrap()
                    .split_once(" = ")
                    .map(|(_, rhs)| {
                        let mut parts = rhs.split_whitespace().skip(1);
                        let sign = parts.next().unwrap();
                        let amount = parts.next().unwrap();

                        if amount == "old" {
                            Operation::Square
                        } else {
                            let amount = amount.parse().unwrap();

                            match sign {
                                "+" => Operation::Add(amount),
                                "*" => Operation::Mult(amount),
                                _ => unreachable!(),
                            }
                        }
                    })
                    .unwrap();

                let divisor = raw_monkey
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut next_monkeys = raw_monkey
                    .take(2)
                    .map(|raw| raw.split_whitespace().last().unwrap().parse().unwrap());

                Monkey {
                    inspected_count: 0,
                    operation,
                    divisor,
                    next_monkey_true: next_monkeys.next().unwrap(),
                    next_monkey_false: next_monkeys.next().unwrap(),
                }
            })
            .collect();

        (monkeys, items)
    }
}

#[test]
fn test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    assert_eq!(Day11::run(input), (10605, 2713310158));
}
