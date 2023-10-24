use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rucksack {
    compartments: [String; 2],
}

impl Rucksack {
    fn from_line(line: &str) -> Self {
        let second_compartment = line.to_string().split_off(line.len() / 2);
        Self {
            compartments: [line.to_string(), second_compartment],
        }
    }
}

fn item_type_to_priority(item: char) -> u32 {
    match item {
        item @ 'a'..='z' => item as u32 - 'a' as u32 + 1,
        item @ 'A'..='Z' => item as u32 - 'A' as u32 + 26 + 1,
        _ => unreachable!(),
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| Rucksack::from_line(&String::from(line)))
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .compartments
                .first()
                .unwrap()
                .chars()
                .find_map(|item_type| {
                    if rucksack.compartments.last().unwrap().contains(item_type) {
                        Some(item_type_to_priority(item_type))
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .chunks_exact(3)
        .map(|group| {
            let full_rucksacks = group.iter().map(|rucksack| rucksack.compartments.join(""));
            item_type_to_priority(
                full_rucksacks
                    .reduce(|accumulator, rucksack| {
                        accumulator
                            .chars()
                            .filter(|item_type| rucksack.contains(*item_type))
                            .collect::<String>()
                    })
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}
