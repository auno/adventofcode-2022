use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use anyhow::{Context, Result};
use itertools::Itertools;
use crate::day13::Value::{List, Number};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    Number(u32),
    List(Vec<Value>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Number(u32),
}

fn tokenize(line: &str) -> Vec<Token> {
    lazy_static! {
        static ref TOKEN_PATTERN: Regex = Regex::new(r"(\d+|[\[\],])").unwrap();
    }

    TOKEN_PATTERN.find_iter(line)
        .filter_map(|m| {
            let token = &line[m.start()..m.end()];
            match token {
                "[" => Some(Token::LeftBracket),
                "]" => Some(Token::RightBracket),
                "," => None,
                _ => Some(Token::Number(token.parse().unwrap())),
            }
        })
        .collect()
}

fn parse_tokenized_signal_list(tokens: &[Token]) -> (Value, usize) {
    let mut list = vec![];

    if tokens.first() != Some(&Token::LeftBracket) {
        panic!("Malformed signal list: Expected LeftBracket");
    }

    let mut i = 1;

    loop {
        if i >= tokens.len() {
            panic!("Malformed signal list: Unexpected end of broadcast");
        }

        match tokens[i] {
            Token::RightBracket => {
                i += 1;
                break;
            },
            _ => {
               let (value, num_parsed) = parse_tokenized_signal_value(&tokens[i..]);
                list.push(value);
                i += num_parsed;
            },
        }
    }

    (List(list), i)
}

fn parse_tokenized_signal_value(tokens: &[Token]) -> (Value, usize) {
    match tokens.first() {
        Some(Token::LeftBracket) => parse_tokenized_signal_list(tokens),
        Some(Token::Number(v)) => (Number(*v), 1),
        Some(Token::RightBracket) => panic!("Malformed signal value: Unexpected RightBracket"),
        None => panic!("Malformed signal value: Unexpected end of broadcast"),
    }
}

fn parse_tokenized_signal(tokens: &[Token]) -> Value {
    let (value, num_parsed) = parse_tokenized_signal_value(tokens);
    assert_eq!(tokens.len(), num_parsed);
    value
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Vec<(Value, Value)>> {
    input
        .split("\n\n")
        .map(|pair| {
            let mut pair = pair
                .split('\n')
                .map(tokenize)
                .map(|tokens| parse_tokenized_signal(&tokens));

            Ok((
                pair.next().context("Malformed pair: missing first element")?,
                pair.next().context("Malformed pair: missing second element")?,
            ))
        })
        .collect()
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b) = (self, other);
        let (a, b) = match (a, b) {
            (Number(av), Number(bv)) => return av.cmp(bv),
            (Number(_), List(bl)) => (vec![a], bl.iter().collect()),
            (List(al), Number(_)) => (al.iter().collect(), vec![b]),
            (List(al), List(bl)) => (al.iter().collect(), bl.iter().collect()),
        };

        let ordering = a
            .iter()
            .zip(b.iter())
            .fold(Equal, |acc, (a, b)| {
                match acc {
                    Less | Greater => acc,
                    Equal => a.cmp(b),
                }
            });

        if ordering == Equal {
            return a.len().cmp(&b.len());
        }

        ordering
    }
}

#[aoc(day13, part1)]
fn part1(input: &[(Value, Value)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[(Value, Value)]) -> usize {
    let dividers = vec![
        List(vec![List(vec![Number(2)])]),
        List(vec![List(vec![Number(6)])]),
    ];

    input
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .chain(&dividers)
        .sorted()
        .enumerate()
        .filter(|(_, signal)| dividers.contains(*signal))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(13, part1(&parse(include_str!("../input/2022/day13.part1.test.13.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6420, part1(&parse(include_str!("../input/2022/day13.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(140, part2(&parse(include_str!("../input/2022/day13.part2.test.140.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(22000, part2(&parse(include_str!("../input/2022/day13.txt")).unwrap()));
    }
}