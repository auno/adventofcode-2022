use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, Error};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Value {
    Constant(i64),
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

#[derive(Debug, Eq, PartialEq, Clone)]
enum Expression {
    Addition(Value, Value),
    Subtraction(Value, Value),
    Multiplication(Value, Value),
    Division(Value, Value),
    Equality(Value, Value),
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
            [a, "==", b] => Ok(Expression::Equality(a.parse()?, b.parse()?)),
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

fn resolve_value(cache: &mut HashMap<String, i64>, expressions: &HashMap<String, Expression>, value: &Value) -> Option<i64> {
    match value {
        Value::Constant(v) => Some(*v),
        Value::Variable(v) => resolve_expression(cache, expressions, v),
    }
}

fn resolve_expression(cache: &mut HashMap<String, i64>, expressions: &HashMap<String, Expression>, expression: &String) -> Option<i64> {
    if let Some(v) = cache.get(expression) {
        return Some(*v);
    }

    let resolved = match expressions.get(expression) {
        Some(Expression::Addition(a, b)) => resolve_value(cache, expressions, a)? + resolve_value(cache, expressions, b)?,
        Some(Expression::Subtraction(a, b)) => resolve_value(cache, expressions, a)? - resolve_value(cache, expressions, b)?,
        Some(Expression::Multiplication(a, b)) => resolve_value(cache, expressions, a)? * resolve_value(cache, expressions, b)?,
        Some(Expression::Division(a, b)) => resolve_value(cache, expressions, a)? / resolve_value(cache, expressions, b)?,
        Some(Expression::Value(a)) => resolve_value(cache, expressions, a)?,
        Some(Expression::Equality(_, _)) => panic!("Can't resolve equality"),
        None => return None,
    };

    cache.insert(expression.clone(), resolved);
    Some(resolved)
}

#[aoc(day21, part1)]
fn part1(expressions: &HashMap<String, Expression>) -> Option<i64> {
    resolve_expression(&mut HashMap::new(), expressions, &"root".to_string())
}

#[aoc(day21, part2)]
fn part2(expressions: &HashMap<String, Expression>) -> Option<i64> {
    let mut expressions = expressions.clone();
    let mut cache = HashMap::new();

    expressions.remove(&"humn".to_string());
    let root = match expressions.remove(&"root".to_string()) {
        Some(Expression::Addition(a, b)) => Expression::Equality(a, b),
        Some(Expression::Subtraction(a, b)) => Expression::Equality(a, b),
        Some(Expression::Multiplication(a, b)) => Expression::Equality(a, b),
        Some(Expression::Division(a, b)) => Expression::Equality(a, b),
        Some(e) => panic!("Unexpected root expression: {:?}", e),
        None => panic!("No root expression"),
    };
    expressions.insert("root".to_string(), root);

    let mut expression_name = "root";
    let mut expected = 0;

    loop {
        if expression_name == "humn" {
            return Some(expected);
        }

        let expression = expressions.get(&expression_name.to_string())?;

        let (a, b) = match expression {
            Expression::Addition(a, b) => (a, b),
            Expression::Subtraction(a, b) => (a, b),
            Expression::Multiplication(a, b) => (a, b),
            Expression::Division(a, b) => (a, b),
            Expression::Equality(a, b) => (a, b),
            Expression::Value(_) => panic!("Unexpected single value expression: {:?}", expression),
        };

        let (av, bv) = (
            resolve_value(&mut cache, &expressions, a),
            resolve_value(&mut cache, &expressions, b),
        );

        expected = match (expression, av, bv) {
            (Expression::Addition(_, _), Some(v), None) => expected - v,
            (Expression::Addition(_, _), None, Some(v)) => expected - v,
            (Expression::Subtraction(_, _), Some(v), None) => v - expected,
            (Expression::Subtraction(_, _), None, Some(v)) => expected + v,
            (Expression::Multiplication(_, _), Some(v), None) => expected / v,
            (Expression::Multiplication(_, _), None, Some(v)) => expected / v,
            (Expression::Division(_, _), Some(v), None) => v / expected,
            (Expression::Division(_, _), None, Some(v)) => expected * v,
            (Expression::Equality(_, _), Some(v), None) => v,
            (Expression::Equality(_, _), None, Some(v)) => v,
            (Expression::Value(_), _, _) => panic!("Unexpected single value expression: {:?}", expression),
            (_, Some(_), Some(_)) => panic!("Unexpected expression with two known branches: {:?}", expression),
            (_, None, None) => panic!("Unexpected expression with no known branches: {:?}", expression)
        };

        expression_name = match (av, bv, a, b) {
            (Some(_), None, _, Value::Variable(name)) => name,
            (None, Some(_), Value::Variable(name), _) => name,
            _ => unreachable!()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(152, part1(&parse(include_str!("../input/2022/day21.part1.test.152.txt"))).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(56490240862410, part1(&parse(include_str!("../input/2022/day21.txt"))).unwrap());
    }

    #[test]
    fn part2_example1() {
        assert_eq!(301, part2(&parse(include_str!("../input/2022/day21.part2.test.301.txt"))).unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!(3403989691757, part2(&parse(include_str!("../input/2022/day21.txt"))).unwrap());
    }
}