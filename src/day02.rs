use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use Play::{Paper, Rock, Scissors};

#[derive(Debug, Copy, Clone)]
enum AocError {
    InvalidPlay,
    InvalidStrategy,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Play {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err(AocError::InvalidPlay),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Strategy {
    X,
    Y,
    Z,
}

impl FromStr for Strategy {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Strategy::X),
            "Y" => Ok(Strategy::Y),
            "Z" => Ok(Strategy::Z),
            _ => Err(AocError::InvalidStrategy),
        }
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(Play, Strategy)> {
    input
        .lines()
        .map(|line| (
            line[0..1].parse::<Play>().unwrap(),
            line[2..3].parse::<Strategy>().unwrap()
        ))
        .collect()
}

fn value(play: Play) -> u32 {
    match play {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn score(other_play: Play, your_play: Play) -> u32 {
    let outcome = match (other_play, your_play) {
        (Rock, Paper) => 6,
        (Paper, Scissors) => 6,
        (Scissors, Rock) => 6,
        (a, b) if a == b => 3,
        (_, _) => 0,
    };

    value(your_play) + outcome
}

#[aoc(day2, part1)]
fn part1(input: &Vec<(Play, Strategy)>) -> u32 {
    input.into_iter()
        .map(|(a, b)| (
            a,
            match b {
                Strategy::X => Rock,
                Strategy::Y => Paper,
                Strategy::Z => Scissors,
            }
        ))
        .map(|(a, b)| score(*a, b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(15, part1(&parse(include_str!("../input/2022/day2.part1.test.15.txt"))));
    }
}