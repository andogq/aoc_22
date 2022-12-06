use std::collections::HashSet;

use crate::day::Day;

fn find_sequence(s: &Vec<char>, n: usize) -> usize {
    if s.len() > n {
        for i in n..s.len() {
            if HashSet::<&char>::from_iter(&s[i - n..i]).len() == n {
                return i;
            }
        }
    }

    unreachable!();
}

pub struct Day06;
impl Day for Day06 {
    type Input = Vec<char>;

    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        find_sequence(&input, 4)
    }

    fn part_2(input: Self::Input) -> Self::Output {
        find_sequence(&input, 14)
    }

    fn parse(raw: &str) -> Self::Input {
        raw.chars().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Day06::run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), (7, 19));
    assert_eq!(Day06::run("bvwbjplbgvbhsrlpgdmjqwftvncz"), (5, 23));
    assert_eq!(Day06::run("nppdvjthqldpwncqszvftbrmjlhg"), (6, 23));
    assert_eq!(Day06::run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), (10, 29));
    assert_eq!(Day06::run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), (11, 26));
}
