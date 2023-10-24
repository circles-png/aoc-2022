use aoc_runner_derive::{aoc, aoc_generator};

fn sides(tree_position: (usize, usize), forest: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let (x, y) = tree_position;
    let row = forest[y].clone();
    let column = forest.iter().map(|row| row[x]).collect::<Vec<u32>>();
    let mut sides = Vec::new();
    for (set, exclude_index) in [(&row, x), (&column, y)] {
        let mut set = set.clone();
        let (left, right) = set.split_at_mut(exclude_index);
        left.reverse();
        let right = &right[1..];

        sides.push(left.to_vec());
        sides.push(right.to_vec());
    }
    sides
}
fn visible(tree_position: (usize, usize), forest: &[Vec<u32>]) -> bool {
    let (x, y) = tree_position;
    let height = forest[y][x];
    sides(tree_position, forest)
        .iter()
        .any(|side| side.iter().all(|tree_height| tree_height < &height))
}

fn scenic_score(tree_position: (usize, usize), forest: &[Vec<u32>]) -> u32 {
    sides(tree_position, forest)
        .iter()
        .map(|side| {
            let mut dist = 0;
            let mut side = side.iter();
            loop {
                match side.next() {
                    Some(tree_height)
                        if tree_height < &forest[tree_position.1][tree_position.0] =>
                    {
                        dist += 1;
                    }
                    Some(_) => {
                        dist += 1;
                        break dist;
                    }
                    None => break dist,
                }
            }
        })
        .product()
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .map(|row| {
            row.chars()
                .map(|character| character.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
#[allow(clippy::cast_possible_truncation)]
fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _)| visible((*x, y), input))
                .count() as u32
        })
        .sum()
}

#[aoc(day8, part2)]
fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| scenic_score((x, y), input))
        })
        .max()
        .unwrap()
}
