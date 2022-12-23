use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use crate::day23::Direction::{East, North, South, West};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn neighbors(&self, (x, y): (i32, i32)) -> [(i32, i32); 3] {
        match self {
            North => [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
            South => [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
            West =>  [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
            East =>  [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    fn translate(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        }
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}

fn step(occupied: &mut HashSet<(i32, i32)>, directions: [Direction; 4], round: usize) -> bool {
    let mut moves = HashMap::new();

    for &(x, y) in occupied.iter() {
        if directions.into_iter().flat_map(|d| d.neighbors((x, y))).unique().all(|n| !occupied.contains(&n)) {
            continue;
        }

        for direction in (0..4).map(|i| directions[(round + i) % 4]) {
            if direction.neighbors((x, y)).iter().all(|n| !occupied.contains(n)) {
                let target = direction.translate((x, y));
                moves.entry(target).or_insert_with(Vec::new).push((x, y));
                break;
            }
        }
    }

    if moves.is_empty() {
        return true;
    }

    for (target, sources) in moves {
        if sources.len() != 1 {
            continue;
        }

        occupied.insert(target);
        occupied.remove(&sources[0]);
    }

    false
}

#[aoc(day23, part1)]
fn part1(input: &HashSet<(i32, i32)>) -> usize {
    let mut occupied = input.clone();
    let directions = [ North, South, West, East ];

    for round in 0..10 {
        step(&mut occupied, directions, round);
    }

    let (min_x, min_y, max_x, max_y) = (
        occupied.iter().map(|(x, _)| *x).min().unwrap(),
        occupied.iter().map(|(_, y)| *y).min().unwrap(),
        occupied.iter().map(|(x, _)| *x).max().unwrap(),
        occupied.iter().map(|(_, y)| *y).max().unwrap(),
    );

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    width * height - occupied.len()
}

#[aoc(day23, part2)]
fn part2(input: &HashSet<(i32, i32)>) -> usize {
    let mut occupied = input.clone();
    let directions = [ North, South, West, East ];

    for round in 0.. {
        if step(&mut occupied, directions, round) {
            return round + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(110, part1(&parse(include_str!("../input/2022/day23.part1.test.110.txt"))));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(25, part1(&parse(include_str!("../input/2022/day23.part1.test.25.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(3800, part1(&parse(include_str!("../input/2022/day23.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(20, part2(&parse(include_str!("../input/2022/day23.part2.test.20.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(916, part2(&parse(include_str!("../input/2022/day23.txt"))));
    }
}