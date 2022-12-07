use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use crate::day07::Line::{CommandCd, CommandLs, Dir, File};

#[derive(Debug)]
enum AocError {}

#[derive(Debug)]
enum Line {
    CommandCd(String),
    CommandLs,
    Dir(String),
    File(String, usize),
}

impl FromStr for Line {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();

        match (parts[0], parts[1]) {
            ("$", "cd") => Ok(CommandCd(parts[2].to_string())),
            ("$", "ls") => Ok(CommandLs),
            ("dir", name) => Ok(Dir(name.to_string())),
            (size, name) => Ok(File(name.to_string(), size.parse().unwrap())),
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(&str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn basename(name: &str) -> String {
    let skips = if name.ends_with("/") {
        2
    } else {
        1
    };

    let parts = name.split("/").collect::<Vec<_>>();
    (&parts).into_iter().take(parts.len() - skips).join("/")
}

fn path_join(cwd: &str, target: &str) -> String {
    match target {
        ".." => basename(cwd),
        _ => format!("{}{}", cwd, target).to_string(),
    }
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Line>) -> usize {
    let mut cwd = "/".to_string();
    let mut files = vec![];
    let mut dirs = vec![];

    for line in input.into_iter().skip(1) {
        match line {
            CommandCd(target) => {
                cwd = format!("{}/", path_join(&cwd, target));
            },
            CommandLs => {},
            Dir(name) => {
                dirs.push(format!("{}/", path_join(&cwd, name)));
            },
            File(name, size) => {
                files.push((path_join(&cwd, name), *size));
            }
        }
    }

    (&dirs)
        .into_iter()
        .map(|dir| {
            (&files)
                .into_iter()
                .filter(|(name, _size)| name.starts_with(dir))
                .map(|(_name, size)| size)
                .sum::<usize>()
        })
        .filter(|size| *size <= 100000)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(95437, part1(&parse(include_str!("../input/2022/day7.part1.test.95437.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1886043, part1(&parse(include_str!("../input/2022/day7.txt"))));
    }
}