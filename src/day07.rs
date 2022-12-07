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

fn parse_lines(lines: &[Line]) -> (Vec<(String, usize)>, Vec<String>) {
    let mut cwd = "/".to_string();
    let mut files = vec![];
    let mut dirs = vec![];

    for line in lines.iter().skip(1) {
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

    (files, dirs)
}

fn calculate_dir_sizes<'a>(files: &[(String, usize)], dirs: &'a [String]) -> Vec<(&'a str, usize)> {
    dirs
        .iter()
        .map(|dir| {
            (
                dir.as_str(),
                files
                    .iter()
                    .filter(|(name, _size)| name.starts_with(dir))
                    .map(|(_name, size)| size)
                    .sum::<usize>()
            )
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Line]) -> usize {
    let (files, dirs) = parse_lines(input);

    calculate_dir_sizes(&files, &dirs)
        .into_iter()
        .map(|(_name, size)| size)
        .filter(|size| *size <= 100000)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Line]) -> usize {
    let (files, dirs) = parse_lines(input);
    let total_used = files.iter().map(|(_name, size)| size).sum::<usize>();
    let needed = total_used - 40000000;

    calculate_dir_sizes(&files, &dirs)
        .into_iter()
        .map(|(_name, size)| size)
        .sorted()
        .find(|size| *size >= needed)
        .unwrap()
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

    #[test]
    fn part2_example1() {
        assert_eq!(24933642, part2(&parse(include_str!("../input/2022/day7.part2.test.24933642.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(3842121, part2(&parse(include_str!("../input/2022/day7.txt"))));
    }
}