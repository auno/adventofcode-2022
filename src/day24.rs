use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}

#[aoc_generator(day24)]
fn parse(input: &str) -> (HashSet<(i32, i32, i32)>, (i32, i32), (i32, i32), i32) {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let period = lcm(width - 2, height - 2);

    let mut map: HashSet<(i32, i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (x as i32, y as i32, c))
        })
        .flat_map(|(x, y, c)| {
            match c {
                '#' => (0..period).map(|z| (x, y, z)).collect::<Vec<_>>(),
                '<' => (0..period).map(|z| ((x - 1 - z).rem_euclid(width - 2) + 1, y, z)).collect::<Vec<_>>(),
                '>' => (0..period).map(|z| ((x - 1 + z).rem_euclid(width - 2) + 1, y, z)).collect::<Vec<_>>(),
                '^' => (0..period).map(|z| (x, (y - 1 - z).rem_euclid(height - 2) + 1, z)).collect::<Vec<_>>(),
                'v' => (0..period).map(|z| (x, (y - 1 + z).rem_euclid(height - 2) + 1, z)).collect::<Vec<_>>(),
                _ => panic!(),
            }
        })
        .collect();

    let source = (input.lines().next().unwrap().chars().position(|c| c == '.').unwrap() as i32, 0);
    let target = (input.lines().last().unwrap().chars().position(|c| c == '.').unwrap() as i32, height - 1);

    for z in 0..period {
        map.insert((source.0, source.1 - 1, z));
        map.insert((target.0, target.1 + 1, z));
    }

    (map, source, target, period)
}

fn neighbors(map: &HashSet<(i32, i32, i32)>, (x, y, z): (i32, i32, i32), period: i32) -> impl IntoIterator<Item=(i32, i32, i32)> {
    let candidates = [
        (x - 1, y, (z + 1) % period),
        (x + 1, y, (z + 1) % period),
        (x, y - 1, (z + 1) % period),
        (x, y + 1, (z + 1) % period),
        (x, y, (z + 1) % period),
    ];

    candidates
        .into_iter()
        .filter(|p| !map.contains(p))
        .collect::<Vec<_>>()
}

fn distance(map: &HashSet<(i32, i32, i32)>, source: (i32, i32), target: (i32, i32), period: i32) -> Option<usize> {
    let mut distances: HashMap<(i32, i32, i32), usize> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<usize>, (i32, i32, i32))> = BinaryHeap::new();
    let source = (source.0, source.1, 0);

    distances.insert(source, 0);
    queue.push((Reverse(0), source));

    while let Some((Reverse(distance), position)) = queue.pop() {
        if (position.0, position.1) == target {
            return Some(distance);
        }

        for neighbor in neighbors(map, position, period) {
            let neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

            if *neighbor_distance > distance + 1 {
                *neighbor_distance = distance + 1;
                queue.push((Reverse(*neighbor_distance), neighbor));
            }
        }
    }

    None
}

#[aoc(day24, part1)]
fn part1((map, source, target, period): &(HashSet<(i32, i32, i32)>, (i32, i32), (i32, i32), i32)) -> usize {
    distance(map, *source, *target, *period).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(18, part1(&parse(include_str!("../input/2022/day24.part1.test.18.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(269, part1(&parse(include_str!("../input/2022/day24.txt"))));
    }
}