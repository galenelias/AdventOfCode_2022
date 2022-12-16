use std::{collections::{HashMap, HashSet, VecDeque, BinaryHeap}, cmp::Ordering};

use itertools::Itertools;
use regex::Regex;

fn valve_to_num(s: &str) -> i32 {
	let chs = s.chars().collect_vec();
	(chs[0] as i32 - 'A' as i32 + 1) * 26 + chs[1] as i32 - 'A' as i32 + 1
}

fn num_to_valve(n: &i32) -> String {
	let mut s = String::new();
	s.push((n / 26 + 'A' as i32 - 1) as u8 as char);
	s.push((n % 26 + 'A' as i32 - 1) as u8 as char);
	s
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Valve {
	name: i32,
	flow_rate: i32,
	tunnels: Vec<i32>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
	activated_valves: Vec<i32>,
	pressure_released: i32,
	total_unreleased_flow: i32,
	room1: i32,
	room2: i32,
	elapsed: i32,
}

impl State {
	fn possible_pressure(&self) -> i32 {
		self.pressure_released + self.total_unreleased_flow * (30 - 1 - self.elapsed)
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State2 {
	activated_valves: Vec<i32>,
	pressure_released: i32,
	total_unreleased_flow: i32,
	room1: i32,
	room2: i32,
	elapsed: i32,
}

impl State2 {
	fn possible_pressure(&self) -> i32 {
		self.pressure_released + self.total_unreleased_flow * (30 - 1 - self.elapsed)
	}
}

impl Ord for State2 {
	fn cmp(&self, other: &Self) -> Ordering {
		self.possible_pressure().cmp(&(other.possible_pressure()))
	}
}

impl PartialOrd for State2 {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// struct SeenState {
// 	activated_valves: Vec<i32>,
// 	// pressure_released: i32,
// 	room: i32,
// }

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct SeenState {
	activated_valves: Vec<i32>,
	// pressure_released: i32,
	room1: i32,
	room2: i32,
}

fn part1(valves: &HashMap<i32, Valve>) {
	let total_flow = valves.values().map(|v| v.flow_rate).sum::<i32>();

	println!("Initial: total_flow = {}, potential = {}", total_flow, total_flow * 29);
	let initial = State{ activated_valves: Vec::new(), pressure_released: 0, room1: valve_to_num("AA"), room2: 0, total_unreleased_flow: total_flow, elapsed: 0 };
	// let mut queue = VecDeque::new();
	let mut queue = BinaryHeap::new();

	queue.push(initial);

	let mut seen = HashSet::new();

	let mut iterations: usize = 0;
	while !queue.is_empty() {
		let node = queue.pop().unwrap();

		if node.elapsed > 30 {
			continue;
		} else if node.elapsed == 30 {
			println!("Found end: {}", node.pressure_released);
			println!("Part 1: {}", node.pressure_released);
			break;
		}

		if !seen.insert(SeenState{ activated_valves: node.activated_valves.clone(), /*pressure_released: node.pressure_released,*/ room1: node.room1, room2: 0} ) {
			continue;
		}

		iterations += 1;
		if iterations % 50000 == 0 {
			println!("{}: {}, elapsed={}, opened_valves={}, pressure={}, potential={}, queue.len()={}", iterations, num_to_valve(&node.room1), node.elapsed, node.activated_valves.len(), node.pressure_released, node.possible_pressure(), queue.len());
		}

		if valves[&node.room1].flow_rate > 0 && !node.activated_valves.contains(&node.room1) {
			let mut new_node = node.clone();
			new_node.activated_valves.push(node.room1);
			new_node.pressure_released += (30 - 1 - node.elapsed) * valves[&node.room1].flow_rate;
			new_node.elapsed += 1;
			new_node.total_unreleased_flow -= valves[&node.room1].flow_rate;
			queue.push(new_node);
		}

		for tunnel in &valves[&node.room1].tunnels {
			// if !seen.contains(&SeenState{ activated_valves: node.activated_valves.clone(), room1: *tunnel, room2: 0} ) {
				let mut new_node = node.clone();
				new_node.room1 = tunnel.clone();
				new_node.elapsed += 1;
	
				queue.push(new_node);
			// }
		}
	}

}

fn part2(valves: &HashMap<i32, Valve>) {
	let total_flow = valves.values().map(|v| v.flow_rate).sum::<i32>();
	let initial = State{ activated_valves: Vec::new(), pressure_released: 0, room1: valve_to_num("AA"), room2: valve_to_num("AA"), total_unreleased_flow: total_flow, elapsed: 4 };
	// let mut queue = VecDeque::new();
	let mut queue = BinaryHeap::new();

	queue.push(initial);

	let mut seen = HashSet::new();

	let mut iterations: usize = 0;
	while !queue.is_empty() {
		let node = queue.pop().unwrap();

		if node.elapsed > 30 {
			continue;
		} else if node.elapsed == 30 {
			println!("Found end: {}", node.pressure_released);
			println!("Valves: {}", node.activated_valves.iter().map(num_to_valve).collect_vec().join(", "));
			println!("Part 2: {}", node.pressure_released);
			break;
		}

		if !seen.insert(SeenState{ activated_valves: node.activated_valves.clone(), /*pressure_released: node.pressure_released,*/ room1: node.room1, room2: node.room2} ) {
			continue;
		}

		iterations += 1;
		if iterations % 50000 == 0 {
			println!("{}: {}, elapsed={}, opened_valves={}, pressure={}, potential={}, queue.len()={}", iterations, num_to_valve(&node.room1), node.elapsed, node.activated_valves.len(), node.pressure_released, node.possible_pressure(), queue.len());
		}

		let activate_valve = |node: State, room| {
			let mut new_node = node.clone();
			new_node.activated_valves.push(room);
			new_node.pressure_released += (30 - 1 - node.elapsed) * valves[&room].flow_rate;
			// new_node.elapsed += 1;
			new_node.total_unreleased_flow -= valves[&room].flow_rate;
			return new_node;			
		};

		let mut try_move_elephant = |node: State| {
			if valves[&node.room2].flow_rate > 0 && !node.activated_valves.contains(&node.room2) {
				let mut activate_node = activate_valve(node.clone(), node.room2);
				activate_node.elapsed += 1;
				queue.push(activate_node);
			} else { // Don't travel if we activated the valve (sketchy optimization...)
				for tunnel in &valves[&node.room2].tunnels {
					// if !seen.contains(&SeenState{ activated_valves: node.activated_valves.clone(), room1: node.room1, room2: *tunnel} ) {
						let mut new_node = node.clone();
						new_node.room2 = *tunnel;
						new_node.elapsed += 1;
						queue.push(new_node);
					// }
				}
			}
		};

		// Check me move/activate
		if valves[&node.room1].flow_rate > 0 && !node.activated_valves.contains(&node.room1) {
			let activate_node = activate_valve(node.clone(), node.room1);

			try_move_elephant(activate_node);
		} else { // Don't travel if we activated the valve (sketchy optimization...)
			for tunnel in &valves[&node.room1].tunnels {
				let mut new_node = node.clone();
				new_node.room1 = *tunnel;

				try_move_elephant(new_node);
			}
		}

	}
}

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();

	let mut valves = HashMap::new();

	for input in inputs {
		let caps = re_input.captures(&input).unwrap();
		let valve = Valve { name: valve_to_num(&caps[1]), flow_rate: caps[2].parse::<i32>().unwrap(), tunnels: caps[3].split(", ").map(|s| s.trim()).map(valve_to_num).collect_vec()};
		valves.insert(valve.name.clone(), valve);
	}

	part1(&valves);
	part2(&valves);
}