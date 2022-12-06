use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day6, part1)]
fn part1(input: &[char]) -> usize {
    input
        .windows(4)
        .enumerate()
        .find_map(|(i, w)| {
            if w.into_iter().tuple_combinations().all(|(a,b)| a != b) {
                Some(i + 3 + 1)
            } else {
                None
            }
        })
        .unwrap()
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
}