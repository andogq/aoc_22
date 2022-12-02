use crate::day::Day;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn outcome(&self, other: &Self) -> Outcome {
        if self == other {
            Outcome::Draw
        } else {
            match (self, other) {
                (Choice::Rock, Choice::Scissors) => Outcome::Win,
                (Choice::Paper, Choice::Rock) => Outcome::Win,
                (Choice::Scissors, Choice::Paper) => Outcome::Win,
                _ => Outcome::Loss,
            }
        }
    }
}

#[derive(Clone)]
pub enum UnknownInput {
    X,
    Y,
    Z,
}
impl From<char> for UnknownInput {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::X,
            'B' | 'Y' => Self::Y,
            'C' | 'Z' => Self::Z,
            _ => unreachable!(),
        }
    }
}
impl From<UnknownInput> for Choice {
    fn from(c: UnknownInput) -> Self {
        match c {
            UnknownInput::X => Self::Rock,
            UnknownInput::Y => Self::Paper,
            UnknownInput::Z => Self::Scissors,
        }
    }
}
impl From<&UnknownInput> for Choice {
    fn from(c: &UnknownInput) -> Self {
        match c {
            UnknownInput::X => Self::Rock,
            UnknownInput::Y => Self::Paper,
            UnknownInput::Z => Self::Scissors,
        }
    }
}
impl From<UnknownInput> for Outcome {
    fn from(c: UnknownInput) -> Self {
        match c {
            UnknownInput::X => Self::Loss,
            UnknownInput::Y => Self::Draw,
            UnknownInput::Z => Self::Win,
        }
    }
}
impl From<&UnknownInput> for Outcome {
    fn from(c: &UnknownInput) -> Self {
        match c {
            UnknownInput::X => Self::Loss,
            UnknownInput::Y => Self::Draw,
            UnknownInput::Z => Self::Win,
        }
    }
}

pub struct Day02;

impl Day for Day02 {
    type Input = Vec<(Choice, UnknownInput)>;

    type Output = u32;

    fn part_1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .map(|game| (&game.0, Choice::from(&game.1)))
            .fold(0, |score, game| {
                score + game.1.score() + game.1.outcome(game.0).score()
            })
    }

    fn part_2(input: &Self::Input) -> Self::Output {
        // let solutions = {
        //     let mut solutions = HashMap::new();
        //
        //     solutions.insert((Choice::Rock, Outcome::Win), Choice::Paper);
        //     solutions.insert((Choice::Rock, Outcome::Draw), Choice::Rock);
        //     solutions.insert((Choice::Rock, Outcome::Loss), Choice::Scissors);
        //
        //     solutions.insert((Choice::Paper, Outcome::Win), Choice::Scissors);
        //     solutions.insert((Choice::Paper, Outcome::Draw), Choice::Paper);
        //     solutions.insert((Choice::Paper, Outcome::Loss), Choice::Rock);
        //
        //     solutions.insert((Choice::Scissors, Outcome::Win), Choice::Rock);
        //     solutions.insert((Choice::Scissors, Outcome::Draw), Choice::Scissors);
        //     solutions.insert((Choice::Scissors, Outcome::Loss), Choice::Paper);
        //
        //     solutions
        // };

        input
            .iter()
            .map(|game| (game.0.clone(), Outcome::from(&game.1)))
            .fold(0, |score, game| {
                let outcome = match (&game.0, &game.1) {
                    (_, Outcome::Draw) => game.0.clone(),
                    (Choice::Rock, Outcome::Win) => Choice::Paper,
                    (Choice::Rock, Outcome::Loss) => Choice::Scissors,
                    (Choice::Paper, Outcome::Win) => Choice::Scissors,
                    (Choice::Paper, Outcome::Loss) => Choice::Rock,
                    (Choice::Scissors, Outcome::Win) => Choice::Rock,
                    (Choice::Scissors, Outcome::Loss) => Choice::Paper,
                };
                //let outcome = solutions.get(&game).unwrap();

                score + game.1.score() + outcome.score()
            })
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|line| {
                (
                    Choice::from(UnknownInput::from(line.chars().next().unwrap())),
                    UnknownInput::from(line.chars().last().unwrap()),
                )
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "A Y
B X
C Z";

    assert_eq!(Day02::run(input), (15, 12));
}
