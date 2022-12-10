use std::collections::{HashMap, HashSet};

use crate::day::Day;

pub struct Day08;
impl Day for Day08 {
    type Input = (Vec<usize>, usize, usize);
    type Output = usize;

    fn part_1((input, height, width): Self::Input) -> Self::Output {
        // Rows
        let rows: Box<dyn Iterator<Item = Box<dyn Iterator<Item = usize>>>> =
            Box::new((0..height).map(|row_number| {
                Box::new(row_number * width..(row_number + 1) * width)
                    as Box<dyn Iterator<Item = usize>>
            }));
        // Cols
        let cols: Box<dyn Iterator<Item = Box<dyn Iterator<Item = usize>>>> =
            Box::new((0..width).map(|col_number| {
                Box::new((col_number..col_number + (height * width)).step_by(width))
                    as Box<dyn Iterator<Item = usize>>
            }));

        [rows, cols]
            .into_iter()
            .flatten()
            .fold(HashSet::new(), |mut visible_trees, mut sequence_indexes| {
                let first_tree_index = sequence_indexes.next().unwrap();
                let first_tree = input[first_tree_index];

                // First tree is on an edge, always visible
                visible_trees.insert(first_tree_index);

                let init = (visible_trees, first_tree, {
                    let mut reverse_trees = HashMap::new();
                    reverse_trees.insert(first_tree, vec![first_tree_index]);
                    reverse_trees
                });
                let (mut forward_visible, _, reverse_visible) = sequence_indexes.fold(
                    init,
                    |(mut visible, mut largest, mut reverse_trees), tree_index| {
                        let tree = input[tree_index];

                        // Forward direction
                        if tree > largest {
                            visible.insert(tree_index);

                            largest = tree;
                        }

                        // Reverse direction
                        (0..=tree).for_each(|smaller_tree| {
                            // Remove all smaller trees
                            reverse_trees.remove(&smaller_tree);
                        });

                        // Add tree
                        reverse_trees.entry(tree).or_default().push(tree_index);

                        (visible, largest, reverse_trees)
                    },
                );

                // Combine forward and reverse visible
                forward_visible.extend(reverse_visible.values().flatten());

                forward_visible
            })
            .len()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        // let mut largest = 0;
        //
        // for y in 0..input.len() {
        //     for x in 0..input[y].len() {
        //         let mut score = 1;
        //         let tree = input[y][x];
        //
        //         let mut counter = 0;
        //         for new_y in (0..y).rev() {
        //             counter += 1;
        //             if input[new_y][x] >= tree {
        //                 break;
        //             }
        //         }
        //         score *= counter;
        //
        //         let mut counter = 0;
        //         for new_y in y + 1..input.len() {
        //             counter += 1;
        //             if input[new_y][x] >= tree {
        //                 break;
        //             }
        //         }
        //         score *= counter;
        //
        //         let mut counter = 0;
        //         for new_x in (0..x).rev() {
        //             counter += 1;
        //             if input[y][new_x] >= tree {
        //                 break;
        //             }
        //         }
        //         score *= counter;
        //
        //         let mut counter = 0;
        //         for new_x in x + 1..input[y].len() {
        //             counter += 1;
        //             if input[y][new_x] >= tree {
        //                 break;
        //             }
        //         }
        //         score *= counter;
        //
        //         if score > largest {
        //             largest = score;
        //         }
        //     }
        // }
        //
        // largest
        0
    }

    fn parse(raw: &str) -> Self::Input {
        (
            raw.lines()
                .flat_map(|line| line.chars().map(|c| (c as usize) - ('0' as usize)))
                .collect(),
            raw.lines().count(),
            raw.lines().next().unwrap().len(),
        )
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
