use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .flat_map(|range| range.split('-'))
        .map(|num| num.parse().unwrap())
        .tuples()
        .map(|(a, b, c, d)| ((a, b), (c, d)))
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<((u32, u32), (u32, u32))>) -> usize {
    input.into_iter()
        .filter(|((a1, b1), (a2, b2))| (a2 >= a1 && b2 <= b1) || (a1 >= a2 && b1 <= b2))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(2, part1(&parse(include_str!("../input/2022/day4.part1.test.2.txt"))));
    }
}