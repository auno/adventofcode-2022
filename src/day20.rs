use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn solve(input: &[i64], multiplier: i64, rounds: usize) -> i64 {
    let len = input.len();
    let input = input.iter().map(|n| n * multiplier).collect::<Vec<_>>();
    let mut indices = (0..len).collect::<VecDeque<_>>();

    for _ in 0..rounds {
        for i in 0..len {
            let from = indices.iter().position(|candidate| *candidate == i).unwrap();
            let to = (from as i64 + input[i]).rem_euclid(len as i64 - 1) as usize;
            let temp = indices.remove(from).unwrap();
            indices.insert(to, temp);
        }
    }

    let pos0 = indices.iter().position(|&i| input[i] == 0).unwrap();
    input[indices[(pos0 + 1000) % len]] + input[indices[(pos0 + 2000) % len]] + input[indices[(pos0 + 3000) % len]]
}

#[aoc(day20, part1)]
fn part1(input: &[i64]) -> i64 {
    solve(input, 1, 1)
}

#[aoc(day20, part2)]
fn part2(input: &[i64]) -> i64 {
    solve(input, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(3, part1(&parse(include_str!("../input/2022/day20.part1.test.3.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(7713, part1(&parse(include_str!("../input/2022/day20.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(1623178306, part2(&parse(include_str!("../input/2022/day20.part2.test.1623178306.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1664569352803, part2(&parse(include_str!("../input/2022/day20.txt"))));
    }
}