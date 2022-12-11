use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
use crate::day11::Operation::{Add, Mul, Square};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisibility_test: u32,
    target_if_true: u32,
    target_if_false: u32,
    num_inspected: u32,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Square,
    Add(u32),
    Mul(u32),
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

    let starting_items = input.next().context("Monkey syntax error")?;
    let Some(("Starting items", items)) = starting_items.trim().split_once(": ") else {
        bail!("Monkey syntax error, could not parse starting items: {}", starting_items);
    };
    let items = items
        .split(", ")
        .map(|item| item.parse().context("Could not parse item worry level"))
        .collect::<Result<Vec<u32>>>()?;

    let operation = input.next().context("Monkey syntax error")?;
    let Some(("Operation", operation)) = operation.trim().split_once(": ") else {
        bail!("Monkey syntax error, could not parse operation: {}", operation);
    };
    let operation = operation.parse::<Operation>()?;

    let divisibility_test = input.next().context("Monkey syntax error")?;
    let Some(("Test: divisible by", divisibility_test)) = divisibility_test.trim().rsplit_once(' ') else {
        bail!("Monkey syntax error, could not parse divisibility test: {}", divisibility_test);
    };
    let divisibility_test = divisibility_test.parse::<u32>()?;

    let target_if_true = input.next().context("Monkey syntax error")?;
    let Some(("If true: throw to monkey", target_if_true)) = target_if_true.trim().rsplit_once(' ') else {
        bail!("Monkey syntax error, could not parse target if true: {}", target_if_true);
    };
    let target_if_true = target_if_true.parse::<u32>().context("Unable to parse target_if_true")?;

    let target_if_false = input.next().context("Monkey syntax error")?;
    let Some(("If false: throw to monkey", target_if_false)) = target_if_false.trim().rsplit_once(' ') else {
        bail!("Monkey syntax error, could not parse target if false: {}", target_if_false);
    };
    let target_if_false = target_if_false.parse::<u32>().context("Unable to parse target_if_true")?;

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

#[aoc(day11, part1)]
fn part1(input: &[Monkey]) -> u32 {
    let mut monkeys = input.to_vec();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            for mut item in monkeys[i].items.clone() {
                item = match monkeys[i].operation {
                    Square => item * item,
                    Add(v) => item + v,
                    Mul(v) => item * v,
                };
                item /= 3;
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
}