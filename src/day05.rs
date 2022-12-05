use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (stacks_input, moves_input) = input.splitn(2, "\n\n").tuples().next().unwrap();

    (
        parse_stacks(stacks_input),
        parse_moves(moves_input)
    )
}

fn parse_stacks(stacks_input: &str) -> Vec<Vec<char>> {
    let mut stacks_input: VecDeque<&str> = stacks_input.lines().rev().collect();
    let stacks_header = stacks_input.pop_front().unwrap();
    let stacks_header: Vec<usize> = stacks_header
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c.to_digit(10) {
            None => None,
            Some(_) => Some(i),
        })
        .collect();

    let mut stacks: Vec<Vec<char>> = (0..stacks_header.len()).map(|_| Vec::new()).collect();

    for line in stacks_input {
        for (stack, &i) in (&stacks_header).into_iter().enumerate() {
            let c = line[i..=i].chars().next().unwrap();

            if c != ' ' {
                stacks[stack].push(c);
            }
        }
    }
    stacks
}

fn parse_moves(moves_input: &str) -> Vec<(usize, usize, usize)> {
    moves_input.lines().into_iter()
        .map(|line| scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap())
        .map(|(a, b, c)| (a, b - 1, c - 1))
        .collect()
}

#[aoc(day5, part1)]
fn part1((stacks, moves): &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
    let mut stacks = stacks.clone();

    for &(count, source, target) in moves {
        for _ in 0..count {
            let temp = stacks[source].pop().unwrap();
            stacks[target].push(temp);
        }
    }

    stacks.into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

#[aoc(day5, part2)]
fn part2((stacks, moves): &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
    let mut stacks = stacks.clone();

    for &(count, source, target) in moves {
        let mut buffer = Vec::new();

        for _ in 0..count {
            buffer.push(stacks[source].pop().unwrap());
        }

        while !buffer.is_empty() {
            stacks[target].push(buffer.pop().unwrap());
        }
    }

    stacks.into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!("CMZ", part1(&parse(include_str!("../input/2022/day5.part1.test.CMZ.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!("MCD", part2(&parse(include_str!("../input/2022/day5.part2.test.MCD.txt"))));
    }
}