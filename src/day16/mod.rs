use std::{
	cmp::Ordering,
	collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Valve {
	flow_rate: usize,
	tunnels: Vec<usize>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Agent {
	position: usize,
	elapsed: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
	activated_valves: Vec<usize>,
	pressure_released: usize,
	total_unreleased_flow: usize,
	agents: Vec<Agent>
}

impl State {
	fn min_elapsed(&self) -> usize {
		self.agents.iter().map(|a| a.elapsed).min().unwrap()
	}

	// Heuristic for the priority queue: released pressure + remaining flow * remaining time
	fn possible_pressure(&self) -> usize {
		self.pressure_released + if self.min_elapsed() < 28 { self.total_unreleased_flow * (30 - 2 - self.min_elapsed()) } else { 0 }
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.possible_pressure().cmp(&(other.possible_pressure()))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn sub_solve(valves: &Vec<Valve>, nodes: &Vec<usize>, distances: &Vec<Vec<usize>>, starting_valve: usize, agents: usize, start_elapsed: usize) -> usize {
	let total_flow = valves.iter().map(|v| v.flow_rate).sum::<usize>();
	let mut queue = BinaryHeap::new();

	queue.push(State {
		activated_valves: vec![],
		pressure_released: 0,
		total_unreleased_flow: total_flow,
		agents: vec![Agent { position: starting_valve, elapsed: start_elapsed}; agents]
	});

	let mut result = 0;
	while !queue.is_empty() {
		let node = queue.pop().unwrap();

		let mut progressed = false;

		// Move the 'oldest' agent first, so that we can prune the search space as quickly as possible
		let mut agents_by_elapsed = node.agents.iter().enumerate().collect_vec();
		agents_by_elapsed.sort_by_key(|(_i, a)| a.elapsed);

		for (a, agent) in &agents_by_elapsed {
			for next_valve in nodes {
				let time_taken = distances[agent.position][*next_valve];
				if agent.elapsed + time_taken + 1 >= 30 || node.activated_valves.contains(next_valve) {
					continue;
				}

				let mut new_node = node.clone();
				new_node.activated_valves.push(*next_valve);
				new_node.agents[*a].elapsed += time_taken + 1;
				new_node.pressure_released += (30 - new_node.agents[*a].elapsed) * valves[*next_valve].flow_rate;
				new_node.total_unreleased_flow -= valves[*next_valve].flow_rate;
				new_node.agents[*a].position = *next_valve;

				queue.push(new_node);
				progressed = true;
			}

			// If we moved the earliest agent, then stop there, as we want the minimumm elapsed elapsed for the priority queue heuristic to converge as quickly as possible
			// But we might need to keep going if some old agents can't move, but the later agents can
			if progressed {
				break;
			}
		}

		if !progressed && node.pressure_released > result {
			result = node.pressure_released;
		} else if !progressed  && node.possible_pressure() < result{
			// Ran out of potentially viable paths, so we must have the optimal answer!
			break;
		} 
	}

	return result;
}

pub fn solve(inputs: Vec<String>) {
	let re_input = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

	// For the solution, we'll use the valve index instead of the name, so come up with the conversion map first
	let name_to_index = inputs
		.iter()
		.enumerate()
		.map(|(i, s)| {
			let caps = re_input.captures(s).unwrap();
			(caps[1].to_string(), i)
		})
		.collect::<HashMap<String, usize>>();

	let valves = inputs.iter().map(|input| {
		let caps = re_input.captures(&input).unwrap();
		Valve {
			flow_rate: caps[2].parse::<usize>().unwrap(),
			tunnels: caps[3]
				.split(", ")
				.map(|s| name_to_index[s])
				.collect_vec(),
		}
	}).collect_vec();

	// Set up distance matrix
	let mut distances = vec![vec![usize::MAX; valves.len()]; valves.len()];
	for (i, valve) in valves.iter().enumerate() {
		for tunnel in &valve.tunnels {
			distances[i][*tunnel] = 1;
		}
	}

	let valve_num = valves.len();
	for i in 0..valve_num {
		distances[i][i] = 0;
	}

	// Floyd-Warshall - compute all-pairs shortest paths
	for k in 0..valve_num {
		for i in 0..valve_num {
			for j in 0..valve_num {
				if distances[i][k] != usize::MAX && distances[k][j] != usize::MAX {
					distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
				}
			}
		}
	}

	// The only 'nodes' we traverse to are valves with flow rate, as there is no point in visiting the other valves
	let nodes = valves
		.iter()
		.enumerate()
		.filter(|(_i, v)| v.flow_rate > 0)
		.map(|(i, _v)| i)
		.collect_vec();

	println!("Part 1: {}", sub_solve(&valves, &nodes, &distances, name_to_index["AA"], 1 /* agent */, 0 /* initial elapsed */));
	println!("Part 2: {}", sub_solve(&valves, &nodes, &distances, name_to_index["AA"], 2 /* agent */, 4 /* initial elapsed */));
}
