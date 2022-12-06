use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn solve(input: &[char], length: usize) -> Option<usize> {
    input
        .windows(length)
        .enumerate()
        .find_map(|(i, w)| {
            if w.into_iter().tuple_combinations().all(|(a,b)| a != b) {
                Some(i + length)
            } else {
                None
            }
        })
}

#[aoc(day6, part1)]
fn part1(input: &[char]) -> usize {
    solve(input, 4).unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &[char]) -> usize {
    solve(input, 14).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(7, part1(&parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(5, part1(&parse("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(6, part1(&parse("nppdvjthqldpwncqszvftbrmjlhg")));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(10, part1(&parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    }

    #[test]
    fn part1_example5() {
        assert_eq!(11, part1(&parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(19, part2(&parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(23, part2(&parse("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(23, part2(&parse("nppdvjthqldpwncqszvftbrmjlhg")));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(29, part2(&parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(26, part2(&parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
    }
}