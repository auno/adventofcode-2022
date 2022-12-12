use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use anyhow::{Result, Context};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = ((i32, i32), (i32, i32), HashMap<(i32, i32), i32>);

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Input> {
    let width = input.lines().next().context("Incorrect input format")?.len() as i32;

    let (source, target, grid) = input
        .lines()
        .flat_map(&str::chars)
        .enumerate()
        .fold((None, None, HashMap::new()), |(source, target, mut grid), (i, c)| {
            let position = (i as i32 / width, i as i32 % width);
            let (source, target, c) = match c {
                'S' => (Some(position), target, 'a'),
                'E' => (source, Some(position), 'z'),
                _ => (source, target, c),
            };

            grid.insert(position, c as i32);
            (source, target, grid)
        });

    Ok((
        source.context("Did not find start position")?,
        target.context("Did not find end position")?,
        grid,
    ))
}

fn neighbors((i, j): (i32, i32), grid: &HashMap<(i32, i32), i32>) -> impl IntoIterator<Item=(i32, i32)> {
    let candidates = vec![
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
    ];

    candidates
        .into_iter()
        .filter(|candidate| grid.contains_key(candidate))
        .filter(|candidate| grid[candidate] <= grid[&(i, j)] + 1)
        .collect::<Vec<_>>()
}

fn distance(sources: &[(i32, i32)], target: &(i32, i32), grid: &HashMap<(i32, i32), i32>) -> Option<i32> {
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<i32>, (i32, i32))> = BinaryHeap::new();

    for source in sources {
        distances.insert(*source, 0);
        queue.push((Reverse(0), *source));
    }

    while let Some((Reverse(distance), position)) = queue.pop() {
        if position == *target {
            break;
        }

        for neighbor in neighbors(position, grid) {
            let neighbor_distance = distances.entry(neighbor).or_insert(i32::MAX);

            if *neighbor_distance > distance + 1 {
                *neighbor_distance = distance + 1;
                queue.push((Reverse(*neighbor_distance), neighbor));
            }
        }
    }

    distances.get(target).copied()
}

#[aoc(day12, part1)]
fn part1((source, target, grid): &Input) -> i32 {
    distance(&[*source], target, grid).unwrap()
}

#[aoc(day12, part2)]
fn part2((_, target, grid): &Input) -> i32 {
    let sources = grid
        .iter()
        .filter(|(_, elevation)| **elevation == 'a' as i32)
        .map(|(position, _)| position)
        .copied()
        .collect::<Vec<_>>();

    distance(&sources, target, grid).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(31, part1(&parse(include_str!("../input/2022/day12.part1.test.31.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(528, part1(&parse(include_str!("../input/2022/day12.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(29, part2(&parse(include_str!("../input/2022/day12.part2.test.29.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(522, part2(&parse(include_str!("../input/2022/day12.txt")).unwrap()));
    }
}