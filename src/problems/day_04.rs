use crate::day::Day;

#[derive(Clone)]
pub struct Assignment(usize, usize);
impl From<&str> for Assignment {
    fn from(s: &str) -> Self {
        s.split_once('-')
            .map(|(start, end)| Assignment(start.parse().unwrap(), end.parse().unwrap()))
            .unwrap()
    }
}

pub struct Day04;
impl Day for Day04 {
    type Input = Vec<(Assignment, Assignment)>;

    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|(a, b)| {
                // Find the largest rang
                let (largest, smallest) = if a.1 - a.0 > b.1 - b.0 {
                    (a, b)
                } else {
                    (b, a)
                };

                largest.0 <= smallest.0 && largest.1 >= smallest.1
            })
            .count()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .filter(|(a, b)| (a.1 >= b.0 && a.0 <= b.0) || (b.1 >= a.0 && b.0 <= a.0))
            .count()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|line| {
                let (a, b) = line.split_once(',').unwrap();
                (Assignment::from(a), Assignment::from(b))
            })
            .collect()
    }
}
#[test]
fn test() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(Day04::run(input), (2, 4));
}
