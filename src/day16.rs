use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type CaveMap = (Vec<usize>, Vec<Vec<usize>>, usize);

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

    let indices = input
        .iter()
        .sorted_by_key(|(_, flow, _)| Reverse(flow))
        .enumerate()
        .map(|(i, (valve, _, _))| {
            (valve.clone(), i)
        })
        .collect::<HashMap<_, _>>();

    let valves = input
        .iter()
        .map(|(valve, flow_rate, _)| {
            (indices[valve], *flow_rate)
        })
        .sorted_by_key(|(i, _)| *i)
        .map(|(_, flow)| flow)
        .collect::<Vec<_>>();

    let tunnels = input
        .iter()
        .map(|(valve, _, neighbors)| {
            (
                indices[valve],
                neighbors
                    .split(", ")
                    .map(&str::to_string)
                    .map(|n| indices[&n])
                    .collect()
            )
        })
        .sorted_by_key(|(i, _)| *i)
        .map(|(_, flow)| flow)
        .collect();

    for (i, flow) in valves.iter().enumerate() {
        assert!(i < 16 || *flow == 0);
    }

    (valves, tunnels, indices["AA"])
}

fn tunnel_distances((_, tunnels, _): &CaveMap) -> HashMap<(usize, usize), usize> {
    (0..tunnels.len())
        .tuple_combinations()
        .map(|(source, target)| {
            let mut distances: HashMap<usize, usize> = HashMap::new();
            let mut queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();

            distances.insert(source, 0);
            queue.push((Reverse(0), source));

            while let Some((Reverse(distance), position)) = queue.pop() {
                if position == target {
                    break;
                }

                for neighbor in &tunnels[position] {
                    let neighbor_distance = distances.entry(*neighbor).or_insert(usize::MAX);

                    if *neighbor_distance > distance + 1 {
                        *neighbor_distance = distance + 1;
                        queue.push((Reverse(*neighbor_distance), *neighbor));
                    }
                }
            }

            ((source, target), distances[&target])
        })
        .flat_map(|((source, target), d)| vec![
            ((source, target), d),
            ((target, source), d),
        ])
        .collect()
}

mod part1 {
    use super::*;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct State {
        location: usize,
        opened_valves: u16,
        current_flow: usize,
        cumulative_flow: usize,
    }

    impl State {
        fn new(location: usize, opened_valves: u16, current_flow: usize, cumulative_flow: usize) -> Self {
            Self { location, opened_valves, current_flow, cumulative_flow }
        }

        fn neighbors(&self, (valves, _, _): &CaveMap, tunnel_distances: &HashMap<(usize, usize), usize>) -> impl IntoIterator<Item=(State, usize)> {
            let mut neighbors = vec![];

            for (valve, flow) in valves.iter().enumerate() {
                let valve_mask = 1 << valve;

                if *flow == 0 || valve == self.location || self.opened_valves & valve_mask > 0 {
                    continue;
                }

                let distance = tunnel_distances[&(self.location, valve)] + 1;
                let mut neighbor = self.clone();
                neighbor.location = valve;
                neighbor.opened_valves |= valve_mask;
                neighbor.cumulative_flow += neighbor.current_flow * distance;
                neighbor.current_flow += flow;
                neighbors.push((neighbor, distance));
            }

            neighbors
        }
    }

    #[aoc(day16, part1)]
    pub fn part1(input: &CaveMap) -> usize {
        let &(_, _, start_location) = input;
        let tunnel_distances = tunnel_distances(input);
        let mut state_distances: HashMap<State, usize> = HashMap::new();
        let mut queue: BinaryHeap<(Reverse<usize>, State)> = BinaryHeap::new();

        let source = State::new(start_location, 0, 0, 0);
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
}

mod part2 {
    use super::*;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct Actor {
        location: usize,
        time: usize,
    }

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    struct State {
        opened_valves: u16,
        actors: [Actor; 2],
        cumulative_flow: usize,
    }

    impl State {
        fn neighbors(&self, (valves, _, _): &CaveMap, tunnel_distances: &HashMap<(usize, usize), usize>) -> impl IntoIterator<Item=State> {
            let mut neighbors = vec![];

            if self.actors[0].time >= 26 && self.actors[1].time >= 26 {
                return vec![];
            }

            let i = match (self.actors[0].time, self.actors[1].time) {
                (0..=25, 0) => 0,
                (26, 0..=25) => 1,
                (26, 26) => return vec![],
                _ => panic!("should not happen!"),
            };

            for (valve, flow) in valves.iter().enumerate() {
                let valve_mask = 1 << valve;

                if *flow == 0 || valve == self.actors[i].location || self.opened_valves & valve_mask > 0 {
                    continue;
                }

                let distance = tunnel_distances[&(self.actors[i].location, valve)] + 1;

                if self.actors[i].time + distance > 26 {
                    continue;
                }

                let mut neighbor = self.clone();
                neighbor.opened_valves |= valve_mask;
                neighbor.actors[i].location = valve;
                neighbor.actors[i].time += distance;
                neighbor.cumulative_flow += (26 - neighbor.actors[i].time) * flow;
                neighbors.push(neighbor);
            }

            let mut neighbor = self.clone();
            neighbor.actors[i].time = 26;
            neighbors.push(neighbor);

            neighbors
        }
    }

    #[aoc(day16, part2)]
    pub fn part2(input: &CaveMap) -> usize {
        let &(_, _, start_location) = input;
        let tunnel_distances = tunnel_distances(input);

        let mut state_distances: HashMap<(usize, usize, u16), State> = HashMap::new();
        let mut queue: BinaryHeap<(usize, State)> = BinaryHeap::new();

        let source = State {
            opened_valves: 0,
            actors: [
                Actor { location: start_location, time: 0 },
                Actor { location: start_location, time: 0 },
            ],
            cumulative_flow: 0,
        };
        state_distances.insert((0, 0, 0), source.clone());
        queue.push((0, source));

        while let Some((_cumulative_flow, state)) = queue.pop() {
            for neighbor in state.neighbors(input, &tunnel_distances) {
                let neighbor_id = (neighbor.actors[0].time, neighbor.actors[1].time, neighbor.opened_valves);
                let current_best = state_distances
                    .get(&neighbor_id)
                    .map(|state| state.cumulative_flow);

                if current_best.is_none() || neighbor.cumulative_flow > current_best.unwrap() {
                    state_distances.insert(neighbor_id, neighbor.clone());
                    queue.push((neighbor.cumulative_flow, neighbor));
                }
            }
        }

        state_distances
            .iter()
            .filter(|((a, b, _), _state)| *a == 26 && *b == 26)
            .map(|(_, state)| state.cumulative_flow)
            .sorted()
            .rev()
            .next()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(1651, part1::part1(&parse(include_str!("../input/2022/day16.part1.test.1651.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(1707, part2::part2(&parse(include_str!("../input/2022/day16.part2.test.1707.txt"))));
    }
}