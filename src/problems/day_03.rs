use std::collections::HashSet;

use crate::day::Day;

fn get_score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => unreachable!(),
    }
}

pub struct Day03;
impl Day for Day03 {
    type Input = Vec<(Vec<char>, Vec<char>)>;
    type Output = u32;

    fn part_1(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|rucksack| {
                let a: HashSet<char> = HashSet::from_iter(rucksack.0.into_iter());
                let b: HashSet<char> = HashSet::from_iter(rucksack.1.into_iter());

                let common = *a.intersection(&b).next().unwrap();
                get_score(common)
            })
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|rucksack| {
                HashSet::from_iter(rucksack.0.into_iter().chain(rucksack.1.into_iter()))
            })
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group: &[HashSet<char>]| {
                let common = *group[0]
                    .intersection(&group[1].intersection(&group[2]).cloned().collect())
                    .next()
                    .unwrap();
                get_score(common)
            })
            .sum()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|rucksack| {
                let half = rucksack.chars().count() / 2;

                (
                    Vec::from_iter(rucksack.chars().take(half)),
                    Vec::from_iter(rucksack.chars().skip(half)),
                )
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(Day03::run(input), (157, 70));
}
