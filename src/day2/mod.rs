use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs.iter().map(|l| l.chars().collect_vec()).collect_vec();
	let inputs = inputs.iter().map(|l| (l[0], l[2])).collect_vec();

	let mut score = 0;
	let mut score2 = 0;

	for input in inputs {
		match input.1 {
			'X' => score += 1,
			'Y' => score += 2,
			'Z' => score += 3,
			_ => (),
		};

		match input.0 {
			'A' => { // rock
				match input.1 {
					'X' => score += 3,
					'Y' => score += 6,
					_ => (),
				}
			},
			'B' => { // paper
				match input.1 {
					'Y' => score += 3,
					'Z' => score += 6,
					_ => (),
				}
			},
			'C' => { // scissor
				match input.1 {
					'X' => score += 6,
					'Z' => score += 3,
					_ => (),
				}
			},
			_ => (),
		};

		match input.0 {
			'A' => { // rock
				match input.1 {
					'X' => score2 += 3 + 0, // scissor, lose
					'Y' => score2 += 1 + 3, // rock, tie
					'Z' => score2 += 2 + 6, // paper, win
					_ => (),
				}
			},
			'B' => { // paper
				match input.1 {
					'X' => score2 += 1 + 0, // rock, lose
					'Y' => score2 += 2 + 3, // paper, tie
					'Z' => score2 += 3 + 6, // scissor, win
					_ => (),
				}
			},
			'C' => { // scissor
				match input.1 {
					'X' => score2 += 2 + 0, // paper, lose
					'Y' => score2 += 3 + 3, // scissor, tie
					'Z' => score2 += 1 + 6, // rock, win
					_ => (),
				}
			},
			_ => (),
		};

	}
	println!("Part 1: {}", score);
	println!("Part 2: {}", score2);
}
