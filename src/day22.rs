use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Error, Result};
use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Movement {
    TurnRight,
    TurnLeft,
    Forward(i32),
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "R" => Ok(Self::TurnRight),
            "L" => Ok(Self::TurnLeft),
            _ => Ok(Self::Forward(s.parse()?)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self, rotation: &Movement) -> Self {
        match (*self, rotation) {
            (Self::Up, Movement::TurnRight) => Self::Right,
            (Self::Down, Movement::TurnRight) => Self::Left,
            (Self::Left, Movement::TurnRight) => Self::Up,
            (Self::Right, Movement::TurnRight) => Self::Down,
            (Self::Up, Movement::TurnLeft) => Self::Left,
            (Self::Down, Movement::TurnLeft) => Self::Right,
            (Self::Left, Movement::TurnLeft) => Self::Down,
            (Self::Right, Movement::TurnLeft) => Self::Up,
            (_, Movement::Forward(_)) => *self,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Open),
            '#' => Ok(Self::Wall),
            _ => bail!("Unrecognized Tile: {}", value),
        }
    }
}

type Input = (HashMap<(i32, i32), Tile>, (i32, i32), Vec<Movement>);

#[aoc_generator(day22)]
fn parse(input: &str) -> Input {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let width = map.lines().map(&str::len).max().unwrap() as i32;
    let height = map.lines().count() as i32;

    let map = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter_map(|((x, y), c)| {
            Some(((x as i32, y as i32), c.try_into().ok()?))
        })
        .collect();

    let movement_token_pattern = Regex::new(r"(\d+|[RL])").unwrap();
    let movements = movement_token_pattern.captures_iter(movements)
        .filter_map(|c| {
            c.get(1)?.as_str().parse().ok()
        })
        .collect();

    (map, (width, height), movements)
}

#[aoc(day22, part1)]
fn part1((map, (width, height), movements): &Input) -> i32 {
    let (width, height) = (*width, *height);
    let mut y = 0;
    let mut x = (0..width).into_iter().find(|x| map.get(&(*x, y)) == Some(&Tile::Open)).unwrap();
    let mut direction = Direction::Right;

    for movement in movements {
        match movement {
            Movement::TurnRight | Movement::TurnLeft => { direction = direction.rotate(movement); },
            Movement::Forward(steps) => {
                let (xmod, ymod) = match direction {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };

                for _ in 0..*steps {
                    let (mut nx, mut ny) = (x + xmod, y + ymod);

                    while !map.contains_key(&(nx, ny)) {
                        (nx, ny) = ((nx + xmod).rem_euclid(width), (ny + ymod).rem_euclid(height))
                    }

                    if map.get(&(nx, ny)) == Some(&Tile::Wall) {
                        break;
                    }

                    (x, y) = (nx, ny);
                }
            }
        }
    }

    (y + 1) * 1000 + (x + 1) * 4 + direction.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(6032, part1(&parse(include_str!("../input/2022/day22.part1.test.6032.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(190066, part1(&parse(include_str!("../input/2022/day22.txt"))));
    }
}