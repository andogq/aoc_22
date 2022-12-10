use std::collections::HashSet;

use crate::day::Day;

type BoxedIter<T> = Box<dyn Iterator<Item = T>>;

pub struct Day08;
impl Day for Day08 {
    type Input = Vec<Vec<usize>>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        let height = input.len();
        let width = input.first().unwrap().len();

        let rows: BoxedIter<_> = Box::new((0..height).map(move |row_number| {
            Box::new((0..width).map(move |x| (x, row_number))) as BoxedIter<(usize, usize)>
        }));
        let cols = Box::new((0..width).map(move |col_number| {
            Box::new((0..height).map(move |y| (col_number, y))) as BoxedIter<_>
        }));

        [rows, cols]
            .into_iter()
            .flatten()
            .fold(HashSet::new(), |mut visible_trees, mut sequence| {
                let first_tree_pos = sequence.next().unwrap();
                let first_tree = input[first_tree_pos.1][first_tree_pos.0];

                // First tree is on an edge, always visible
                visible_trees.insert(first_tree_pos);

                let init = (visible_trees, first_tree, {
                    let mut reverse_trees = vec![None; 10];
                    reverse_trees[first_tree] = Some(first_tree_pos);
                    reverse_trees
                });

                let (mut forward_visible, _, reverse_visible) = sequence.fold(
                    init,
                    |(mut visible, mut largest, mut reverse_trees), tree_pos| {
                        let tree = input[tree_pos.1][tree_pos.0];

                        // Forward direction
                        if tree > largest {
                            visible.insert(tree_pos);

                            largest = tree;
                        }

                        // Reverse direction
                        (0..=tree).for_each(|smaller_tree| {
                            // Remove all smaller trees
                            reverse_trees[smaller_tree] = None;
                        });

                        // Add tree
                        reverse_trees[tree] = Some(tree_pos);

                        (visible, largest, reverse_trees)
                    },
                );

                // Combine forward and reverse visible
                forward_visible.extend(reverse_visible.into_iter().flatten());

                forward_visible
            })
            .len()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let height = input.len();
        let width = input.first().unwrap().len();

        (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| {
                let tree = input[y][x];

                // Travel in each direction
                [(0_isize, 1_isize), (1, 0), (0, -1), (-1, 0)]
                    .into_iter()
                    .map(|(dx, dy)| {
                        let mut steps = 0;
                        let (mut x, mut y) = (x, y);

                        // Ensure bounds are met (don't go negative or larger than array)
                        while !((x == 0 && dx.is_negative())
                            || (y == 0 && dy.is_negative())
                            || (x >= width - 1 && dx.is_positive())
                            || (y >= height - 1 && dy.is_positive()))
                        {
                            // Step to next tree
                            x = if dx.is_negative() {
                                x - dx.unsigned_abs()
                            } else {
                                x + dx.unsigned_abs()
                            };
                            y = if dy.is_negative() {
                                y - dy.unsigned_abs()
                            } else {
                                y + dy.unsigned_abs()
                            };

                            steps += 1;

                            // Early break out if tree height is too high
                            if input[y][x] >= tree {
                                break;
                            }
                        }

                        steps
                    })
                    .product::<usize>()
            })
            .max()
            .unwrap()
    }

    fn parse(raw: &str) -> Self::Input {
        raw.lines()
            .map(|line| {
                line.chars()
                    .map(|c| (c as usize) - ('0' as usize))
                    .collect()
            })
            .collect()
    }
}

#[test]
fn test() {
    let input = "30373
25512
65332
33549
35390";

    assert_eq!(Day08::run(input), (21, 8));
}
