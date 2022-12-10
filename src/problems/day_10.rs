use crate::day::Day;

#[derive(Clone)]
pub enum Instruction {
    Add(isize),
    Nop,
}

pub struct Day10;
impl Day for Day10 {
    type Input = Vec<Instruction>;
    type Output = isize;

    fn part_1(input: Self::Input) -> Self::Output {
        let mut reg_x = 1;
        let mut signal_values = Vec::new();

        let mut cycle_count = 0;
        let mut tick = |reg_x| {
            cycle_count += 1;

            if (cycle_count - 20) % 40 == 0 {
                let signal = reg_x * cycle_count;
                signal_values.push(signal);
            }
        };

        for instruction in input {
            match instruction {
                Instruction::Add(amount) => {
                    tick(reg_x);
                    tick(reg_x);
                    reg_x += amount;
                }
                Instruction::Nop => {
                    tick(reg_x);
                }
            }
        }

        signal_values.into_iter().sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut reg_x: isize = 1;
        let mut signal_values = Vec::new();

        let mut cycle_count = 1;
        let mut line = Vec::with_capacity(40);

        let mut tick = |reg_x: isize| {
            cycle_count += 1;

            if cycle_count > 20 && (cycle_count - 20) % 40 == 0 {
                let signal = reg_x * (cycle_count as isize);
                signal_values.push(signal);
            }

            line.push({
                let crt_position = line.len() as isize;
                if crt_position <= (reg_x % 40) + 1
                    && crt_position >= (reg_x % 40).saturating_sub(1)
                {
                    '#'
                } else {
                    '.'
                }
            });

            if line.len() == 40 {
                println!("{}", line.iter().cloned().collect::<String>());
                line = Vec::with_capacity(40);
            }
        };

        for instruction in input {
            match instruction {
                Instruction::Add(amount) => {
                    tick(reg_x);
                    tick(reg_x);
                    reg_x += amount;
                }
                Instruction::Nop => {
                    tick(reg_x);
                }
            }
        }

        0
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|line| {
                if let Some((_, amount)) = line.split_once(' ') {
                    Instruction::Add(amount.parse().unwrap())
                } else {
                    Instruction::Nop
                }
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(Day10::run(input), (13140, 0));
}
