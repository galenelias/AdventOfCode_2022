use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Resources
{
	ore: usize,
	clay: usize,
	obsidian: usize,
	geode: usize,
}

impl Resources {
	fn get(&self, resource_type: &ResourceType) -> usize {
		match resource_type {
			ResourceType::Ore => self.ore,
			ResourceType::Clay => self.clay,
			ResourceType::Obsidian => self.obsidian,
			ResourceType::Geode => self.geode,
		}
	}

	fn entry(&mut self, resource_type: &ResourceType) -> &mut usize {
		match resource_type {
			ResourceType::Ore => &mut self.ore,
			ResourceType::Clay => &mut self.clay,
			ResourceType::Obsidian => &mut self.obsidian,
			ResourceType::Geode => &mut self.geode,
		}
	}
}
// type Resources = HashMap<ResourceType, usize>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum ResourceType
{
	Ore,
	Clay,
	Obsidian,
	Geode,
}

impl ResourceType
{
	fn from_str(s: &str) -> ResourceType {
		match s {
			"ore" => ResourceType::Ore,
			"geode" => ResourceType::Geode,
			"clay" => ResourceType::Clay,
			"obsidian" => ResourceType::Obsidian,
			_ => panic!("Unknown robot type: {}", s),
		}
	}
}

#[derive(Debug, Copy, Clone)]
struct Resource
{
	resource_type: ResourceType,
	amount: usize,
}

#[derive(Debug)]
struct RobotBluePrint
{
	robot_type: ResourceType,
	cost: Vec<Resource>,
}

#[derive(Debug)]
struct BluePrint
{
	num: usize,
	robot_blueprints: Vec<RobotBluePrint>,
}


type Memo = HashMap<(usize, Resources, Resources), usize>;

fn recurse(minute: usize, orig_resources: Resources, robots: Resources, blueprints: &[RobotBluePrint], memo: &mut Memo) -> usize {
	// Each day, each robot produces one of their resource types
	// BFS/DFS on building each sub-type of robot...
	// State: Day, Resources, Robots

	if minute == 24 {
		// println!("Finished with {:?}", orig_resources);
		return orig_resources.get(&ResourceType::Geode);
	}

	if let Some(result) = memo.get(&(minute, orig_resources, robots)) {
		// println!("Found memo result! minute {}", minute);
		return *result;
	}

	let mut resources = orig_resources.clone();
	for resource in &[ResourceType::Ore, ResourceType::Clay, ResourceType::Obsidian, ResourceType::Geode] {
		*resources.entry(&resource) += robots.get(&resource);
	}

	let mut results = Vec::new();

	// Assume we only build one blueprint per minute (seems right?)
	for blueprint in blueprints {
		let can_afford = blueprint.cost.iter().all(|r| orig_resources.get(&r.resource_type) >= r.amount);

		if can_afford {
			// println!("Constructing {:?} machine at minute {}", blueprint.robot_type, minute+1);
			let mut new_resources = resources.clone();
			for resource in &blueprint.cost {
				*new_resources.entry(&resource.resource_type) -= resource.amount;
			}
			let mut new_robots = robots.clone();
			*new_robots.entry(&blueprint.robot_type) += 1;
			results.push(recurse(minute + 1, new_resources, new_robots, blueprints, memo));
		}
	}

	results.push(recurse(minute + 1, resources, robots, blueprints, memo)); //no-op

	let result = results.iter().max().unwrap().clone();

	memo.insert((minute, orig_resources, robots), result);

	return result;
}

pub fn solve(inputs: Vec<String>) {
	let re_input = Regex::new(r"Each (\w+) robot costs (.*)").unwrap();

	let mut blueprints = Vec::new();
	let mut blueprint_counter = 1;

	for input in &inputs {
		let parts = input.split(": ").collect_vec();
		let robot_directions = parts[1].split(".").map(|s| s.trim()).filter(|s| !s.is_empty()).collect_vec();

		let mut blueprint = BluePrint{ num: blueprint_counter, robot_blueprints: Vec::new() };
		blueprint_counter += 1;
		for robot in &robot_directions {
			let caps = re_input.captures(&robot).unwrap();
			let mut sub_blueprint = RobotBluePrint{ robot_type: ResourceType::from_str(&caps[1]), cost: Vec::new()};
			for resource in caps[2].split(" and ") {
				let (amt, rtype) = resource.split_once(' ').unwrap();
				let resource_type = ResourceType::from_str(rtype);
				let amount = amt.parse::<usize>().unwrap();
				sub_blueprint.cost.push(Resource{ resource_type, amount });
			}
			blueprint.robot_blueprints.push(sub_blueprint);
		}
		blueprints.push(blueprint);
	}

	let mut part1 = 0;

	let no_resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
	let initial_robots = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
	// Simulate each blueprint
	for blueprint in &blueprints {
		let mut memo = Memo::new();
		let geodes = recurse(0, no_resources, initial_robots, &blueprint.robot_blueprints, &mut memo);
		println!("Blueprint {}: {}", blueprint.num, geodes);

		part1 += blueprint.num * geodes;
	}

	println!("Part 1: {}", part1);
}