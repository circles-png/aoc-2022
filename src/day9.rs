use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum Motion {
    Left,
    Right,
    Up,
    Down,
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Motion> {
    input
        .split('\n')
        .flat_map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let direction = match parts.next().unwrap() {
                "L" => Motion::Left,
                "R" => Motion::Right,
                "U" => Motion::Up,
                "D" => Motion::Down,
                _ => unreachable!(),
            };
            let distance = parts.next().unwrap().parse().unwrap();
            (0..distance).map(move |_| direction)
        })
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(input: &[Motion]) -> usize {
    let mut knots: Vec<(i32, i32)> = (0..2).map(|_| (0, 0)).collect();
    let mut tail_positions = HashSet::new();
    for motion in input {
        let (head_x, head_y) = knots.first_mut().unwrap();
        match motion {
            Motion::Left => *head_x -= 1,
            Motion::Right => *head_x += 1,
            Motion::Up => *head_y += 1,
            Motion::Down => *head_y -= 1,
        }
        for ((leader_x, leader_y), (knot_x, knot_y)) in
            knots.clone().iter().copied().zip(knots.iter_mut().skip(1))
        {
            if leader_x.abs_diff(*knot_x) >= 2 && leader_y == *knot_y {
                *knot_x += (leader_x - *knot_x).signum();
            } else if leader_y.abs_diff(*knot_y) >= 2 && leader_x == *knot_x {
                *knot_y += (leader_y - *knot_y).signum();
            } else if leader_x.abs_diff(*knot_x) > 1 || leader_y.abs_diff(*knot_y) > 1 {
                *knot_x += (leader_x - *knot_x).signum();
                *knot_y += (leader_y - *knot_y).signum();
            }
        }
        tail_positions.insert(*knots.last().unwrap());
    }
    tail_positions.len()
}

#[aoc(day9, part2)]
fn solve_part2(input: &[Motion]) -> usize {
    let mut knots: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();
    let mut tail_positions = HashSet::new();
    for motion in input {
        let (head_x, head_y) = knots.first_mut().unwrap();
        match motion {
            Motion::Left => *head_x -= 1,
            Motion::Right => *head_x += 1,
            Motion::Up => *head_y += 1,
            Motion::Down => *head_y -= 1,
        }
        for index in 1..knots.len() {
            let (leader_x, leader_y) = knots[index - 1];
            let (knot_x, knot_y) = &mut knots[index];
            if leader_x.abs_diff(*knot_x) >= 2 && leader_y == *knot_y {
                *knot_x += (leader_x - *knot_x).signum();
            } else if leader_y.abs_diff(*knot_y) >= 2 && leader_x == *knot_x {
                *knot_y += (leader_y - *knot_y).signum();
            } else if leader_x.abs_diff(*knot_x) > 1 || leader_y.abs_diff(*knot_y) > 1 {
                *knot_x += (leader_x - *knot_x).signum();
                *knot_y += (leader_y - *knot_y).signum();
            }
        }
        tail_positions.insert(*knots.last().unwrap());
    }
    tail_positions.len()
}
