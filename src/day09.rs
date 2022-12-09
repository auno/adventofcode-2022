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

#[aoc(day9, part1)]
fn part1(input: &Vec<(Direction, usize)>) -> usize {
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let movements = input.iter()
        .flat_map(|(direction, count)| (0..*count).map(|_| *direction))
        .collect::<Vec<_>>();
    let mut t_been = HashSet::new();

    for movement in movements {
        (hx, hy) = match movement {
            Up => (hx, hy + 1),
            Down => (hx, hy - 1),
            Left => (hx - 1, hy),
            Right => (hx + 1, hy),
        };

        (tx, ty) = match () {
            _ if tx == hx && ty == hy + 2 => (tx, ty - 1),
            _ if tx == hx && ty == hy - 2 => (tx, ty + 1),
            _ if tx == hx + 2 && ty == hy => (tx - 1, ty),
            _ if tx == hx - 2 && ty == hy => (tx + 1, ty),

            _ if tx == hx + 2 && ty == hy + 2 => (tx - 1, ty - 1),
            _ if tx == hx - 2 && ty == hy - 2 => (tx + 1, ty + 1),
            _ if tx == hx + 2 && ty == hy - 2 => (tx - 1, ty + 1),
            _ if tx == hx - 2 && ty == hy + 2 => (tx + 1, ty - 1),

            _ if (tx == hx + 1 || tx == hx - 1) && ty == hy + 2 => (hx, hy + 1),
            _ if (tx == hx + 1 || tx == hx - 1) && ty == hy - 2 => (hx, hy - 1),
            _ if tx == hx + 2 && (ty == hy + 1 || ty == hy - 1) => (hx + 1, hy),
            _ if tx == hx - 2 && (ty == hy + 1 || ty == hy - 1) => (hx - 1, hy),

            _ => (tx, ty)
        };

        t_been.insert((tx, ty));
    }

    t_been.len()
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
}