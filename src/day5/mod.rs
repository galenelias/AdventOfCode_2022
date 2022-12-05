use itertools::Itertools;

fn sub_solve(mut stacks: Vec<Vec<char>>, inputs: &Vec<String>, move_multiple_crates: bool) -> String {
	for input in inputs {
		if !input.starts_with("move") {
			continue;
		}

		let parts = input.split_whitespace().collect_vec();
		let quantity = parts[1].parse::<usize>().unwrap();
		let from = parts[3].parse::<usize>().unwrap() - 1;
		let to = parts[5].parse::<usize>().unwrap() - 1;

		// Move 'quantity' items from stacks[from] to stacks[to]
		let stack_from_len = stacks[from].len();
		let mut items = stacks[from].drain(stack_from_len - quantity..).collect_vec();

		if !move_multiple_crates {
			items.reverse();
		}

		stacks[to].append(&mut items);
	}

	return stacks.iter().map(|s| s.last().unwrap()).join("");
}

pub fn solve(inputs: Vec<String>) {
	let num_stacks = (inputs[0].len() + 1) / 4;
	let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

	for line in &inputs {
		if line.starts_with(" 1") {
			break;
		}

		for (i, c) in line.chars().enumerate() {
			if c.is_ascii_alphabetic() {
				stacks[i / 4].push(c);
			}
		}
	}

	// Puzzle input lists top to bottom, so need to reverse the stacks now
	for stack in &mut stacks {
		stack.reverse();
	}

	println!("Part 1: {}", sub_solve(stacks.clone(), &inputs, false));
	println!("Part 2: {}", sub_solve(stacks, &inputs, true));
}
