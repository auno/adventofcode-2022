use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn mix1(indices: &mut VecDeque<i32>, input: &[i32], original_index: i32) {
    if input[original_index as usize] == 0 {
        return;
    }

    let len = input.len() as i32;

    let i = indices.iter().position(|candidate| *candidate == original_index).unwrap() as i32;
    let mut next_i = i + input[original_index as usize];

    if next_i <= 0 || next_i >= len {
        next_i += input[original_index as usize].signum();
        next_i = next_i.rem_euclid(len);
    }

    let temp = indices.remove(i as usize).unwrap();
    indices.insert(next_i as usize, temp);
}

#[aoc(day20, part1)]
fn part1(input: &[i32]) -> i32 {
    let len = input.len() as i32;
    let mut indices = (0..len).collect::<VecDeque<_>>();

    for original_index in 0..len {
        mix1(&mut indices, input, original_index)
    }

    let pos0 = indices.iter().position(|i| input[*i as usize] == 0).unwrap();
    input[indices[(pos0 + 1000) % input.len()] as usize] + input[indices[(pos0 + 2000) % input.len()] as usize] + input[indices[(pos0 + 3000) % input.len()] as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(3, part1(&parse(include_str!("../input/2022/day20.part1.test.3.txt"))));
    }

    #[test]
    fn mix1_example1() {
        let input = [1, 2, -3, 3, -2, 0, 4];
        let mut indices = VecDeque::from([0, 1, 2, 3, 4, 5, 6]);

        mix1(&mut indices, &input, 0);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[2, 1, -3, 3, -2, 0, 4], actual.as_slice());

        mix1(&mut indices, &input, 1);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[1, -3, 2, 3, -2, 0, 4], actual.as_slice());

        mix1(&mut indices, &input, 2);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[1, 2, 3, -2, -3, 0, 4], actual.as_slice());

        mix1(&mut indices, &input, 3);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[1, 2, -2, -3, 0, 3, 4], actual.as_slice());

        mix1(&mut indices, &input, 4);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[1, 2, -3, 0, 3, 4, -2], actual.as_slice());

        mix1(&mut indices, &input, 5);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[1, 2, -3, 0, 3, 4, -2], actual.as_slice());

        mix1(&mut indices, &input, 6);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[1, 2, -3, 4, 0, 3, -2], actual.as_slice());
    }

    #[test]
    fn mix1_example2() {
        let input = [4, 5, 6, 1, 7, 8, 9];
        let mut indices = VecDeque::from([0, 1, 2, 3, 4, 5, 6]);

        mix1(&mut indices, &input, 3);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[4, 5, 6, 7, 1, 8, 9], actual.as_slice());
    }

    #[test]
    fn mix1_example3() {
        let input = [4, -2, 5, 6, 7, 8, 9];
        let mut indices = VecDeque::from([0, 1, 2, 3, 4, 5, 6]);

        mix1(&mut indices, &input, 1);
        let actual = indices.iter().map(|&i| input[i as usize]).collect::<Vec<_>>();
        assert_eq!(&[4, 5, 6, 7, 8, -2, 9], actual.as_slice());
    }

    #[test]
    fn part1_input() {
        assert_eq!(7713, part1(&parse(include_str!("../input/2022/day20.txt"))));
    }
}