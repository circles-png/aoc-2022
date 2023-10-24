use aoc_runner_derive::{aoc, aoc_generator};

struct Elf {
    total_calories: u32,
}

impl Elf {
    const fn new(total_calories: u32) -> Self {
        Self { total_calories }
    }
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|chunk| {
            Elf::new(
                chunk
                    .split('\n')
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>(),
            )
        })
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(elves: &[Elf]) -> u32 {
    elves.iter().map(|elf| elf.total_calories).max().unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(elves: &[Elf]) -> u32 {
    ({
        let mut calories = elves
            .iter()
            .map(|elf| elf.total_calories)
            .collect::<Vec<u32>>();
        calories.sort_unstable();
        calories.reverse();
        calories
    })[0..3]
        .iter()
        .sum()
}
