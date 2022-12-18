use std::collections::{HashSet, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use anyhow::Result;

type Point = (i32, i32, i32);

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse().unwrap())
        .tuples()
        .collect()
}

fn neighbors((x, y, z): &Point) -> [Point; 6] {
    [
        (x + 1, *y, *z),
        (x - 1, *y, *z),
        (*x, y + 1, *z),
        (*x, y - 1, *z),
        (*x, *y, z + 1),
        (*x, *y, z - 1),
    ]
}

fn within_bounds(((min_x, min_y, min_z), (max_x, max_y, max_z)): (Point, Point), (x, y, z): Point) -> bool {
    (min_x..=max_x).contains(&x) && (min_y..=max_y).contains(&y) && (min_z..=max_z).contains(&z)
}

fn fill(rocks: &HashSet<Point>, bounds: (Point, Point), source: Point) -> HashSet<Point> {
    assert!(!rocks.contains(&source));

    let mut seen = HashSet::from([source]);
    let mut queue = VecDeque::from([source]);

    while let Some(p) = queue.pop_front() {
        for n in neighbors(&p) {
            if rocks.contains(&n) || seen.contains(&n) || !within_bounds(bounds, n) {
                continue;
            }

            seen.insert(n);
            queue.push_back(n);
        }
    }

    seen
}

fn find_bounds(rocks: &HashSet<Point>) -> ((i32, i32, i32), (i32, i32, i32)) {
    (
        (
            rocks.iter().map(|(x, _, _)| x).min().unwrap() - 1,
            rocks.iter().map(|(_, y, _)| y).min().unwrap() - 1,
            rocks.iter().map(|(_, _, z)| z).min().unwrap() - 1,
        ),
        (
            rocks.iter().map(|(x, _, _)| x).max().unwrap() + 1,
            rocks.iter().map(|(_, y, _)| y).max().unwrap() + 1,
            rocks.iter().map(|(_, _, z)| z).max().unwrap() + 1,
        )
    )
}

#[aoc(day18, part1)]
fn part1(input: &[Point]) -> usize {
    let occupied: HashSet<&Point> = HashSet::from_iter(input);

    input
        .iter()
        .flat_map(neighbors)
        .filter(|p| !occupied.contains(p))
        .count()
}

#[aoc(day18, part2)]
fn part2(input: &[Point]) -> usize {
    let rocks: HashSet<Point> = HashSet::from_iter(input.iter().copied());
    let (min, max) = find_bounds(&rocks);
    let outside = fill(&rocks, (min, max), min);

    outside
        .iter()
        .flat_map(neighbors)
        .filter(|p| rocks.contains(p))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(64, part1(&parse(include_str!("../input/2022/day18.part1.test.64.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(4548, part1(&parse(include_str!("../input/2022/day18.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(58, part2(&parse(include_str!("../input/2022/day18.part2.test.58.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(2588, part2(&parse(include_str!("../input/2022/day18.txt"))));
    }
}