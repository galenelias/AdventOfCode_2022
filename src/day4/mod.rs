use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	let mut part2 = 0;

	for input in inputs {
		let p = input.split(&[',','-']).map(|s| s.parse::<u32>().unwrap()).collect_vec();

		if (p[0] >= p[2] && p[1] <= p[3]) || (p[2] >= p[0] && p[3] <= p[1]) {
			part1 += 1;
		}

		if !(p[0] > p[3] || p[2] > p[1]) {
			part2 += 1;
		}
	}
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
