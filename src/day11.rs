use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    inspection_count: u32,
}

#[derive(Clone, Debug)]
struct Operation {
    kind: OperationKind,
    value: OperationValue,
}

#[derive(Clone, Debug)]
enum OperationKind {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
enum OperationValue {
    Constant(u128),
    OldValue,
}

#[derive(Clone, Debug)]
struct Test {
    divisible_by: u128,
    if_true: u128,
    if_false: u128,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.split('\n').map(str::trim);
            lines.next();
            let starting_items: Vec<u128> = lines
                .next()
                .unwrap()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();
            let operation = {
                let raw = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Operation: new = old ")
                    .unwrap();
                let mut parts = raw.split(' ');
                let kind = match parts.next().unwrap() {
                    "+" => OperationKind::Add,
                    "*" => OperationKind::Multiply,
                    _ => unreachable!(),
                };
                let value = match parts.next().unwrap() {
                    "old" => OperationValue::OldValue,
                    value => OperationValue::Constant(value.parse().unwrap()),
                };
                Operation { kind, value }
            };
            let test = {
                let divisible_by = lines
                    .next()
                    .unwrap()
                    .strip_prefix("Test: divisible by ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let if_true = lines
                    .next()
                    .unwrap()
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let if_false = lines
                    .next()
                    .unwrap()
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
                Test {
                    divisible_by,
                    if_true,
                    if_false,
                }
            };
            Monkey {
                items: starting_items,
                operation,
                test,
                inspection_count: 0,
            }
        })
        .collect()
}

#[aoc(day11, part1)]
fn solve_part1(input: &[Monkey]) -> u128 {
    let mut monkeys = input.to_vec();
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut directions = Vec::new();
            for worry_level in &mut monkey.items {
                match monkey.operation.value {
                    OperationValue::Constant(value) => match monkey.operation.kind {
                        OperationKind::Add => *worry_level += value,
                        OperationKind::Multiply => *worry_level *= value,
                    },
                    OperationValue::OldValue => match monkey.operation.kind {
                        OperationKind::Add => *worry_level += *worry_level,
                        OperationKind::Multiply => *worry_level *= *worry_level,
                    },
                }
                monkey.inspection_count += 1;
                *worry_level /= 3;
                directions.push((
                    if *worry_level % monkey.test.divisible_by == 0 {
                        monkey.test.if_true
                    } else {
                        monkey.test.if_false
                    } as usize,
                    *worry_level,
                ));
            }
            monkey.items = Vec::new();
            for (direction, item) in directions {
                monkeys[direction].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.inspection_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| u128::from(monkey.inspection_count))
        .product()
}

#[aoc(day11, part2)]
fn solve_part2(input: &[Monkey]) -> u64 {
    let mut monkeys = input.to_vec();
    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<u128>();
    for _ in 0..10_000 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut directions = Vec::new();
            for worry_level in &mut monkey.items {
                match monkey.operation.value {
                    OperationValue::Constant(value) => match monkey.operation.kind {
                        OperationKind::Add => *worry_level += value,
                        OperationKind::Multiply => *worry_level *= value,
                    },
                    OperationValue::OldValue => match monkey.operation.kind {
                        OperationKind::Add => *worry_level += *worry_level,
                        OperationKind::Multiply => *worry_level *= *worry_level,
                    },
                }
                *worry_level %= modulo;
                monkey.inspection_count += 1;
                directions.push((
                    if *worry_level % monkey.test.divisible_by == 0 {
                        monkey.test.if_true
                    } else {
                        monkey.test.if_false
                    } as usize,
                    *worry_level,
                ));
            }
            monkey.items = Vec::new();
            for (direction, item) in directions {
                monkeys[direction].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.inspection_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| u64::from(monkey.inspection_count))
        .product()
}
