use std::collections::{VecDeque, HashSet};
use itertools::Itertools;

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

struct Blizzard {
	direction: Direction,
	pos: (usize, usize),
}

#[derive(Debug, Eq, PartialEq)]
enum Phase {
	InitialEnd,
	BackToStart,
	FinalEnd,
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs.iter().map(|l| l.chars().collect_vec()).collect_vec();

	let start = (0, grid[0].iter().position(|&c| c == '.').unwrap());
	let end = (
		grid.len() - 1,
		grid.last().unwrap().iter().position(|&c| c == '.').unwrap(),
	);

	let mut blizzards = Vec::new();
	for (r, row) in grid.iter().enumerate() {
		for (c, ch) in row.iter().enumerate() {
			if let Some(dir) = match ch {
				&'<' => Some(Direction::Left),
				&'>' => Some(Direction::Right),
				&'^' => Some(Direction::Up),
				&'v' => Some(Direction::Down),
				_ => None,
			} {
				blizzards.push(Blizzard {
					direction: dir,
					pos: (r, c),
				});
			}
		}
	}

	let mut seen = HashSet::new();
	let mut queue = VecDeque::new();
	let mut blizzard_positions = HashSet::new();
	let mut current_time = 0;
	let mut phase = Phase::InitialEnd;

	queue.push_back((start, 0));

	while !queue.is_empty() {
		let (pos, time) = queue.pop_front().unwrap();

		if grid[pos.0][pos.1] == '#' {
			continue;
		}

		if !seen.insert((pos, time)) {
			continue;
		}

		if phase == Phase::FinalEnd && pos == end {
			println!("Found path in {} steps", time);
			break;
		}

		if phase == Phase::BackToStart && pos == start {
			println!("Got back to start in {} steps", time);
			queue.clear();
			phase = Phase::FinalEnd;
		}

		if phase == Phase::InitialEnd && pos == end {
			println!("Got to end initially in {} steps", time);
			queue.clear();
			phase = Phase::BackToStart;
		}

		if time != current_time {
			// Advance blizzards
			blizzard_positions.clear();
			for mut blizzard in &mut blizzards {
				let (r, c) = blizzard.pos;
				match blizzard.direction {
					Direction::Up => {
						if r == 1 {
							blizzard.pos.0 = grid.len() - 2;
						} else {
							blizzard.pos.0 -= 1;
						}
					},
					Direction::Down => {
						if r == grid.len() - 2 {
							blizzard.pos.0 = 1;
						} else {
							blizzard.pos.0 += 1;
						}
					},
					Direction::Left => {
						if c == 1 {
							blizzard.pos.1 = grid[0].len() - 2;
						} else {
							blizzard.pos.1 -= 1;
						}
					},
					Direction::Right => {
						if c == grid[0].len() - 2 {
							blizzard.pos.1 = 1;
						} else {
							blizzard.pos.1 += 1;
						}
					}
				}
				blizzard_positions.insert(blizzard.pos);
			}
			current_time = time;
		}

		if blizzard_positions.contains(&pos) {
			continue;
		}

		if pos.0 > 0 {
			queue.push_back(((pos.0 - 1, pos.1), time + 1));
		}
		if pos.0 < grid.len() - 1 {
			queue.push_back(((pos.0 + 1, pos.1), time + 1));
		}
		queue.push_back(((pos.0, pos.1 - 1), time + 1));
		queue.push_back(((pos.0, pos.1 + 1), time + 1));
		queue.push_back(((pos.0, pos.1), time + 1)); // no-op
	}
}
