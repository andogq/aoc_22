use crate::day::Day;

pub fn wrapping_add(n: usize, d: isize, max: usize) -> usize {
    ((max + n) as isize + (d % max as isize)) as usize % max
}

fn mix(original: &[isize], decryption_key: usize, amount: usize) -> isize {
    let mut message = original
        .iter()
        .map(|&n| n * decryption_key as isize)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..amount {
        for (id, offset) in original
            .iter()
            .map(|&n| n * decryption_key as isize)
            .enumerate()
        {
            let current_index = message
                .iter()
                .enumerate()
                .find(|&(_, (original_id, _))| *original_id == id)
                .map(|(i, _)| i)
                .unwrap();
            let new_index = wrapping_add(current_index, offset, message.len() - 1);

            let item = message.remove(current_index);
            message.insert(new_index, item);
        }
    }

    let message = message.into_iter().map(|(_, n)| n).collect::<Vec<_>>();

    // Find the index of 0
    let zero_index = message
        .iter()
        .enumerate()
        .find(|(_, &n)| n == 0)
        .map(|(i, _)| i)
        .unwrap();

    (1..=3)
        .map(|n| message[wrapping_add(zero_index, n * 1000, message.len())])
        .sum()
}

pub struct Day20;
impl Day for Day20 {
    type Input = Vec<isize>;
    type Output = isize;

    fn part_1(input: Self::Input) -> Self::Output {
        let mut input: Vec<_> = input.into_iter().map(|n| (false, n)).collect();
        let input_size = input.len() - 1;

        while let Some(next_i) = {
            let mut next_index = None;

            for (i, (checked, _)) in input.iter().enumerate() {
                if !checked {
                    next_index = Some(i);
                    break;
                }
            }

            next_index
        } {
            // Get number
            let (_, offset) = input.remove(next_i);

            let mut new_index = ((input_size + next_i) as isize + (offset % input_size as isize))
                as usize
                % input_size;

            if new_index == 0 {
                new_index = input_size;
            }

            input.insert(new_index, (true, offset));
        }

        let base_i = input
            .iter()
            .enumerate()
            .find(|(_, &(_, n))| n == 0)
            .map(|(i, _)| i)
            .unwrap();

        (1..=3)
            .map(|n| input[(base_i + (n * 1000)) % input.len()].1)
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        mix(&input, 811589153, 10)
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines().map(|n| n.parse().unwrap()).collect()
    }
}

#[test]
fn test() {
    let input = "1
2
-3
3
-2
0
4
";
    assert_eq!(Day20::run(input), (3, 1623178306));
}
