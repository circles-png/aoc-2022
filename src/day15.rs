use std::{collections::HashSet, ops::Range};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

const fn distance(a: Point, b: Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let sensor = parts.next().unwrap().strip_prefix("Sensor at ").unwrap();
            let sensor: Vec<i32> = sensor
                .split(", ")
                .map(|coordinate| coordinate[2..].parse().unwrap())
                .collect();
            let sensor = (sensor[0], sensor[1]);

            let closest_beacon = parts
                .next()
                .unwrap()
                .strip_prefix("closest beacon is at ")
                .unwrap();
            let closest_beacon: Vec<i32> = closest_beacon
                .split(", ")
                .map(|coordinate| coordinate[2..].parse().unwrap())
                .collect();
            let closest_beacon = (closest_beacon[0], closest_beacon[1]);

            Sensor {
                position: sensor,
                closest_beacon,
            }
        })
        .collect()
}

#[allow(dead_code)]
fn display(sensors: &[Sensor], range: (Range<i32>, Range<i32>), highlight: &[Point]) {
    for y in range.1 {
        for x in range.0.clone() {
            if sensors.iter().any(|sensor| sensor.position == (x, y)) {
                print!("S");
            } else if sensors.iter().any(|sensor| sensor.closest_beacon == (x, y)) {
                print!("B");
            } else if highlight.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

const ROW: i32 = 2_000_000;

fn points(sensor: &Sensor) -> Vec<Point> {
    let distance = distance(sensor.position, sensor.closest_beacon);
    let range_x = sensor.position.0 - distance..=sensor.position.0 + distance;
    let mut points = HashSet::new();
    for x in range_x {
        if ROW < sensor.position.1 - distance + (x - sensor.position.0).abs()
            || ROW > sensor.position.1 + distance - (x - sensor.position.0).abs()
        {
            continue;
        }
        points.insert((x, ROW));
    }
    points.iter().copied().collect::<Vec<_>>()
}

#[aoc(day15, part1)]
fn solve_part1(input: &[Sensor]) -> usize {
    let existing = input
        .iter()
        .flat_map(|sensor| [sensor.position, sensor.closest_beacon])
        .collect::<Vec<_>>();
    input
        .iter()
        .flat_map(points)
        .filter(|point| point.1 == ROW && !existing.contains(point))
        .collect::<HashSet<_>>()
        .len()
}
