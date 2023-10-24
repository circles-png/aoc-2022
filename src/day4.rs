use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

type SectionAssignmentPair = [RangeInclusive<u32>; 2];

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<SectionAssignmentPair> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|section_assignment| {
                    let mut ends = section_assignment
                        .split('-')
                        .map(|end| end.parse::<u32>().unwrap());
                    ends.next().unwrap()..=ends.next().unwrap()
                })
                .collect::<Vec<RangeInclusive<u32>>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

#[aoc(day4, part1)]
#[allow(clippy::cast_possible_truncation)]
fn solve_part1(section_assignment_pairs: &[SectionAssignmentPair]) -> u32 {
    section_assignment_pairs
        .iter()
        .filter(|pair| {
            pair[0].clone().all(|section| pair[1].contains(&section))
                || pair[1].clone().all(|section| pair[0].contains(&section))
        })
        .count() as u32
}

#[aoc(day4, part2)]
#[allow(clippy::cast_possible_truncation)]
fn solve_part2(section_assignment_pairs: &[SectionAssignmentPair]) -> u32 {
    section_assignment_pairs
        .iter()
        .filter(|pair| pair[0].clone().any(|section| pair[1].contains(&section)))
        .count() as u32
}
