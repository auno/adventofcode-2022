use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type CaveMap = (HashMap<String, usize>, HashMap<String, Vec<String>>);

#[aoc_generator(day16)]
fn parse(input: &str) -> CaveMap {
    let pattern = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
    let input = input
        .lines()
        .map(|line| {
            let c = pattern.captures(line).unwrap();

            (
                c.get(1).unwrap().as_str().to_string(),
                c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                c.get(3).unwrap().as_str().to_string(),
            )
        })
        .collect::<Vec<_>>();

    let flow_rates = input
        .iter()
        .map(|(valve, flow_rate, _)| {
            (valve.clone(), *flow_rate)
        })
        .collect();

    let tunnels = input
        .iter()
        .map(|(valve, _, neighbors)| {
            (valve.clone(), neighbors.split(", ").map(&str::to_string).collect())
        })
        .collect();

    (flow_rates, tunnels)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
struct State {
    location: String,
    opened_valves: Vec<String>,
    current_flow: usize,
    cumulative_flow: usize,
}

impl State {
    fn new(location: String, opened_valves: Vec<String>, current_flow: usize, cumulative_flow: usize) -> Self {
        Self { location, opened_valves, current_flow, cumulative_flow }
    }

    fn neighbors(&self, (flow_rates, _tunnels): &CaveMap, tunnel_distances: &HashMap<(String, String), usize>) -> impl IntoIterator<Item=(State, usize)> {
        let mut neighbors = vec![];

        for (valve, flow) in flow_rates {
            if *flow == 0 || *valve == self.location || self.opened_valves.contains(valve) {
                continue;
            }

            let distance = tunnel_distances[&(self.location.clone(), valve.clone())] + 1;
            let mut neighbor = self.clone();
            neighbor.location = valve.clone();
            neighbor.opened_valves.push(valve.clone());
            neighbor.opened_valves.sort();
            neighbor.cumulative_flow += neighbor.current_flow * distance;
            neighbor.current_flow += flow;
            neighbors.push((neighbor, distance));
        }

        neighbors
    }
}

fn tunnel_distances((_, tunnels): &CaveMap) -> HashMap<(String, String), usize> {
    tunnels
        .keys()
        .tuple_combinations()
        .map(|(source, target)| {
            let mut distances: HashMap<&str, usize> = HashMap::new();
            let mut queue: BinaryHeap<(Reverse<usize>, &str)> = BinaryHeap::new();

            distances.insert(source, 0);
            queue.push((Reverse(0), source));

            while let Some((Reverse(distance), position)) = queue.pop() {
                if position == *target {
                    break;
                }

                for neighbor in &tunnels[&position.to_string()] {
                    let neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

                    if *neighbor_distance > distance + 1 {
                        *neighbor_distance = distance + 1;
                        queue.push((Reverse(*neighbor_distance), neighbor));
                    }
                }
            }

            // let foo = distances.get(target).copied().unwrap();
            ((source.to_string(), target.to_string()), distances[target.as_str()])
        })
        .flat_map(|((source, target), d)| vec![
            ((source.clone(), target.clone()), d),
            ((target, source), d),
        ])
        .collect()
}

#[aoc(day16, part1)]
fn part1(input: &CaveMap) -> usize {
    let tunnel_distances = tunnel_distances(input);
    let mut state_distances: HashMap<State, usize> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<usize>, State)> = BinaryHeap::new();

    let source = State::new("AA".to_string(), vec![], 0, 0);
    state_distances.insert(source.clone(), 0);
    queue.push((Reverse(0), source));

    while let Some((Reverse(distance), state)) = queue.pop() {
        for (neighbor, d) in state.neighbors(input, &tunnel_distances) {
            let neighbor_distance = state_distances.entry(neighbor.clone()).or_insert(usize::MAX);

            if *neighbor_distance > distance + d && distance + d <= 30 {
                *neighbor_distance = distance + d;
                queue.push((Reverse(*neighbor_distance), neighbor));
            }
        }
    }

    state_distances
        .iter()
        .filter(|(_, distance)| **distance <= 30)
        .map(|(state, distance)| state.cumulative_flow + (30 - *distance) * state.current_flow)
        .sorted()
        .rev()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(1651, part1(&parse(include_str!("../input/2022/day16.part1.test.1651.txt"))));
    }
}