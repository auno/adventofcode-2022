use std::cmp::max;
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use rayon::prelude::*;

const GEODE: usize = 3;
const GEODE_BOT: usize = 7;

type Inventory = [i32; 8];
type Blueprint = [Inventory; 5];

#[aoc_generator(day19)]
fn parse(input: &str) -> Vec<Blueprint> {
    let pattern = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    input
        .lines()
        .map(|line| {
            let c = pattern.captures(line).unwrap();
            let c = (1..=6).map(|i| c[i].parse::<i32>().unwrap()).collect::<Vec<_>>();
            [
                [-c[0],     0,     0, 0, 1, 0, 0, 0],
                [-c[1],     0,     0, 0, 0, 1, 0, 0],
                [-c[2], -c[3],     0, 0, 0, 0, 1, 0],
                [-c[4],     0, -c[5], 0, 0, 0, 0, 1],
                [    0,     0,     0, 0, 0, 0, 0, 0],
            ]
        })
        .collect()
}

fn solve(cache: &mut HashMap<(i32, Inventory), i32>, best: i32, time: i32, inventory: Inventory, blueprint: &Blueprint) -> i32 {
    if let Some(x) = cache.get(&(time, inventory)) {
        return *x;
    }

    if time == 0 {
        return inventory[GEODE];
    }

    let upper_bound = inventory[GEODE] + inventory[GEODE_BOT] * time + (time * time + time) / 2;

    if upper_bound <= best {
        return best;
    }

    let max = blueprint
        .iter()
        .fold(best, |acc, recipe| {
            let mut next_inventory: Inventory = inventory
                .iter()
                .zip(recipe.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            if next_inventory.iter().any(|&a| a < 0) {
                return acc;
            }

            for i in 0..4 {
                next_inventory[i] += inventory[i + 4];
            }

            max(acc, solve(cache, acc, time - 1, next_inventory, blueprint))
        });

    cache.insert((time, inventory), max);
    max
}

#[aoc(day19, part1)]
fn part1(input: &[Blueprint]) -> i32 {
    input
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let mut cache = HashMap::new();
            (i as i32 + 1) * solve(&mut cache, 0, 24, [0, 0, 0, 0, 1, 0, 0, 0], blueprint)
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &[Blueprint]) -> i32 {
    input
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let mut cache = HashMap::new();
            solve(&mut cache, 0, 32, [0, 0, 0, 0, 1, 0, 0, 0], blueprint)
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(33, part1(&parse(include_str!("../input/2022/day19.part1.test.33.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(3472, part2(&parse(include_str!("../input/2022/day19.part2.test.3472.txt"))));
    }
}