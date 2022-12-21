use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, Error};

enum Value {
    Constant(u64),
    Variable(String),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().all(|c| c.is_numeric()) {
            Ok(Value::Constant(s.parse()?))
        } else {
            Ok(Value::Variable(s.to_string()))
        }
    }
}

enum Expression {
    Addition(Value, Value),
    Subtraction(Value, Value),
    Multiplication(Value, Value),
    Division(Value, Value),
    Value(Value)
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

        match parts[..] {
            [a, "+", b] => Ok(Expression::Addition(a.parse()?, b.parse()?)),
            [a, "-", b] => Ok(Expression::Subtraction(a.parse()?, b.parse()?)),
            [a, "*", b] => Ok(Expression::Multiplication(a.parse()?, b.parse()?)),
            [a, "/", b] => Ok(Expression::Division(a.parse()?, b.parse()?)),
            _ => Ok(Expression::Value(s.parse()?)),
        }
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> HashMap<String, Expression> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(a, b)| (a.to_string(), b.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

fn resolve_value(cache: &mut HashMap<String, u64>, expressions: &HashMap<String, Expression>, value: &Value) -> u64 {
    match value {
        Value::Constant(v) => *v,
        Value::Variable(v) => resolve_expression(cache, expressions, v),
    }
}

fn resolve_expression(cache: &mut HashMap<String, u64>, expressions: &HashMap<String, Expression>, expression: &String) -> u64 {
    if let Some(v) = cache.get(expression) {
        return *v;
    }

    let resolved = match expressions.get(expression) {
        Some(Expression::Addition(a, b)) => resolve_value(cache, expressions, a) + resolve_value(cache, expressions, b),
        Some(Expression::Subtraction(a, b)) => resolve_value(cache, expressions, a) - resolve_value(cache, expressions, b),
        Some(Expression::Multiplication(a, b)) => resolve_value(cache, expressions, a) * resolve_value(cache, expressions, b),
        Some(Expression::Division(a, b)) => resolve_value(cache, expressions, a) / resolve_value(cache, expressions, b),
        Some(Expression::Value(a)) => resolve_value(cache, expressions, a),
        None => panic!("Unknown expression: {}", expression),
    };

    cache.insert(expression.clone(), resolved);
    resolved
}

#[aoc(day21, part1)]
fn part1(expressions: &HashMap<String, Expression>) -> u64 {
    resolve_expression(&mut HashMap::new(), expressions, &"root".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(152, part1(&parse(include_str!("../input/2022/day21.part1.test.152.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(56490240862410, part1(&parse(include_str!("../input/2022/day21.txt"))));
    }
}