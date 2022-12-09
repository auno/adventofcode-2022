use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::to_string)
        .collect()
}

fn priority(c: char) -> u32 {
    let ord: u32 = c.into();

    match ord {
        97..=122 => ord - 97 + 1,
        65..=90  => ord - 65 + 27,
        _ => panic!("Unknown character: {}", c),
    }
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> u32 {
    input.iter()
        .map(|line| (&line[0..(line.len() / 2)], &line[(line.len() / 2)..]))
        .filter_map(|(a, b)| {
            a.chars()
                .find(|ac| b.chars().contains(ac))
        })
        .map(priority)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> u32 {
    input.iter()
        .tuples()
        .filter_map(|(a, b, c)| {
            a.chars()
                .filter(|candidate| b.chars().contains(candidate))
                .find(|candidate| c.chars().contains(candidate))
        })
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(157, part1(&parse(include_str!("../input/2022/day3.part1.test.157.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(70, part2(&parse(include_str!("../input/2022/day3.part2.test.70.txt"))));
    }
}