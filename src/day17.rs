use std::cmp::max;
use std::collections::HashSet;
use std::iter;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Error, Result};

enum Movement {
    Left,
    Right,
}

impl TryFrom<char> for Movement {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '<' => Ok(Movement::Left),
            '>' => Ok(Movement::Right),
            _ => bail!("Unrecognized Movement: {}", value)
        }
    }
}

const ROCK_SHAPES: [&[(usize, usize)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
];

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<Vec<Movement>> {
    input
        .trim()
        .chars()
        .map(|c| c.try_into())
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &[Movement]) -> usize {
    let mut movements = iter::repeat(())
        .flat_map(|_| input);

    let mut height = 0;
    let mut occupied = HashSet::new();

    for rock in iter::repeat(()).flat_map(|_| ROCK_SHAPES).take(2022) {
        let (mut rx, mut ry) = (2, height + 3);

        loop {
            match movements.next().unwrap() {
                Movement::Left => {
                    if rock.iter().all(|&(x, y)| rx + x > 0 && !occupied.contains(&(rx + x - 1, ry + y))) {
                        rx -= 1;
                    }
                },
                Movement::Right => {
                    if rock.iter().all(|&(x, y)| rx + x < 6 && !occupied.contains(&(rx + x + 1, ry + y))) {
                        rx += 1;
                    }
                },
            }

            if ry == 0 || rock.iter().any(|&(x, y)| occupied.contains(&(rx + x, ry + y - 1))) {
                break
            }

            ry -= 1;
        }

        for &(x, y) in rock {
            occupied.insert((rx + x, ry + y));
            height = max(height, ry + y + 1);
        }
    }

    height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(3068, part1(&parse(include_str!("../input/2022/day17.part1.test.3068.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(3153, part1(&parse(include_str!("../input/2022/day17.txt")).unwrap()));
    }
}