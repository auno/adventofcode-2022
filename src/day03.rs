use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| (line[0..(line.len() / 2)].to_string(), line[(line.len() / 2)..].to_string()))
        .collect()
}

fn value(c: char) -> u32 {
    let ord: u32 = c.into();

    match ord {
        97..=122 => ord - 97 + 1,
        65..=90  => ord - 65 + 27,
        _ => panic!("Unknown character: {}", c),
    }
}

#[aoc(day3, part1)]
fn part1(input: &Vec<(String, String)>) -> u32 {
    input.into_iter()
        .filter_map(|(a, b)| a.chars().find(|ac| b.chars().contains(ac)))
        .map(value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(157, part1(&parse(include_str!("../input/2022/day3.part1.test.157.txt"))));
    }
}