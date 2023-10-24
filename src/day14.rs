use aoc_runner_derive::{aoc_generator, aoc};

type Point = (i32, i32);

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
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

#[aoc(day14, part1)]
fn solve_part1(input: &[Line]) -> u32 {
    dbg!(input);
    0
}
