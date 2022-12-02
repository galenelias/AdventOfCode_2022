use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs.iter().map(|l| l.chars().collect_vec()).collect_vec();
	let inputs = inputs.iter().map(|l| (l[0], l[2])).collect_vec();

	let part1: u32 = inputs.iter()
		.map(|input| match (input.0, input.1) {
			('A', 'X') => 1 + 3,
			('A', 'Y') => 2 + 6,
			('A', 'Z') => 3 + 0,
			('B', 'X') => 1 + 0,
			('B', 'Y') => 2 + 3,
			('B', 'Z') => 3 + 6,
			('C', 'X') => 1 + 6,
			('C', 'Y') => 2 + 0,
			('C', 'Z') => 3 + 3,
			_ => 0,
		})
		.sum();

	let part2: u32 = inputs.iter()
		.map(|input| match (input.0, input.1) {
			('A', 'X') => 3 + 0,
			('A', 'Y') => 1 + 3,
			('A', 'Z') => 2 + 6,
			('B', 'X') => 1 + 0,
			('B', 'Y') => 2 + 3,
			('B', 'Z') => 3 + 6,
			('C', 'X') => 2 + 0,
			('C', 'Y') => 3 + 3,
			('C', 'Z') => 1 + 6,
			_ => 0,
		})
		.sum();

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
