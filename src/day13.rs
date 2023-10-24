use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<[Packet; 2]> {
    fn parse_packet(raw: &str) -> Packet {
        let mut packet = Packet::List(Vec::new());
        let inside = &raw[1..raw.len() - 1];
        let mut raw_items = Vec::new();
        let mut item = String::new();
        let mut bracket_balance = 0;
        for (index, character) in inside.char_indices() {
            bracket_balance += match character {
                '[' => 1,
                ']' => -1,
                _ => 0,
            };
            if bracket_balance == 0 && character == ',' {
                raw_items.push(item);
                item = String::new();
            } else if index == inside.len() - 1 {
                item.push(character);
                raw_items.push(item.clone());
            } else {
                item.push(character);
            }
        }
        for item in raw_items {
            let element = item
                .parse::<i32>()
                .map_or_else(|_| parse_packet(&item), Packet::Number);
            match &mut packet {
                Packet::List(list) => list.push(element),
                Packet::Number(_) => unreachable!(),
            }
        }
        packet
    }
    input
        .split("\n\n")
        .map(|packet_pair| {
            let pair: Vec<Packet> = packet_pair.lines().map(parse_packet).collect();
            pair.try_into().unwrap()
        })
        .collect()
}

fn correct(left: &Packet, right: &Packet) -> Option<bool> {
    match [left, right] {
        [Packet::Number(left), Packet::Number(right)] if left < right => Some(true),
        [Packet::Number(left), Packet::Number(right)] if left > right => Some(false),
        [Packet::List(left), Packet::List(right)] => {
            let mut left = left.iter();
            let mut right = right.iter();
            loop {
                let left = left.next();
                let right = right.next();
                match (left, right) {
                    (Some(left), Some(right)) => match correct(left, right) {
                        Some(true) => return Some(true),
                        Some(false) => return Some(false),
                        None => continue,
                    },
                    (None, Some(_)) => return Some(true),
                    (Some(_), None) => return Some(false),
                    (None, None) => return None,
                }
            }
        }
        [Packet::Number(number), Packet::List(list)] => {
            let number = Packet::List(vec![Packet::Number(*number)]);
            correct(&number, &Packet::List(list.clone()))
        }
        [Packet::List(list), Packet::Number(number)] => {
            let number = Packet::List(vec![Packet::Number(*number)]);
            correct(&Packet::List(list.clone()), &number)
        }
        _ => None,
    }
}

#[aoc(day13, part1)]
fn solve_part1(input: &[[Packet; 2]]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, [left, right])| {
            let correct = correct(left, right).unwrap();
            if correct {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &[[Packet; 2]]) -> usize {
    let mut packets: Vec<_> = input.iter().flatten().collect();
    let two = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.extend_from_slice(&[&two, &six]);
    packets.sort_unstable_by(|left, right| {
        if correct(left, right).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    (packets.iter().position(|packet| packet == &&two).unwrap() + 1)
        * (packets.iter().position(|packet| packet == &&six).unwrap() + 1)
}
