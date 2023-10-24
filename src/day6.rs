use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    for index in 4 - 1..input.len() {
        let window = &input[index - (4 - 1)..=index];
        if window
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .all(|character| window.chars().filter(|&c| c == *character).count() == 1)
        {
            return index + 1;
        }
    }
    unreachable!()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    for index in 14 - 1..input.len() {
        let window = &input[index - (14 - 1)..=index];
        if window
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .all(|character| window.chars().filter(|&c| c == *character).count() == 1)
        {
            return index + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
