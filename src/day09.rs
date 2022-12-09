use std::collections::HashSet;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, bail};
use crate::day09::Direction::{Down, Left, Right, Up};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => bail!("Unknown direction: {}", s),
        }
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> anyhow::Result<Vec<(Direction, usize)>> {
    input
        .lines()
        .map(|line| {
            let Some((direction, count)) = line.split_once(' ') else {
                bail!("Unable to parse movement");
            };

            Ok((direction.parse()?, count.parse()?))
        })
        .collect()
}

fn expand_movements(movements: &Vec<(Direction, usize)>) -> Vec<Direction> {
    movements.iter()
        .flat_map(|(direction, count)| (0..*count).map(|_| *direction))
        .collect::<Vec<_>>()
}

fn apply_movement((x, y): (i32, i32), movement: Direction) -> (i32, i32) {
    match movement {
        Up => (x, y + 1),
        Down => (x, y - 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    }
}

fn follow((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        return (tx, ty);
    }

    (tx + (hx - tx).signum(), ty + (hy - ty).signum())
}

fn solve(input: &Vec<(Direction, usize)>, num_knots: usize) -> usize {
    let mut knots = vec![(0, 0); num_knots];
    let mut tail_been = HashSet::from([(0, 0)]);

    for movement in expand_movements(input) {
        knots[0] = apply_movement(knots[0], movement);

        for i in 1..(knots.len()) {
            knots[i] = follow(knots[i - 1], knots[i]);
        }

        tail_been.insert(knots[knots.len() - 1]);
    }

    tail_been.len()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<(Direction, usize)>) -> usize {
    solve(input, 2)
}

#[aoc(day9, part2)]
fn part2(input: &Vec<(Direction, usize)>) -> usize {
    solve(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(13, part1(&parse(include_str!("../input/2022/day9.part1.test.13.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6090, part1(&parse(include_str!("../input/2022/day9.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(1, part2(&parse(include_str!("../input/2022/day9.part2.test.1.txt")).unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(36, part2(&parse(include_str!("../input/2022/day9.part2.test.36.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(2566, part2(&parse(include_str!("../input/2022/day9.txt")).unwrap()));
    }
}