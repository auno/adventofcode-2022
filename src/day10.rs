use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, bail, Context};
use crate::day10::Instruction::{AddX, NoOp};
use crate::ocr::ocr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Instruction> {
        let mut parts = s.split(' ');
        match parts.next() {
            Some("noop") => Ok(NoOp),
            Some("addx") => Ok(AddX(parts.next().context("Missing operand")?.parse()?)),
            Some(_) | None => bail!("Malformed input line: {}", s),
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(&str::parse)
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    input.iter()
        .flat_map(|instruction| match *instruction {
            NoOp => vec![NoOp],
            AddX(v) => vec![NoOp, AddX(v)],
        })
        .take(221)
        .enumerate()
        .map(|(cycle, instruction)| (cycle + 1, instruction))
        .fold((1, 0), |(mut x, mut cumulative_strength), (cycle, instruction)| {
            cumulative_strength += match cycle % 40 {
                20 => (cycle as i32) * x,
                _ => 0,
            };

            x += match instruction {
                NoOp => 0,
                AddX(v) => v,
            };

            (x, cumulative_strength)
        })
        .1
}

fn execute_instructions(input: &[Instruction]) -> String {
    input.iter()
        .flat_map(|instruction| match *instruction {
            NoOp => vec![NoOp],
            AddX(v) => vec![NoOp, AddX(v)],
        })
        .enumerate()
        .map(|(cycle, instruction)| (cycle, instruction))
        .fold((1, vec!['.'; 240]), |(x, mut screen), (cycle, instruction)| {
            if ((x - 1)..=(x + 1)).contains(&((cycle % 40) as i32)) {
                screen[cycle] = '#';
            }

            let x = match instruction {
                NoOp => x,
                AddX(v) => x + v,
            };

            (x, screen)
        })
        .1
        .iter()
        .collect::<String>()
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> String {
    let screen = execute_instructions(input);
    ocr('#', &screen).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(13140, part1(&parse(include_str!("../input/2022/day10.part1.test.13140.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(13220, part1(&parse(include_str!("../input/2022/day10.txt")).unwrap()));
    }

    #[test]
    fn execute_instructions_example1() {
        let expected = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ].join("");

        assert_eq!(expected, execute_instructions(&parse(include_str!("../input/2022/day10.part2.test.124.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!("RUAKHBEK", part2(&parse(include_str!("../input/2022/day10.txt")).unwrap()));
    }
}