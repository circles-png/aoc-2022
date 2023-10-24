use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    const fn value(self) -> u32 {
        match self {
            Self::Rock => 0,
            Self::Paper => 1,
            Self::Scissors => 2,
        }
    }

    const fn from_result_and_opponent(result: Ordering, opponent: Self) -> Self {
        match result {
            Ordering::Less => match opponent {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            Ordering::Equal => opponent,
            Ordering::Greater => match opponent {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::cast_possible_wrap)]
        match (self.value() as i32 - other.value() as i32).rem_euclid(3) {
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            2 => Ordering::Less,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = ();
    fn try_from(value: &str) -> std::result::Result<Self, ()> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

struct Round {
    opponent: Shape,
    player: (Shape, u32),
}

impl Round {
    const fn new(opponent: Shape, player: (Shape, u32)) -> Self {
        Self { opponent, player }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Round> {
    input
        .split('\n')
        .map(|line| {
            let mut line_split = line.split_ascii_whitespace();
            Round::new(line_split.next().unwrap().try_into().unwrap(), {
                let letter = line_split.next().unwrap();
                (
                    letter.try_into().unwrap(),
                    match letter {
                        "X" => 0,
                        "Y" => 3,
                        "Z" => 6,
                        _ => unreachable!(),
                    },
                )
            })
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(strategy_guide: &[Round]) -> u32 {
    strategy_guide
        .iter()
        .map(|round| {
            let shape_score = round.player.0.value() + 1;
            let round_result_score = match round.player.0.cmp(&round.opponent) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            };
            shape_score + round_result_score
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(strategy_guide: &[Round]) -> u32 {
    strategy_guide
        .iter()
        .map(|round| {
            let shape_score = Shape::from_result_and_opponent(
                match round.player.1 {
                    0 => Ordering::Less,
                    3 => Ordering::Equal,
                    6 => Ordering::Greater,
                    _ => unreachable!(),
                },
                round.opponent,
            )
            .value()
                + 1;
            let round_result_score = round.player.1;
            shape_score + round_result_score
        })
        .sum()
}
