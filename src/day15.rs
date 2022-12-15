use std::cmp::max;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Coordinate = (i64, i64);

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Vec<(Coordinate, Coordinate)>> {
    input
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = scan_fmt!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", i64, i64, i64, i64)?;
            Ok(((sx, sy), (bx, by)))
        })
        .collect()
}

fn distance((ax, ay): &Coordinate, (bx, by): &Coordinate) -> i64 {
    (ax - bx).abs() + (ay - by).abs()
}

fn find_and_normalize_ranges(input: &[(Coordinate, Coordinate)], y: i64) -> Vec<(i64, i64)> {
    let mut excluded = input
        .iter()
        .copied()
        .filter_map(|((sx, sy), beacon)| {
            let d = distance(&(sx, sy), &beacon);
            let dy = (sy - y).abs();

            if dy >= d {
                return None;
            }

            Some((sx - (d - dy), sx + (d - dy)))
        })
        .sorted()
        .collect::<Vec<_>>();

    let mut i = 1;

    while i < excluded.len() {
        let (a1, b1) = excluded[i - 1];
        let (a2, b2) = excluded[i];

        if a2 <= b1 {
            excluded[i - 1] = (a1, max(b1, b2));
            excluded.remove(i);
            continue;
        }

        i += 1;
    }

    excluded
}

fn solve1(input: &[(Coordinate, Coordinate)], y: i64) -> i64 {
    find_and_normalize_ranges(input, y)
        .iter()
        .map(|(a, b)| b - a)
        .sum()
}

#[aoc(day15, part1)]
fn part1(input: &[(Coordinate, Coordinate)]) -> i64 {
    solve1(input, 2000000)
}

fn solve2(input: &[(Coordinate, Coordinate)], max: i64) -> Option<i64> {
    (0..=max)
        .map(|y| find_and_normalize_ranges(input, y))
        .enumerate()
        .find(|(_, ranges)| ranges.len() > 1)
        .map(|(y, ranges)| (ranges[0].1 as i64 + 1, y as i64))
        .map(|(x, y)| x * 4000000 + y)
}

#[aoc(day15, part2)]
fn part2(input: &[(Coordinate, Coordinate)]) -> i64 {
    solve2(input, 4000000).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(26, solve1(&parse(include_str!("../input/2022/day15.part1.test.26.txt")).unwrap(), 10));
    }

    #[test]
    fn part1_input() {
        assert_eq!(4665948, part1(&parse(include_str!("../input/2022/day15.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(Some(56000011), solve2(&parse(include_str!("../input/2022/day15.part2.test.56000011.txt")).unwrap(), 20));
    }

    #[test]
    fn part2_input() {
        assert_eq!(13543690671045, part2(&parse(include_str!("../input/2022/day15.txt")).unwrap()));
    }
}