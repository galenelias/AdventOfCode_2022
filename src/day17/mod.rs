use std::collections::HashMap;
use itertools::Itertools;

fn get_rock(rock_type: u64) -> Vec<Vec<bool>> {
	match rock_type % 5{
		0 => { vec![ // ----
			vec![true, true, true, true]
		]},
		1 => { vec![ // +
			vec![false, true, false],
			vec![true, true, true],
			vec![false, true, false],
		]},
		2 => { vec![ // _|
			vec![false, false, true],
			vec![false, false, true],
			vec![true, true, true],
		]},
		3 => { vec![ // |
			vec![true],
			vec![true],
			vec![true],
			vec![true],
		]},
		4 => { vec![ // o
			vec![true, true],
			vec![true, true],
		]},

		_ => unreachable!(),
	}
}

fn does_collide(chamber: &Vec<[bool; 7]>, rock: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
	// y is offset of top edge of rock
	// x is offset of left edge of rock
	for (irow, row) in rock.iter().enumerate() {
		for (icol, col) in row.iter().enumerate() {
			if *col && y - irow < chamber.len() && chamber[y - irow][x + icol] {
				return true;
			}
		}
	}

	false
}

fn get_fingerprint(chamber: &Vec<[bool; 7]>) -> [u64; 7] {
	let mut fingerprint = [0; 7];
	for (irow, row) in chamber.iter().rev().enumerate().take(64) {
		for (icol, col) in row.iter().enumerate() {
			if *col {
				fingerprint[icol] |= 1 << irow;
			}
		}
	}

	fingerprint
}

fn sub_solve(input: &Vec<char>, mut target_iterations: u64) -> u64 {
	let mut chamber: Vec<[bool; 7]> = Vec::new();
	let mut input_char = 0;
	let mut history = HashMap::new();
	let mut irock: u64 = 0;
	let mut cycle_adjustment: u64 = 0;
	let mut found_cycle: bool = false;

	while target_iterations > 0 {

		if !found_cycle && chamber.len() > 64 {
			let fingerprint = get_fingerprint(&chamber);
			let state = (fingerprint, input_char, irock % 5);

			if let Some((prev_irock, prev_height)) = history.get(&state) {
				// Burn off iterations using the result.
				let cycle_period = irock - prev_irock;
				let remaining_cycles = target_iterations / cycle_period;
				target_iterations = target_iterations % cycle_period;
				cycle_adjustment = remaining_cycles * (chamber.len() - prev_height) as u64;
				found_cycle = true;
			} else {
				history.insert((fingerprint, input_char, irock % 5), (irock, chamber.len()));
			}
		}

		let rock = get_rock(irock);

		let mut x = 2;
		let mut y = chamber.len() + 3 + rock.len() - 1;

		loop {
			let dir = input[input_char];
			input_char = (input_char + 1) % input.len();

			if dir == '>' {
				if x + rock[0].len() < 7 {
					if !does_collide(&chamber, &rock, x + 1, y) {
						x += 1;
					}
				}
			} else if dir == '<' {
				if x > 0 {
					if !does_collide(&chamber, &rock, x - 1, y) {
						x -= 1;
					}
				}
			}
			
			// Now try to drop
			if y - (rock.len() - 1) > 0 && !does_collide(&chamber, &rock, x, y - 1) {
				y -= 1;
			} else {
				break;
			}
		}

		// Now add rock to chamber
		let new_height = y + 1;
		if new_height > chamber.len() {
			chamber.resize(new_height, [false; 7]);
		}

		for (irow, row) in rock.iter().enumerate() {
			for (icol, col) in row.iter().enumerate() {
				if *col {
					chamber[y - irow][x + icol] = true;
				}
			}
		}

		irock += 1;
		target_iterations -= 1;
	}	

	return chamber.len() as u64 + cycle_adjustment;
}

pub fn solve(inputs: Vec<String>) {
	let input = inputs[0].chars().collect_vec();

	println!("Part 1: {}", sub_solve(&input, 2022));
	println!("Part 2: {}", sub_solve(&input, 1000000000000));
}