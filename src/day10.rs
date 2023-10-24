use aoc_runner_derive::{aoc, aoc_generator};

enum Instruction {
    NoOperation,
    AddX(i32),
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            line.strip_prefix("addx ")
                .map_or(Instruction::NoOperation, |value| {
                    Instruction::AddX(value.parse().unwrap())
                })
        })
        .collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &[Instruction]) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strengths = 0;
    for instruction in input {
        let current = signal_strengths;
        match instruction {
            Instruction::NoOperation => {
                cycle += 1;
                if (cycle + 20) % 40 == 0 && current == signal_strengths {
                    let signal_strength = cycle * x;
                    signal_strengths += signal_strength;
                }
            }
            Instruction::AddX(value) => {
                cycle += 1;
                if (cycle + 20) % 40 == 0 && current == signal_strengths {
                    let signal_strength = cycle * x;
                    signal_strengths += signal_strength;
                }
                cycle += 1;
                if (cycle + 20) % 40 == 0 && current == signal_strengths {
                    let signal_strength = cycle * x;
                    signal_strengths += signal_strength;
                }

                x += value;
            }
        }
    }
    signal_strengths
}

#[aoc(day10, part2)]
fn solve_part2(input: &[Instruction]) -> String {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn cycle(display: &mut Vec<bool>, x: i32) {
        let centers = (0..6).map(|index| x + index * 40);
        display.push(
            centers
                .flat_map(|center| [center - 1, center, center + 1])
                .any(|x| x == (display.len() as i32)),
        );
    }
    let mut x = 1;
    let mut display = Vec::new();
    for instruction in input {
        match instruction {
            Instruction::NoOperation => {
                cycle(&mut display, x);
            }
            Instruction::AddX(value) => {
                cycle(&mut display, x);
                cycle(&mut display, x);
                x += value;
            }
        }
    }

    format!(
        "\n{}",
        display
            .chunks_exact(40)
            .map(|row| {
                row.iter()
                    .map(|pixel| if *pixel { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    )
}
