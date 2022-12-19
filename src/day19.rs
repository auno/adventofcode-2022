use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use rayon::prelude::*;

const GEODE: usize = 3;

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

fn solve(cache: &mut HashMap<(u32, Inventory), i32>, blueprint: &Blueprint, time: u32, inventory: Inventory) -> i32 {
    if let Some(x) = cache.get(&(time, inventory)) {
        return *x;
    }

    if time == 0 {
        return inventory[GEODE];
    }

    let ans = blueprint
        .iter()
        .filter_map(|recipe| {
            let mut next_inventory: Inventory = inventory
                .iter()
                .zip(recipe.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            if next_inventory.iter().any(|&a| a < 0) {
                return None;
            }

            for i in 0..4 {
                next_inventory[i] += inventory[i + 4];
            }

            Some(solve(cache, blueprint, time - 1, next_inventory))
        })
        .max()
        .unwrap();

    cache.insert((time, inventory), ans);
    ans
}

#[aoc(day19, part1)]
fn part1(input: &[Blueprint]) -> i32 {
    input
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let mut cache = HashMap::new();
            (i as i32 + 1) * solve(&mut cache, blueprint, 24, [0, 0, 0, 0, 1, 0, 0, 0])
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(33, part1(&parse(include_str!("../input/2022/day19.part1.test.33.txt"))));
    }
}