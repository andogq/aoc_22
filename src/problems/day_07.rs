use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::day::Day;

type Directory = HashMap<String, Entry>;

#[derive(Clone)]
pub struct FlattenedDirectories {
    pub dirs: Vec<usize>,
    pub size: usize,
}

pub enum Entry {
    File(usize),
    Dir(Rc<RefCell<Directory>>),
}

const MAX_DIR_SIZE: usize = 100000;
const DISK_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

pub struct Day07;
impl Day for Day07 {
    type Input = FlattenedDirectories;
    type Output = usize;

    fn part_1(input: Self::Input) -> Self::Output {
        // Find directories under MAX_DIR_SIZE
        input
            .dirs
            .into_iter()
            .filter(|&dir| dir <= MAX_DIR_SIZE)
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output {
        let remaining_size = DISK_SIZE - input.size;

        input.dirs.into_iter().fold(input.size, |smallest, dir| {
            if remaining_size + dir >= UPDATE_SIZE && dir < smallest {
                dir
            } else {
                smallest
            }
        })
    }

    fn parse(raw: &str) -> Self::Input {
        struct State {
            pub root: Rc<RefCell<Directory>>,
            pub path: Vec<String>,
        }

        let root = raw
            .lines()
            .fold(
                State {
                    root: Rc::new(RefCell::new(Directory::new())),
                    path: Vec::new(),
                },
                |State { root, mut path }, line| {
                    let current_dir = path.iter().fold(Rc::clone(&root), |dir, next_dir| {
                        if let Entry::Dir(dir) = dir.borrow().get(next_dir).unwrap() {
                            Rc::clone(dir)
                        } else {
                            unreachable!()
                        }
                    });

                    if line.starts_with('$') {
                        // Command
                        if line.contains("cd") {
                            // cd command
                            match &line[5..] {
                                ".." => {
                                    path.pop();
                                }
                                "/" => path.clear(),
                                p => {
                                    path.push(p.to_string());
                                }
                            }
                        }
                    } else {
                        // Directory listing
                        let (size_or_type, name) = line.split_once(' ').unwrap();
                        let name = name.to_string();

                        if size_or_type == "dir" {
                            // Create dir
                            current_dir
                                .borrow_mut()
                                .insert(name, Entry::Dir(Rc::new(RefCell::new(Directory::new()))));
                        } else {
                            // Create file
                            let size = size_or_type.parse().unwrap();

                            current_dir.borrow_mut().insert(name, Entry::File(size));
                        }
                    }

                    State { root, path }
                },
            )
            .root;

        fn size_dir(dir: Rc<RefCell<Directory>>) -> FlattenedDirectories {
            let mut size = 0;
            let mut dirs = Vec::new();

            for entry in dir.borrow().values() {
                match entry {
                    Entry::Dir(dir) => {
                        let sized_dir = size_dir(Rc::clone(dir));
                        size += sized_dir.size;

                        // Add this dir's sub dirs
                        dirs.extend(sized_dir.dirs);

                        // Add this dir
                        dirs.push(sized_dir.size);
                    }
                    Entry::File(file_size) => {
                        size += file_size;
                    }
                }
            }

            FlattenedDirectories { dirs, size }
        }

        size_dir(root)
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
