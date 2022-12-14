use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type RockSegment = ((u32, u32), (u32, u32));

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<RockSegment> {
    input
        .lines()
        .map(|line| {
            line
                .split(" -> ")
                .flat_map(|pair| {
                    pair
                        .split(',')
                        .map(|n| n.parse().unwrap())
                })
                .tuples()
                .collect::<Vec<(u32, u32)>>()
        })
        .flat_map(|sequence| {
            sequence
                .iter()
                .copied()
                .tuple_windows()
                .collect::<Vec<RockSegment>>()
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[RockSegment]) -> usize {
    let mut occupied = input
        .iter()
        .map(|&segment| match segment {
            ((ax, ay), (bx, by)) if ax > bx => ((bx, by), (ax, ay)),
            ((ax, ay), (bx, by)) if ax == bx && ay > by => ((bx, by), (ax, ay)),
            _ => segment,
        })
        .flat_map(|((ax, ay), (bx, by))| {
            (ax..=bx)
                .flat_map(|x| {
                    (ay..=by).map(|y| (x, y)).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    let max_y = occupied.iter().copied().map(|(_, y)| y).max().unwrap_or_default();

    for round in 0.. {
        let (mut x, mut y) = (500, 0);

        loop {
            if y >= max_y {
                return round;
            }

            let Some(next) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .into_iter()
                .find(|candidate| !occupied.contains(candidate))
            else {
                occupied.insert((x, y));
                break;
            };

            (x, y) = next;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(24, part1(&parse(include_str!("../input/2022/day14.part1.test.24.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1061, part1(&parse(include_str!("../input/2022/day14.txt"))));
    }
}