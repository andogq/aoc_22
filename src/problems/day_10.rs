use crate::day::Day;

#[derive(Clone)]
pub enum Instruction {
    Add(isize),
    Nop,
}

mod cpu {
    use super::Instruction;

    const CYCLE_OFFSET: usize = 20;
    const DISPLAY_WIDTH: usize = 40;

    pub struct CpuState {
        cycles: usize,
        reg_x: isize,
    }

    impl CpuState {
        pub fn new() -> Self {
            CpuState {
                cycles: 0,
                reg_x: 1,
            }
        }
    }

    pub trait Peripheral {
        fn tick(&mut self, state: &CpuState);
    }

    pub struct Cpu<'a> {
        state: CpuState,
        pub signal_values: Vec<isize>,
        peripherals: Vec<&'a mut dyn Peripheral>,
    }

    impl<'a> Cpu<'a> {
        pub fn new() -> Self {
            Self {
                state: CpuState::new(),
                signal_values: Vec::new(),
                peripherals: Vec::new(),
            }
        }

        pub fn with_peripheral(mut self, peripheral: &'a mut dyn Peripheral) -> Self {
            self.peripherals.push(peripheral);

            self
        }

        pub fn run(&mut self, instructions: &[Instruction]) {
            for instruction in instructions {
                self.tick();

                match instruction {
                    Instruction::Add(amount) => {
                        self.tick();

                        self.state.reg_x += amount;
                    }
                    Instruction::Nop => {}
                }
            }
        }

        pub fn tick(&mut self) {
            self.state.cycles += 1;

            if self.state.cycles >= CYCLE_OFFSET
                && (self.state.cycles - CYCLE_OFFSET) % DISPLAY_WIDTH == 0
            {
                self.signal_values
                    .push(self.state.reg_x * self.state.cycles as isize);
            }

            for p in self.peripherals.iter_mut() {
                p.tick(&self.state);
            }
        }
    }

    pub mod crt {
        use super::*;
        use std::fmt::Display;

        pub struct CrtScreen {
            screen: Vec<char>,
        }

        impl CrtScreen {
            pub fn new() -> Self {
                CrtScreen { screen: Vec::new() }
            }

            pub fn print(&mut self, c: char) {
                self.screen.push(c)
            }
        }

        impl Display for CrtScreen {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    self.screen
                        .chunks(40)
                        .flat_map(|line| [line, &['\n']].concat())
                        .collect::<String>()
                )
            }
        }

        impl Peripheral for CrtScreen {
            fn tick(&mut self, state: &CpuState) {
                self.print({
                    let crt_position = self.screen.len() as isize % 40;
                    if crt_position <= (state.reg_x % 40) + 1
                        && crt_position >= (state.reg_x % 40).saturating_sub(1)
                    {
                        '#'
                    } else {
                        '.'
                    }
                });
            }
        }
    }
}

pub struct Day10;
impl Day for Day10 {
    type Input = Vec<Instruction>;
    type Output = isize;

    fn part_1(input: Self::Input) -> Self::Output {
        let mut cpu = cpu::Cpu::new();
        cpu.run(&input);

        cpu.signal_values.into_iter().sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let mut screen = cpu::crt::CrtScreen::new();
        let mut cpu = cpu::Cpu::new().with_peripheral(&mut screen);
        cpu.run(&input);
        println!("{}", screen);

        // TODO: Parse generated output, and test against known string
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

    fn run(input: &str) -> (Self::Output, Self::Output) {
        let input = Self::parse(input);

        (Self::part_1(input.clone()), Self::part_2(input))
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
