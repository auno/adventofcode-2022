use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    (
        parse_stacks(stacks_input),
        parse_moves(moves_input)
    )
}

fn parse_stacks(stacks_input: &str) -> Vec<Vec<char>> {
    let mut stacks_input: VecDeque<&str> = stacks_input.lines().rev().collect();
    let stacks_header = stacks_input.pop_front().unwrap();
    let num_stacks = stacks_header.chars()
        .filter_map(|c| c.to_digit(10))
        .count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    for line in stacks_input {
        for (stack, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
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
    let mut buffer = Vec::new();

    for &(count, source, target) in moves {
        let source_offset = stacks[source].len() - count;
        buffer.extend(stacks[source].drain(source_offset..));
        stacks[target].extend(buffer.drain(0..));
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