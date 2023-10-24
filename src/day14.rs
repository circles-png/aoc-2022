use std::{fmt::Write, thread::sleep, time::Duration};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn contains(self, point: Point) -> bool {
        let mut x_range = [self.start.0, self.end.0];
        x_range.sort_unstable();
        let x_range = x_range[0]..=x_range[1];
        let mut y_range = [self.start.1, self.end.1];
        y_range.sort_unstable();
        let y_range = y_range[0]..=y_range[1];
        x_range.contains(&point.0) && y_range.contains(&point.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Sand(Point);

fn blocked(point: Point, environment: (&[Line], &[Sand])) -> bool {
    environment.0.iter().any(|line| line.contains(point))
        || environment.1.iter().any(|sand| sand.0 == point)
}

impl Sand {
    fn fall(&mut self, environment: (&[Line], &[Self])) -> bool {
        let three_below = [
            (self.0 .0, self.0 .1 + 1),
            (self.0 .0 - 1, self.0 .1 + 1),
            (self.0 .0 + 1, self.0 .1 + 1),
        ];
        for point in three_below {
            if !blocked(point, environment) {
                self.0 = point;
                return true;
            }
        }
        false
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|path| {
            let points: Vec<_> = path.split(" -> ").collect();
            points
                .windows(2)
                .map(|points| {
                    let start = points[0]
                        .split(',')
                        .map(|number| number.parse().unwrap())
                        .collect::<Vec<_>>();
                    let end = points[1]
                        .split(',')
                        .map(|number| number.parse().unwrap())
                        .collect::<Vec<_>>();
                    Line {
                        start: (start[0], start[1]),
                        end: (end[0], end[1]),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

const SOURCE: Point = (500, 0);

fn display(environment: (&[Line], &[Sand])) {
    let (min_x, max_x, min_y, max_y) = (
        environment
            .0
            .iter()
            .map(|line| line.start.0)
            .min()
            .unwrap()
            .min(
                environment
                    .1
                    .iter()
                    .map(|sand| sand.0 .0)
                    .min()
                    .unwrap_or(i32::MAX),
            ),
        environment
            .0
            .iter()
            .map(|line| line.end.0)
            .max()
            .unwrap()
            .max(
                environment
                    .1
                    .iter()
                    .map(|sand| sand.0 .0)
                    .max()
                    .unwrap_or(i32::MIN),
            ),
        environment
            .0
            .iter()
            .map(|line| line.start.1)
            .min()
            .unwrap()
            .min(
                environment
                    .1
                    .iter()
                    .map(|sand| sand.0 .1)
                    .min()
                    .unwrap_or(i32::MAX),
            ),
        environment
            .0
            .iter()
            .map(|line| line.end.1)
            .max()
            .unwrap()
            .max(
                environment
                    .1
                    .iter()
                    .map(|sand| sand.0 .1)
                    .max()
                    .unwrap_or(i32::MIN),
            ),
    );
    let mut buffer = String::new();
    writeln!(buffer, "\n\n\n\n\n\n\n").unwrap();
    for y in min_y - 5..max_y + 5 {
        for x in min_x - 5..max_x + 5 {
            write!(
                buffer,
                "{}",
                if environment.0.iter().any(|line| line.contains((x, y))) {
                    "#"
                } else if environment.1.iter().any(|sand| sand.0 == (x, y)) {
                    "o"
                } else if (x, y) == SOURCE {
                    "+"
                } else {
                    "."
                }
            ).unwrap();
        }
        writeln!(buffer).unwrap();
    }
    println!("{buffer}");
}

#[aoc(day14, part1)]
fn solve_part1(rocks: &[Line]) -> usize {
    let mut sand = Vec::new();
    'outer: loop {
        let existing_sand = sand.clone();
        sand.push(Sand(SOURCE));
        let last_sand = sand.last_mut().unwrap();

        display((rocks, &existing_sand));
        while last_sand.fall((rocks, &existing_sand)) {
            if (last_sand.0 .1
                ..=rocks
                    .iter()
                    .map(|line| line.start.1.max(line.end.1))
                    .max()
                    .unwrap())
                .map(|y_position| {
                    rocks
                        .iter()
                        .all(|line| !line.contains((last_sand.0 .0, y_position)))
                })
                .all(|check| check)
            {
                break 'outer;
            }
        }
    }
    display((rocks, &sand));
    sand.len() - 1
}
