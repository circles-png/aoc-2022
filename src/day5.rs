extern crate regex;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Crate = char;
type Stack = Vec<Crate>;

struct Step {
    quantity: usize,
    from_stack: usize,
    to_stack: usize,
}

type Input = (Vec<Stack>, Vec<Step>);

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    let [raw_stacks, raw_steps] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        unreachable!()
    };
    let rows = raw_stacks
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(index, character)| {
                    if index % 2 == 1 {
                        Some(character)
                    } else {
                        None
                    }
                })
                .enumerate()
                .filter_map(|(index, character)| {
                    if index % 2 == 0 {
                        Some(character)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .take(raw_stacks.lines().count() - 1)
        .collect::<Vec<Vec<char>>>();
    let stacks = (0..rows.iter().map(std::vec::Vec::len).max().unwrap())
        .map(|stack_index| {
            rows.iter()
                .map(|line| line.get(stack_index).copied().unwrap_or(' '))
                .filter(|character| !character.is_whitespace())
                .rev()
                .collect()
        })
        .collect();
    let steps = raw_steps
        .lines()
        .map(|line| {
            let captures = Regex::new(
                r"(?m)move (?P<quantity>\d+) from (?P<from_stack>\d+) to (?P<to_stack>\d+)",
            )
            .unwrap()
            .captures(line)
            .unwrap();
            let get = |name| {
                captures
                    .name(name)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap()
            };

            Step {
                quantity: get("quantity"),
                from_stack: get("from_stack") - 1,
                to_stack: get("to_stack") - 1,
            }
        })
        .collect();
    (stacks, steps)
}

#[aoc(day5, part1)]
fn solve_part1(input: &Input) -> String {
    let (stacks, steps) = input;
    let mut stacks = stacks.clone();
    for step in steps {
        let last = &mut stacks[step.from_stack]
            .iter()
            .rev()
            .take(step.quantity)
            .copied()
            .collect();
        stacks[step.to_stack].append(last);
        let stack_height = stacks[step.from_stack].len();
        stacks[step.from_stack].truncate(stack_height - step.quantity);
    }
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[aoc(day5, part2)]
fn solve_part2(input: &Input) -> String {
    let (stacks, steps) = input;
    let mut stacks = stacks.clone();
    for step in steps {
        let last = &mut stacks[step.from_stack]
            .iter()
            .rev()
            .take(step.quantity)
            .rev()
            .copied()
            .collect();
        stacks[step.to_stack].append(last);
        let stack_height = stacks[step.from_stack].len();
        stacks[step.from_stack].truncate(stack_height - step.quantity);
    }
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
