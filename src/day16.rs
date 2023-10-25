use aoc_runner_derive::aoc_generator;
use regex::Regex;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<String> {
    let regex = Regex::new(r"Valve (?P<valve_name>[A-Z]+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)").unwrap();
    let result = regex.capture_names().collect::<Vec<_>>();
    dbg!(result);
    Vec::new()
}
