use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
	North,
	South,
	West,
	East,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum ElfAction {
	Noop,
	MoveTo((i32, i32)),
}

fn apply_direction(position: &(i32, i32), direction: &Direction) -> (i32, i32) {
	match direction {
		Direction::North => (position.0 - 1, position.1),
		Direction::South => (position.0 + 1, position.1),
		Direction::West => (position.0, position.1 - 1),
		Direction::East => (position.0, position.1 + 1),
	}
}

fn get_adjacent_positions(position: &(i32, i32)) -> Vec<(i32, i32)> {
	let mut positions = Vec::new();

	positions.push((position.0 - 1, position.1 - 1));
	positions.push((position.0 - 1, position.1));
	positions.push((position.0 - 1, position.1 + 1));
	positions.push((position.0, position.1 - 1));
	positions.push((position.0, position.1 + 1));
	positions.push((position.0 + 1, position.1 - 1));
	positions.push((position.0 + 1, position.1));
	positions.push((position.0 + 1, position.1 + 1));

	positions
}

fn get_points_in_direction(position: &(i32, i32), direction: &Direction) -> Vec<(i32, i32)> {
	match direction {
		Direction::North => vec![(position.0 - 1, position.1 - 1), (position.0 - 1, position.1), (position.0 - 1, position.1 + 1)],
		Direction::South => vec![(position.0 + 1, position.1 - 1), (position.0 + 1, position.1), (position.0 + 1, position.1 + 1)],
		Direction::West => vec![(position.0 - 1, position.1 - 1), (position.0, position.1 - 1), (position.0 + 1, position.1 - 1)],
		Direction::East => vec![(position.0 - 1, position.1 + 1), (position.0, position.1 + 1), (position.0 + 1, position.1 + 1)],
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut elves = Vec::new();

	for (r, row) in inputs.iter().enumerate() {
		for (c, ch) in row.chars().enumerate() {
			if ch == '#' {
				elves.push((r as i32, c as i32));
			}
		}
	}

	let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
	let mut directions_start = 0;
	let mut round = 1;

	loop {
		let positions: HashSet<(i32, i32)> = HashSet::from_iter(elves.iter().cloned());
		let mut proposed_positions: HashMap<(i32, i32), usize> = HashMap::new();

		let mut actions = vec![ElfAction::Noop; elves.len()];

		for (i, elf) in elves.iter().enumerate() {
			// If there are no adjacent elves, skip turn
			if get_adjacent_positions(elf).iter().all(|p| !positions.contains(p)) {
				continue;
			}

			for dir_offset in 0..4 {
				let dir = directions[(directions_start + dir_offset) % 4];

				if get_points_in_direction(elf, &dir).iter().all(|p| !positions.contains(p)) {
					let new_pos = apply_direction(elf, &dir);
					proposed_positions.entry(new_pos).and_modify(|e| *e += 1).or_insert(1);
					actions[i] = ElfAction::MoveTo(new_pos);
					break;
				}
			}
		}

		for (i, action) in actions.iter().enumerate() {
			if let ElfAction::MoveTo(new_pos) = action {
				if proposed_positions.get(&new_pos).unwrap() == &1 {
					elves[i] = *new_pos;
				}
			}
		}
		
		if round == 11 {
			let min_r = elves.iter().map(|e| e.0).min().unwrap();
			let max_r = elves.iter().map(|e| e.0).max().unwrap();
			let min_c = elves.iter().map(|e| e.1).min().unwrap();
			let max_c = elves.iter().map(|e| e.1).max().unwrap();

			let positions: HashSet<(i32, i32)> = HashSet::from_iter(elves.iter().cloned());

			let mut part1 = 0;
			for r in min_r..=max_r {
				for c in min_c..=max_c {
					if !positions.contains(&(r, c)) {
						part1 += 1;
					}
				}
			}
			println!("Part 1: {}", part1);
		}

		if actions.iter().all(|a| a == &ElfAction::Noop) {
			println!("Part 2: {}", round);
			break;
		}

		directions_start = (directions_start + 1) % 4;
		round += 1;
	}
}
