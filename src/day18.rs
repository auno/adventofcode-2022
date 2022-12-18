use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use anyhow::Result;

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse().unwrap())
        .tuples()
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[(i32, i32, i32)]) -> usize {
    let occupied: HashSet<&(i32, i32, i32)> = HashSet::from_iter(input);

    input
        .iter()
        .flat_map(|(x, y, z)| {
            [
                (x + 1, *y, *z),
                (x - 1, *y, *z),
                (*x, y + 1, *z),
                (*x, y - 1, *z),
                (*x, *y, z + 1),
                (*x, *y, z - 1),
            ]
        })
        .filter(|p| !occupied.contains(p))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(64, part1(&parse(include_str!("../input/2022/day18.part1.test.64.txt"))));
    }
}