use itertools::Itertools;
// use std::collections::{HashMap, VecDeque, HashSet};

fn advance_pos(pos: (i32, i32), facing: (i32, i32), grid: &Vec<Vec<char>>) -> (i32, i32) {
	let mut new_pos = (pos.0 + facing.0, pos.1 + facing.1);
	if new_pos.0 < 0 {
		new_pos.0 = grid.len() as i32 - 1;
	} else if new_pos.0 >= grid.len() as i32 {
		new_pos.0 = 0;
	}

	if new_pos.1 < 0 {
		new_pos.1 = grid[new_pos.0 as usize].len() as i32 - 1;
	} else if new_pos.1 >= grid[new_pos.0 as usize].len() as i32 {
		new_pos.1 = 0;
	}
	new_pos
}

fn facing_to_char(facing: &(i32, i32)) -> char {
	match facing {
		(0, 1) => '>',
		(0, -1) => '<',
		(1, 0) => 'v',
		(-1, 0) => '^',
		_ => panic!("Unknown facing {:?}", facing),
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs.iter().take_while(|l| !l.is_empty()).map(|l| l.chars().collect_vec()).collect_vec();

	let max_col_length = grid.iter().map(|l| l.len()).max().unwrap();
	for line in &mut grid {
		while line.len() < max_col_length {
			line.push(' ');
		}
	}

	let directions = inputs.last().unwrap();
	let directions_amounts = directions.split(&['L', 'R']).map(|d| d.parse::<usize>().unwrap()).collect_vec();
	let directions_turns = directions.split(char::is_numeric).filter(|d| !d.is_empty()).collect_vec();
	let mut turn_iter = directions_turns.iter();

	let start_pos: (i32, i32) = (0, grid[0].iter().position(|c| *c != ' ').unwrap() as i32);
	let mut pos = start_pos;
	let mut facing: (i32, i32) = (0, 1);

	for amount in &directions_amounts {
		println!("{} {}, facing: {}, moving {}", pos.0, pos.1, facing_to_char(&facing), amount);

		for _ in 0..*amount {
			let mut new_pos = advance_pos(pos, facing, &grid);
			while grid[new_pos.0 as usize][new_pos.1 as usize] == ' ' {
				new_pos = advance_pos(new_pos, facing, &grid);
			}

			if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
				break;
			}

			pos = new_pos;
		}

		if let Some(turn) = turn_iter.next() {
			if turn == &"L" {
				facing = (-facing.1, facing.0);
			} else {
				facing = (facing.1, -facing.0);
			}
		}
	}

	println!("{} {}, facing: {}", pos.0, pos.1, facing_to_char(&facing));

	let facing_val = match facing {
		(0, 1) => 0, // right
		(0, -1) => 2, // left
		(1, 0) => 1, // down
		(-1, 0) => 3, // up
		_ => panic!("Invalid facing: {:?}", facing),
	};

	let part1 = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + facing_val;
	println!("Part 1: {}", part1);
}
