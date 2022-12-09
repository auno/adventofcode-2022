use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type RangePair = ((u32, u32), (u32, u32));

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<RangePair> {
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
fn part1(input: &[RangePair]) -> usize {
    input.iter()
        .filter(|((a1, b1), (a2, b2))| (a2 >= a1 && b2 <= b1) || (a1 >= a2 && b1 <= b2))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[RangePair]) -> usize {
    input.iter()
        .map(|((a1, b1), (a2, b2))| match a1 <= a2 {
            true => ((a1, b1), (a2, b2)),
            false => ((a2, b2), (a1, b1)),
        })
        .filter(|((_, b1), (a2, _))| a2 <= b1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(2, part1(&parse(include_str!("../input/2022/day4.part1.test.2.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(4, part2(&parse(include_str!("../input/2022/day4.part2.test.4.txt"))));
    }
}