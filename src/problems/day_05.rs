use std::str::SplitWhitespace;

use crate::day::Day;

#[derive(Clone)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

// index 0 is the top of the stack
type Stack = Vec<char>;

pub trait StackMover {
    fn pick_up(stack: &mut Stack, amount: usize) -> Stack;
}

#[derive(Clone)]
pub struct Stacks(Vec<Stack>);
impl Stacks {
    pub fn new(size: usize) -> Self {
        Self(vec![Vec::new(); size])
    }

    pub fn execute<M>(&mut self, moves: &[Move])
    where
        M: StackMover,
    {
        for m in moves {
            let mut new_stack = M::pick_up(&mut self.0[m.from], m.amount);
            new_stack.append(&mut self.0[m.to]);
            self.0[m.to] = new_stack;
        }
    }

    pub fn get_tops(&self) -> String {
        self.0.iter().map(|stack| stack[0]).collect()
    }
}

pub struct Day05;
impl Day for Day05 {
    type Input = (Stacks, Vec<Move>);

    type Output = String;

    fn part_1((mut stacks, moves): Self::Input) -> Self::Output {
        struct StackMover3000;
        impl StackMover for StackMover3000 {
            fn pick_up(stack: &mut Stack, amount: usize) -> Stack {
                stack.drain(..amount).rev().collect::<Vec<_>>()
            }
        }

        stacks.execute::<StackMover3000>(&moves);
        stacks.get_tops()
    }

    fn part_2((mut stacks, moves): Self::Input) -> Self::Output {
        struct StackMover3001;
        impl StackMover for StackMover3001 {
            fn pick_up(stack: &mut Stack, amount: usize) -> Stack {
                stack.drain(..amount).collect::<Vec<_>>()
            }
        }

        stacks.execute::<StackMover3001>(&moves);
        stacks.get_tops()
    }

    fn parse(raw: &str) -> Self::Input {
        let (stacks, moves) = raw.split_once("\n\n").unwrap();

        // Parse stacks
        let stacks = {
            let mut stacks_raw = stacks.lines().rev();
            let stack_count = stacks_raw
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            stacks_raw.fold(Stacks::new(stack_count), |mut stacks, line| {
                let mut line = line.chars();
                for stack in stacks.0.iter_mut() {
                    let c = line.nth(1).unwrap();

                    if !c.is_whitespace() {
                        stack.insert(0, c);
                    }

                    // Skip closing bracket, and next space
                    line.next();
                    line.next();
                }
                stacks
            })
        };

        // Parse moves
        let moves = moves
            .lines()
            .map(|m| {
                let mut words = m.split_whitespace();

                // nth consumes the previous elements
                let get_num =
                    |words: &mut SplitWhitespace| words.nth(1).unwrap().parse::<usize>().unwrap();

                Move {
                    amount: get_num(&mut words),
                    from: get_num(&mut words) - 1,
                    to: get_num(&mut words) - 1,
                }
            })
            .collect();

        (stacks, moves)
    }
}

#[test]
fn test() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!(Day05::run(input), ("CMZ".to_string(), "MCD".to_string()));
}
