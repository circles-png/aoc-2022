use aoc_runner_derive::{aoc, aoc_generator};

enum Line {
    Command(Command),
    Listing(Listing),
}

enum Command {
    ChangeDirectory(ChangeDirectory),
    List,
}

enum ChangeDirectory {
    Parent,
    Child(String),
}

enum Listing {
    Directory(String),
    File(u32, String),
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Node> {
    let input: Vec<_> = input
        .lines()
        .map(|line| {
            line.strip_prefix("$ ").map_or_else(
                || {
                    Line::Listing(line.strip_prefix("dir ").map_or_else(
                        || {
                            let mut parts = line.split(' ');
                            Listing::File(
                                parts.next().unwrap().parse().unwrap(),
                                parts.next().unwrap().to_string(),
                            )
                        },
                        |directory_name| Listing::Directory(directory_name.to_string()),
                    ))
                },
                |command| {
                    command.strip_prefix("cd ").map_or(
                        Line::Command(Command::List),
                        |directory_name| {
                            Line::Command(Command::ChangeDirectory(match directory_name {
                                ".." => ChangeDirectory::Parent,
                                directory_name => {
                                    ChangeDirectory::Child(directory_name.to_string())
                                }
                            }))
                        },
                    )
                },
            )
        })
        .collect();
    let input = input.as_slice();

    let mut file_system = vec![Node::Directory {
        id: 0,
        _name: "/".to_string(),
        parent: None,
    }];
    let mut current_working_directory: Option<usize> = Some(0);
    for line in input {
        match line {
            Line::Command(Command::ChangeDirectory(ChangeDirectory::Parent)) => {
                current_working_directory = Some(
                    Node::from_id(current_working_directory.unwrap(), &file_system)
                        .unwrap()
                        .parent()
                        .unwrap(),
                );
            }
            Line::Command(Command::ChangeDirectory(ChangeDirectory::Child(name))) => {
                let id = file_system.len();
                file_system.push(Node::Directory {
                    id,
                    _name: name.to_string(),
                    parent: current_working_directory,
                });
                current_working_directory = Some(id);
            }
            Line::Listing(Listing::Directory(name)) => {
                let id = file_system.len();
                file_system.push(Node::Directory {
                    id,
                    _name: name.to_string(),
                    parent: current_working_directory,
                });
            }
            Line::Listing(Listing::File(size, name)) => {
                let id = file_system.len();
                file_system.push(Node::File {
                    id,
                    _name: name.to_string(),
                    size: *size,
                    parent: current_working_directory.unwrap(),
                });
            }
            Line::Command(Command::List) => {}
        }
    }
    file_system
}

#[derive(Clone)]
enum Node {
    File {
        id: usize,
        _name: String,
        size: u32,
        parent: usize,
    },
    Directory {
        id: usize,
        _name: String,
        parent: Option<usize>,
    },
}

impl Node {
    const fn parent(&self) -> Option<usize> {
        match self {
            Self::File { parent, .. } => Some(*parent),
            Self::Directory { parent, .. } => *parent,
        }
    }

    fn children(&self, file_system: &[Self]) -> Option<Vec<usize>> {
        match self {
            Self::File { .. } => None,
            Self::Directory { id, .. } => Some(
                file_system
                    .iter()
                    .filter_map(|node| {
                        if node.parent()? == *id {
                            Some(node.id())
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
        }
    }

    fn from_id(id: usize, file_system: &[Self]) -> Option<&Self> {
        file_system.iter().find(|node| node.id() == id)
    }

    const fn id(&self) -> usize {
        match self {
            Self::File { id, .. } | Self::Directory { id, .. } => *id,
        }
    }

    fn size(&self, file_system: &Vec<Self>) -> u32 {
        match self {
            Self::File { size, .. } => *size,
            Self::Directory { .. } => self
                .children(file_system)
                .unwrap()
                .iter()
                .map(|node_id| {
                    Self::from_id(*node_id, file_system)
                        .unwrap()
                        .size(file_system)
                })
                .sum(),
        }
    }
}

#[aoc(day7, part1)]
fn solve_part1(file_system: &[Node]) -> u32 {
    let file_system = file_system.to_vec();
    file_system
        .iter()
        .filter_map(|node| {
            if matches!(node, Node::Directory { .. }) && node.size(&file_system) <= 100_000 {
                Some(node.size(&file_system))
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
fn solve_part2(file_system: &[Node]) -> u32 {
    let file_system = file_system.to_vec();
    let total_space = 70_000_000;
    let needed = 30_000_000;
    let space_used = file_system.first().unwrap().size(&file_system);
    let unused = total_space - space_used;
    let difference = needed - unused;
    let mut clone = file_system.clone();
    clone.sort_unstable_by_key(|node| node.size(&file_system));
    let directories_big_enough: Vec<_> = clone
        .iter()
        .filter(|node| node.size(&file_system) >= difference)
        .collect();
    directories_big_enough.first().unwrap().size(&file_system)
}
