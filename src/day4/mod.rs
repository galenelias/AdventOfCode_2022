pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	let mut part2 = 0;

	for input in inputs {
		let ranges = input.split_once(',').unwrap();
		let r1 = ranges.0.split_once('-').unwrap();
		let r1s = r1.0.parse::<u32>().unwrap();
		let r1e = r1.1.parse::<u32>().unwrap();
		let r2 = ranges.1.split_once('-').unwrap();
		let r2s = r2.0.parse::<u32>().unwrap();
		let r2e = r2.1.parse::<u32>().unwrap();

		if (r1s >= r2s && r1e <= r2e) || (r2s >= r1s && r2e <= r1e) {
			part1 += 1;
		}

		if !(r1s > r2e || r2s > r1e) {
			part2 += 1;
		}
	}
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
