use itertools::Itertools;

fn sub_solve(inputs: &[i64], multiplier: i64, loop_count: usize) -> i64 {
	// Store the (original index, value) pair so we can locate items based on their original index
	let mut vals = inputs
		.iter()
		.map(|x| x * multiplier)
		.enumerate()
		.collect_vec();

	for _ in 0..loop_count {
		for vi in 0..vals.len() {
			// Find the value based on the original index
			let (i, (_, v)) = vals
				.iter()
				.cloned()
				.enumerate()
				.find(|(_, (orig_i, _v))| orig_i == &vi)
				.unwrap();

			// Remove and re-insert it by adding 'v' (modulo the length of the list)
			vals.remove(i);
			let new_index = (i as i64 + v).rem_euclid(vals.len() as i64);
			vals.insert(new_index as usize, (vi, v));
		}
	}

	let zero_index = vals.iter().position(|(_, v)| *v == 0).unwrap();

	vals[(zero_index + 1000) % vals.len()].1
		+ vals[(zero_index + 2000) % vals.len()].1
		+ vals[(zero_index + 3000) % vals.len()].1
}

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs
		.iter()
		.map(|x| x.parse::<i64>().unwrap())
		.collect_vec();

	println!("Part 1: {}", sub_solve(&inputs, 1, 1));
	println!("Part 2: {}", sub_solve(&inputs, 811589153, 10));
}
