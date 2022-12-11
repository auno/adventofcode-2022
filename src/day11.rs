use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use crate::day11::Operation::{Add, Mul, Square};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisibility_test: u64,
    target_if_true: usize,
    target_if_false: usize,
    num_inspected: usize,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Square,
    Add(u64),
    Mul(u64),
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

        match parts[..] {
            [ "new", "=", "old", "*", "old"] => Ok(Square),
            [ "new", "=", "old", "+", v] => Ok(Add(v.parse()?)),
            [ "new", "=", "old", "*", v] => Ok(Mul(v.parse()?)),
            _ => bail!("Unrecognized operation: {}", s),
        }
    }
}

fn parse_monkey(input: &str) -> Result<Monkey> {
    let mut input = input.lines();
    input.next();

    let items = scan_fmt!(input.next().context("Missing starting items")?, "Starting items: {[0-9, ]}", String)?
        .split(", ")
        .map(|item| item.parse().context("Could not parse item worry level"))
        .collect::<Result<Vec<u64>>>()?;
    let operation = scan_fmt!(input.next().context("Missing operation")?.trim(), "Operation: {/.*/}", String).map(|s| s.parse())??;
    let divisibility_test = scan_fmt!(input.next().context("Missing divisibility test")?.trim(), "Test: divisible by {d}", u64)?;
    let target_if_true = scan_fmt!(input.next().context("Missing target if true")?.trim(), "If true: throw to monkey {d}", usize)?;
    let target_if_false = scan_fmt!(input.next().context("Missing target if false")?.trim(), "If false: throw to monkey {d}", usize)?;

    Ok(Monkey {
        items,
        operation,
        divisibility_test,
        target_if_true,
        target_if_false,
        num_inspected: 0,
    })
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Vec<Monkey>> {
    input
        .split("\n\n")
        .map(parse_monkey)
        .collect()
}

fn solve(monkeys: &[Monkey], rounds: usize, management_technique: impl Fn(u64) -> u64) -> usize {
    let mut monkeys = monkeys.to_vec();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for mut item in monkeys[i].items.clone() {
                item = match monkeys[i].operation {
                    Square => item * item,
                    Add(v) => item + v,
                    Mul(v) => item * v,
                };
                item = management_technique(item);
                let target = if item % monkeys[i].divisibility_test == 0 {
                    monkeys[i].target_if_true
                } else {
                    monkeys[i].target_if_false
                } as usize;
                monkeys[target].items.push(item);
                monkeys[i].num_inspected += 1;
            }

            monkeys[i].items = vec![];
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.num_inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc(day11, part1)]
fn part1(monkeys: &[Monkey]) -> usize {
    solve(monkeys, 20, |item| item / 3)
}

#[aoc(day11, part2)]
fn part2(monkeys: &[Monkey]) -> usize {
    let modulus: u64 = monkeys.iter().map(|m| m.divisibility_test).product();
    solve(monkeys, 10000, |item| item % modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(10605, part1(&parse(include_str!("../input/2022/day11.part1.test.10605.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(54054, part1(&parse(include_str!("../input/2022/day11.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(2713310158, part2(&parse(include_str!("../input/2022/day11.part2.test.2713310158.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(14314925001, part2(&parse(include_str!("../input/2022/day11.txt")).unwrap()));
    }
}