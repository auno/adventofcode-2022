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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

    fn mods(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
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
type Jumps = HashMap<((i32, i32), Direction), ((i32, i32), Direction)>;

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

fn solve(map: &HashMap<(i32, i32), Tile>, movements: &Vec<Movement>, wrap_strategy: impl Fn((i32, i32), Direction) -> ((i32, i32), Direction)) -> i32 {
    let mut y = 0;
    let mut x = *map.iter().filter(|((_, y), t)| *y == 0 && **t == Tile::Open).map(|((x, _), _)| x).min().unwrap();
    let mut direction = Direction::Right;

    for movement in movements {
        match movement {
            Movement::TurnRight | Movement::TurnLeft => { direction = direction.rotate(movement); },
            Movement::Forward(steps) => {
                for _ in 0..*steps {
                    let (xmod, ymod) = direction.mods();
                    let (mut nx, mut ny, mut ndirection) = (x + xmod, y + ymod, direction);

                    ((nx, ny), ndirection) = wrap_strategy((nx, ny), ndirection);

                    if map.get(&(nx, ny)) == Some(&Tile::Wall) {
                        break;
                    }

                    (x, y, direction) = (nx, ny, ndirection);
                }
            }
        }
    }

    (y + 1) * 1000 + (x + 1) * 4 + direction.value()
}

fn solve2(map: &HashMap<(i32, i32), Tile>, movements: &Vec<Movement>, jumps: &mut Jumps) -> i32 {
    solve(map, movements, |(nx, ny), ndirection| {
        if !map.contains_key(&(nx, ny)) {
            return *jumps.get(&((nx, ny), ndirection)).unwrap();
        }

        ((nx, ny), ndirection)
    })
}

#[aoc(day22, part1)]
fn part1((map, (width, height), movements): &Input) -> i32 {
    solve(map, movements, |(mut nx, mut ny), ndirection| {
        let (xmod, ymod) = ndirection.mods();
        while !map.contains_key(&(nx, ny)) {
            (nx, ny) = ((nx + xmod).rem_euclid(*width), (ny + ymod).rem_euclid(*height))
        }

        ((nx, ny), ndirection)
    })
}

#[aoc(day22, part2)]
fn part2((map, _, movements): &Input) -> i32 {
    let mut jumps = HashMap::new();

    for i in 0..50 {
        jumps.insert(((49, 0 + i), Direction::Left), ((0, 149 - i), Direction::Right));
        jumps.insert(((-1, 100 + i), Direction::Left), ((50, 49 - i), Direction::Right));
        jumps.insert(((50 + i, -1), Direction::Up), ((0, 150 + i), Direction::Right));
        jumps.insert(((-1, 150 + i), Direction::Left), ((50 + i, 0), Direction::Down));
        jumps.insert(((100 + i, 50), Direction::Down), ((99, 50 + i), Direction::Left));
        jumps.insert(((100, 50 + i), Direction::Right), ((100 + i, 49), Direction::Up));
        jumps.insert(((150, 0 + i), Direction::Right), ((99, 149 - i), Direction::Left));
        jumps.insert(((100, 100 + i), Direction::Right), ((149, 49 - i), Direction::Left));
        jumps.insert(((100 + i, -1), Direction::Up), ((0 + i, 199), Direction::Up));
        jumps.insert(((0 + i, 200), Direction::Down), ((100 + i, 0), Direction::Down));
        jumps.insert(((49, 50 + i), Direction::Left), ((0 + i, 100), Direction::Down));
        jumps.insert(((0 + i, 99), Direction::Up), ((50, 50 + i), Direction::Right));
        jumps.insert(((50 + i, 150), Direction::Down), ((49, 150 + i), Direction::Left));
        jumps.insert(((50, 150 + i), Direction::Right), ((50 + i, 149), Direction::Up));
    }

    solve2(map, movements, &mut jumps)
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

    #[test]
    fn part2_example1() {
        let mut jumps = HashMap::new();

        for i in 0..4 {
            jumps.insert(((4 + i, 8), Direction::Down), ((8, 11 - i), Direction::Right));
            jumps.insert(((7, 8 + i), Direction::Left), ((7 - i, 7), Direction::Up));
            jumps.insert(((8 + i, 12), Direction::Down), ((3 - i, 7), Direction::Up));
            jumps.insert(((0 + i, 8), Direction::Down), ((11 - i, 11), Direction::Up));
            jumps.insert(((8 + i, -1), Direction::Up), ((3 - i, 4), Direction::Down));
            jumps.insert(((0 + i, 3), Direction::Up), ((11 - i, 0), Direction::Down));
            jumps.insert(((4 + i, 3), Direction::Up), ((8, 0 + i), Direction::Right));
            jumps.insert(((7, 0 + i), Direction::Left), ((4 + i, 4), Direction::Down));
            jumps.insert(((12 + i, 7), Direction::Up), ((11, 7 - i), Direction::Left));
            jumps.insert(((12, 4 + i), Direction::Right), ((15 - i, 8), Direction::Down));
            jumps.insert(((16, 8 + i), Direction::Right), ((11, 3 - i), Direction::Left));
            jumps.insert(((12, 0 + i), Direction::Right), ((15, 11 - i), Direction::Left));
            jumps.insert(((12 + i, 12), Direction::Down), ((0, 7 - i), Direction::Right));
            jumps.insert(((-1, 4 + i), Direction::Left), ((15 - i, 11), Direction::Up));
        }

        let (map, _, movements) = parse(include_str!("../input/2022/day22.part2.test.5031.txt"));
        assert_eq!(5031, solve2(&map, &movements, &mut jumps));
    }

    #[test]
    fn part2_input() {
        assert_eq!(134170, part2(&parse(include_str!("../input/2022/day22.txt"))));
    }
}