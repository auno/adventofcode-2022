use std::cmp::{max, min};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Vec<u32>>) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut count = 0;

    for row in 0..height {
        for col in 0..width {
            let visible =
                (0..row).all(|i| input[i][col] < input[row][col]) ||
                ((row+1)..height).all(|i| input[i][col] < input[row][col]) ||
                (0..col).all(|i| input[row][i] < input[row][col]) ||
                ((col+1)..width).all(|i| input[row][i] < input[row][col]);

            if visible {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Vec<u32>>) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut best_score = 0;

    for row in 0..height {
        for col in 0..width {
            let distances = vec![
                min((0..row).rev().take_while(|&i| input[i][col] < input[row][col]).count() + 1, row),
                min(((row+1)..height).take_while(|&i| input[i][col] < input[row][col]).count() + 1, height - row - 1),
                min((0..col).rev().take_while(|&i| input[row][i] < input[row][col]).count() + 1, col),
                min(((col+1)..width).take_while(|&i| input[row][i] < input[row][col]).count() + 1, width - col - 1),
            ];
            let score = distances.iter().product();
            best_score = max(best_score, score);
        }
    }

    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(21, part1(&parse(include_str!("../input/2022/day8.part1.test.21.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(8, part2(&parse(include_str!("../input/2022/day8.part2.test.8.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1859, part1(&parse(include_str!("../input/2022/day8.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(332640, part2(&parse(include_str!("../input/2022/day8.txt"))));
    }
}