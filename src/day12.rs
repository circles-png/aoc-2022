use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);
#[aoc_generator(day12)]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn input_generator(input: &str) -> (Vec<Vec<i32>>, Point, Point) {
    (
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|letter| match letter {
                        'S' => 0,
                        'E' => 25,
                        letter => letter as i32 - 'a' as i32,
                    })
                    .collect()
            })
            .collect(),
        {
            let y = input.lines().position(|line| line.contains('S')).unwrap();
            let x = input.lines().nth(y).unwrap().find('S').unwrap();
            (x as i32, y as i32)
        },
        {
            let y = input.lines().position(|line| line.contains('E')).unwrap();
            let x = input.lines().nth(y).unwrap().find('E').unwrap();
            (x as i32, y as i32)
        },
    )
}

#[aoc(day12, part1)]
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn solve_part1(input: &(Vec<Vec<i32>>, Point, Point)) -> i32 {
    let (map, start, end) = input;
    let mut queue = vec![(*end, 0)];
    let mut queue_index = 0;
    loop {
        let queue_copy = queue.clone();
        let ((current_x, current_y), value) = match queue_copy.get(queue_index) {
            Some(value) => *value,
            None => break,
        };
        if queue_copy.iter().any(|point| point.0 == *start) {
            break;
        }
        let adjacent = [
            (current_x, current_y + 1),
            (current_x, current_y - 1),
            (current_x + 1, current_y),
            (current_x - 1, current_y),
        ];
        let adjacent = adjacent
            .iter()
            .map(|point| (point, value + 1))
            .filter(|&point| {
                let ((x, y), _) = (*point.0, point.1);
                let height = *match map.get(y as usize).and_then(|row| row.get(x as usize)) {
                    Some(height) => height,
                    None => return false,
                };
                height >= map[current_y as usize][current_x as usize] - 1
                    && !queue_copy
                        .iter()
                        .any(|(queue_point, _)| *queue_point == (x, y))
            })
            .map(|point| (*point.0, point.1));
        queue.extend(adjacent);
        queue_index += 1;
    }
    let mut path = vec![*start];
    while path.last().unwrap() != end {
        let (current_x, current_y) = *path.last().unwrap();
        let adjacent = [
            (current_x, current_y + 1),
            (current_x, current_y - 1),
            (current_x + 1, current_y),
            (current_x - 1, current_y),
        ];
        let adjacent = adjacent
            .iter()
            .filter(|position| {
                queue.iter().any(|(point, _)| point == *position) && !path.contains(position)
            })
            .map(|position| {
                (
                    position,
                    queue.iter().find(|(point, _)| point == position).unwrap().1,
                )
            })
            .filter(|(_, value)| {
                queue
                    .iter()
                    .find(|(point, _)| *point == (current_x, current_y))
                    .unwrap()
                    .1
                    <= *value + 1
            })
            .min_by_key(|(_, value)| *value)
            .unwrap()
            .0;
        path.push(*adjacent);
    }
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if let Some((&(from_x, from_y), &(to_x, to_y))) = path
                .iter()
                .zip(path.iter().skip(1))
                .find(|(point, _)| **point == (x as i32, y as i32))
            {
                if from_x < to_x {
                    print!(">");
                } else if from_x > to_x {
                    print!("<");
                } else if from_y < to_y {
                    print!("v");
                } else if from_y > to_y {
                    print!("^");
                }
            } else if (x as i32, y as i32) == *end {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!();
    }
    path.len() as i32 - 1
}

#[aoc(day12, part2)]
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn solve_part2(input: &(Vec<Vec<i32>>, Point, Point)) -> i32 {
    let (map, _, end) = input;
    let mut queue = vec![(*end, 0)];
    let mut queue_index = 0;
    loop {
        let queue_copy = queue.clone();
        let ((current_x, current_y), value) = match queue_copy.get(queue_index) {
            Some(value) => *value,
            None => break,
        };
        if queue_copy
            .iter()
            .any(|(position, _)| map[position.1 as usize][position.0 as usize] == 0)
        {
            dbg!(queue_copy);
            break;
        }
        let adjacent = [
            (current_x, current_y + 1),
            (current_x, current_y - 1),
            (current_x + 1, current_y),
            (current_x - 1, current_y),
        ];
        let adjacent = adjacent
            .iter()
            .map(|point| (point, value + 1))
            .filter(|&(&(x, y), _)| {
                let height = *match map.get(y as usize).and_then(|row| row.get(x as usize)) {
                    Some(height) => height,
                    None => return false,
                };
                height >= map[current_y as usize][current_x as usize] - 1
                    && !queue_copy
                        .iter()
                        .any(|(queue_point, _)| *queue_point == (x, y))
            })
            .map(|point| (*point.0, point.1));
        queue.extend(adjacent);
        queue_index += 1;
    }
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let value = queue
                .iter()
                .find(|(point, _)| point == &(x as i32, y as i32));
            let ansi_bg = match value {
                Some((_, value)) => {
                    let color = 255
                        - (value * 255_i32)
                            .checked_div(queue.last().unwrap().1)
                            .unwrap_or(0);
                    format!("\x1b[48;2;{color};{color};{color}m")
                }
                None => "\x1b[48;2;255;0;0m".to_string(),
            };
            print!(
                "{}{}",
                ansi_bg,
                char::from_u32(*height as u32 + 'a' as u32).unwrap()
            );
        }
        println!("\x1b[0m\x1b[0m");
    }
    queue.last().unwrap().1
}
