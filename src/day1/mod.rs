use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs.iter().map(|line| line.parse::<u32>().unwrap()).collect_vec();

	let part1 = inputs.iter().sum::<u32>();
	println!("Part 1: {}", part1);
}