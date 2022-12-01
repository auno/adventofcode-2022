use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|line| line.lines().map(|line| line.parse().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.into_iter()
        .map(|calorie_counts| calorie_counts.into_iter().sum())
        .max()
        .unwrap_or_default()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    input.into_iter()
        .map(|calorie_counts| calorie_counts.into_iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(24000, part1(&parse(include_str!("../input/2022/day1.part1.test.24000.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(45000, part2(&parse(include_str!("../input/2022/day1.part2.test.45000.txt"))));
    }
}