use std::cmp::max;
use std::collections::{HashMap, HashSet};
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

fn step<'a>(movements: &mut impl Iterator<Item=&'a Movement>, height: usize, occupied: &mut HashSet<(usize, usize)>, rock: &[(usize, usize)]) -> usize {
    let mut height = height;
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

    height
}

fn update_hash(occupied: &HashSet<(usize, usize)>, height: &usize, previous_hash: u128) -> u128 {
    let mut hash = previous_hash;
    let y = height - 1;

    for x in 0..7 {
        let bit = match occupied.get(&(x, y)) {
            None => 0,
            Some(_) => 1,
        };

        hash <<= 1;
        hash |= bit;
    }

    hash
}

#[aoc(day17, part1)]
fn part1(input: &[Movement]) -> usize {
    let mut movements = input.iter().cycle();
    let rocks = ROCK_SHAPES.iter().cycle().take(2022);

    let mut height = 0;
    let mut occupied = HashSet::new();

    for rock in rocks {
        height = step(&mut movements, height, &mut occupied, rock);
    }

    height
}

#[aoc(day17, part2)]
fn part2(input: &[Movement]) -> usize {
    let mut movements = input.iter().cycle();
    let mut rocks = ROCK_SHAPES.iter().cycle();

    let mut heights = vec![0];
    let mut occupied = HashSet::new();
    let mut hashes = HashMap::new();
    let mut hash = 0;

    let (base, period) = loop {
        let rock = rocks.next().unwrap();
        let height = step(&mut movements, *heights.last().unwrap(), &mut occupied, rock);
        heights.push(height);
        hash = update_hash(&occupied, &height, hash);

        if let Some(&prev) = hashes.get(&hash) {
            break (prev, heights.len() - 1 - prev);
        }

        hashes.insert(hash, heights.len() - 1);
    };

    let num_cycles = (1000000000000 - base) / period;
    let remainder = (1000000000000 - base) % period;

    heights[base] + num_cycles * (heights[base + period] - heights[base]) + (heights[base + remainder] - heights[base])
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

    #[test]
    fn part2_example1() {
        assert_eq!(1514285714288, part2(&parse(include_str!("../input/2022/day17.part2.test.1514285714288.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1553665689155, part2(&parse(include_str!("../input/2022/day17.txt")).unwrap()));
    }
}