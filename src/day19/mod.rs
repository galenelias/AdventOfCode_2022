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

#[derive(Debug)]
struct RobotBluePrint
{
	robot_type: ResourceType,
	cost: Resources,
}

#[derive(Debug)]
struct BluePrint
{
	num: usize,
	robot_blueprints: Vec<RobotBluePrint>,
}

type Memo = HashMap<(usize, Resources, Resources), usize>;

const ALL_RESOURCES: [ResourceType; 4] = [ResourceType::Ore, ResourceType::Clay, ResourceType::Obsidian, ResourceType::Geode];

// Return number of days until we can afford the specific blueprint
fn when_afford(resources: &Resources, cost: &Resources, robots: &Resources) -> Option<usize> {
	// If we will never afford the blueprint, return None
	if ALL_RESOURCES.iter().any(|r| resources.get(r) < cost.get(r) && robots.get(r) == 0) {
		return None;
	}

	Some(ALL_RESOURCES.iter().map(|r| {
		if cost.get(r) > resources.get(r) {
			(cost.get(r) - resources.get(r) + robots.get(r) - 1) / robots.get(r)
		} else {
			0
		}
	}).max().unwrap())
}

fn recurse(minute: usize, mut resources: Resources, mut robots: Resources, blueprints: &[RobotBluePrint], max_resources_needed: &Resources, memo: &mut Memo) -> usize {
	const TARGET_MINUTES: usize = 32;

	if minute > TARGET_MINUTES {
		return 0; 
	} else if minute == TARGET_MINUTES {
		return resources.get(&ResourceType::Geode);
	}



	for resource in &[ResourceType::Ore, ResourceType::Clay, ResourceType::Obsidian] {
		if robots.get(&resource) > max_resources_needed.get(&resource) {
			*robots.entry(&resource) = max_resources_needed.get(&resource);
		}
	}

	// If we have more resources than we need, cap them to increase memoization hit rate
	for resource in &[ResourceType::Ore, ResourceType::Clay, ResourceType::Obsidian] {
		if robots.get(&resource) == max_resources_needed.get(&resource) && resources.get(&resource) > max_resources_needed.get(&resource) {
			*resources.entry(&resource) = max_resources_needed.get(&resource);
		}
	}

	if let Some(result) = memo.get(&(minute, resources, robots)) {
		return *result;
	}

	let mut results = Vec::new();

	// Assume we only build one blueprint per minute (seems right?)
	for blueprint in blueprints {

		// Don't build more of a type of robot if we are already producing more of that resource than we can spend per minute
		if blueprint.robot_type != ResourceType::Geode && robots.get(&blueprint.robot_type) >= max_resources_needed.get(&blueprint.robot_type) {
			continue;
		}

		if let Some(days_to_afford) = when_afford(&resources, &blueprint.cost, &robots) {
			// println!("Can afford {:?} in {} days", blueprint.robot_type, days_to_afford);
			let mut new_resources = resources.clone();

			// Add days_to_afford worth of resources
			for resource in &ALL_RESOURCES {
				*new_resources.entry(&resource) += (1 + days_to_afford) * robots.get(&resource);
			}

			// Now pay for the robot
			for resource in &ALL_RESOURCES {
				*new_resources.entry(&resource) -= blueprint.cost.get(&resource);
			}

			let mut new_robots = robots.clone();
			*new_robots.entry(&blueprint.robot_type) += 1;
			results.push(recurse(minute + days_to_afford + 1, new_resources, new_robots, blueprints, max_resources_needed, memo));
		}
	}

	// Don't try to build anything, just in case waiting to afford something more would put us past TARGET_MINUTES
	{
		let mut new_resources = resources.clone();

		// Add days_to_afford worth of resources
		for resource in &ALL_RESOURCES {
			*new_resources.entry(&resource) += (TARGET_MINUTES - minute) * robots.get(&resource);
		}
		results.push(recurse(TARGET_MINUTES, new_resources, robots, blueprints, max_resources_needed, memo)); //no-op
	}

	let result = results.iter().max().unwrap().clone();

	memo.insert((minute, resources, robots), result);

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
			let mut cost = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
			for resource in caps[2].split(" and ") {
				let (amt, rtype) = resource.split_once(' ').unwrap();
				let resource_type = ResourceType::from_str(rtype);
				let amount = amt.parse::<usize>().unwrap();
				*cost.entry(&resource_type) += amount;
			}
			let sub_blueprint = RobotBluePrint{ robot_type: ResourceType::from_str(&caps[1]), cost};
			blueprint.robot_blueprints.push(sub_blueprint);
		}
		blueprints.push(blueprint);
	}

	let mut part1 = 0;

	let no_resources = Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 };
	let initial_robots = Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 };
	// Simulate each blueprint
	for blueprint in &blueprints[0..] {

		let mut max_resources_needed = Resources{ ore: 0, clay: 0, obsidian: 0, geode: 0 };
		// Cap resources at max needed for any robot
		for resource in &ALL_RESOURCES {
			let entry = max_resources_needed.entry(&resource);
			*entry = std::cmp::max(*entry, blueprint.robot_blueprints.iter().map(|b| b.cost.get(&resource)).max().unwrap());
		}
		println!("Max cost: {:?}", max_resources_needed);
		
		let mut memo = Memo::new();
		let geodes = recurse(0, no_resources, initial_robots, &blueprint.robot_blueprints, &max_resources_needed, &mut memo);
		println!("Blueprint {}: {}", blueprint.num, geodes);
		println!("Memo size = {}", memo.len());

		part1 += blueprint.num * geodes;
	}

	println!("Part 1: {}", part1);
}