use std::collections::HashMap;

use crate::day::Day;

const MAX_DIR_SIZE: usize = 100000;
const DISK_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

pub struct Day07;
impl Day for Day07 {
    type Input = HashMap<String, usize>;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        // Find directories under MAX_DIR_SIZE
        input.into_values().filter(|&dir| dir <= MAX_DIR_SIZE).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let root_size = *input.get("/").unwrap();
        let remaining_size = DISK_SIZE - root_size;

        input.into_values().fold(root_size, |smallest, dir| {
            if remaining_size + dir >= UPDATE_SIZE && dir < smallest {
                dir
            } else {
                smallest
            }
        })
    }

    fn parse(raw: &str) -> Self::Input {
        let mut path = Vec::new();
        let mut dirs = HashMap::new();

        let mut lines = raw.lines().peekable();
        while let Some(line) = lines.next() {
            match &line[2..4] {
                "cd" => match &line[5..] {
                    ".." => {
                        path.pop();
                    }
                    "/" => path.clear(),
                    p => {
                        path.push(p.to_string());
                    }
                },
                "ls" => {
                    // Add file sizes to current directory
                    let mut dir_size = 0;
                    while {
                        if let Some(l) = lines.peek() {
                            !l.starts_with('$')
                        } else {
                            false
                        }
                    } {
                        let size_or_type = lines.next().unwrap().split_whitespace().next().unwrap();

                        if size_or_type != "dir" {
                            dir_size += size_or_type.parse::<usize>().unwrap();
                        }
                    }

                    // Update entry and all parent entries
                    for i in 0..=path.len() {
                        let path = format!("/{}", path[0..i].join("/"));
                        *dirs.entry(path).or_insert(0) += dir_size;
                    }
                }
                _ => unreachable!(),
            }
        }

        dirs
    }
}

#[test]
fn test() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    assert_eq!(Day07::run(input), (95437, 24933642));
}
