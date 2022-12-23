use itertools::Itertools;

fn facing_to_char(facing: &(i32, i32)) -> char {
	match facing {
		(0, 1) => '>',
		(0, -1) => '<',
		(1, 0) => 'v',
		(-1, 0) => '^',
		_ => panic!("Unknown facing {:?}", facing),
	}
}

fn facing_value(facing: &(i32, i32)) -> i32 {
	match facing {
		(0, 1) => 0, // right
		(0, -1) => 2, // left
		(1, 0) => 1, // down
		(-1, 0) => 3, // up
		_ => panic!("Unknown facing {:?}", facing),
	}
}

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

fn part1(grid: &Vec<Vec<char>>, directions_amounts: &Vec<usize>, directions_turns: &Vec<&str>) {
	let mut turn_iter = directions_turns.iter();

	let mut pos: (i32, i32) = (0, grid[0].iter().position(|c| *c != ' ').unwrap() as i32);
	let mut facing: (i32, i32) = (0, 1);

	for amount in directions_amounts {
		// println!("{} {}, facing: {}, moving {}", pos.0, pos.1, facing_to_char(&facing), amount);

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

	let facing_val = facing_value(&facing);
	let part1 = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + facing_val;

	println!("Row: {}, Col: {}, Facing: {}", pos.0 + 1, pos.1 + 1, facing_to_char(&facing));
	println!("Part 1: {}", part1);
}

#[derive(Debug, Clone, Copy,  Hash, Eq, PartialEq)]
enum FaceSide {
	Right,
	Bottom,
	Left,
	Top,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Connections {
	right: (usize, FaceSide),
	bottom: (usize, FaceSide),
	left: (usize, FaceSide),
	top: (usize, FaceSide),
}

impl Connections {
	fn new() -> Self {
		Self {
			right: (0, FaceSide::Right),
			bottom: (0, FaceSide::Right),
			left: (0, FaceSide::Right),
			top: (0, FaceSide::Right),
		}
	}
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Face {
	identifier: usize,
	position: (i32, i32),
	connections: Connections,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
	face: usize,
	pos: (i32, i32),
	facing: (i32, i32),
}

fn advance_pos_cube(position: &Position, face_size: i32, faces: &[Face]) -> Position {
	let pos = position.pos;
	let mut new_pos = *position;
	let face = &faces[position.face];
	new_pos.pos = (new_pos.pos.0 + position.facing.0, new_pos.pos.1 + position.facing.1);
	let face_edge = face_size - 1;

	if new_pos.pos.0 < 0 {
		new_pos.face = face.connections.top.0;

		(new_pos.pos, new_pos.facing) = match face.connections.top.1 {
			FaceSide::Bottom => ((face_edge, pos.1), (-1, 0)),
			FaceSide::Top => ((0, face_edge - pos.1), (1, 0)),
			FaceSide::Left => ((pos.1, 0), (0, 1)),
			FaceSide::Right => ((face_edge - pos.1, face_edge), (0, -1)),
		};
	} else if new_pos.pos.0 >= face_size {
		new_pos.face = face.connections.bottom.0;

		(new_pos.pos, new_pos.facing) = match face.connections.bottom.1 {
			FaceSide::Top => ((0, pos.1), (1, 0)),
			FaceSide::Bottom => ((face_edge, face_edge - pos.1), (-1, 0)),
			FaceSide::Left => ((face_edge - pos.1, 0), (0, 1)),
			FaceSide::Right => ((pos.1, face_edge), (0, -1)),
		};
	}

	if new_pos.pos.1 < 0 {
		new_pos.face = face.connections.left.0;

		(new_pos.pos, new_pos.facing) = match face.connections.left.1 {
			FaceSide::Right => ((pos.0, face_edge), (0, -1)),
			FaceSide::Left => ((face_edge - pos.0, 0), (0, 1)),
			FaceSide::Bottom => ((face_edge, face_edge - pos.0), (-1, 0)),
			FaceSide::Top => ((0, pos.0), (1, 0)),
		};
	} else if new_pos.pos.1 >= face_size {
		new_pos.face = face.connections.right.0;

		(new_pos.pos, new_pos.facing) = match face.connections.right.1 {
			FaceSide::Left => ((pos.0, 0), (0, 1)),
			FaceSide::Right => ((face_edge - pos.0, face_edge), (0, -1)),
			FaceSide::Bottom => ((face_edge, pos.0), (-1, 0)),
			FaceSide::Top => ((0, face_edge - pos.0), (1, 0)),
		};
	}

	new_pos
}

fn get_row_col(pos: (i32, i32), face_size: &usize, face: &Face) -> (usize, usize) {
	(face.position.0 as usize * *face_size + pos.0 as usize, face.position.1 as usize * *face_size + pos.1 as usize)
}
fn get_grid_value(grid: &Vec<Vec<char>>, pos: (i32, i32), face_size: &usize, face: &Face) -> char {
	let (row, col) = get_row_col(pos, face_size, face);
	grid[row][col]
}

fn part2(grid: &Vec<Vec<char>>, directions_amounts: &Vec<usize>, directions_turns: &Vec<&str>) {
	let mut turn_iter = directions_turns.iter();

	let face_size = if grid.len() == 12 { 4 } else { 50 };

	let mut faces = Vec::new();
	let mut face_identifier = 0;
	for row in 0..grid.len()/face_size {
		for col in 0..grid[0].len()/face_size {
			if grid[row * face_size][col * face_size] != ' ' {
				faces.push(Face {
					identifier: face_identifier,
					position: (row as i32, col as i32),
					connections: Connections::new(),
				});
				face_identifier += 1;
			}
		}
	}

	if face_size == 4 {
		// Sample input
		faces[0].connections.right = (5, FaceSide::Right);
		faces[0].connections.bottom = (3, FaceSide::Top);
		faces[0].connections.left = (2, FaceSide::Top);
		faces[0].connections.top = (1, FaceSide::Top);

		faces[1].connections.right = (2, FaceSide::Left);
		faces[1].connections.bottom = (4, FaceSide::Bottom);
		faces[1].connections.left = (5, FaceSide::Top);
		faces[1].connections.top = (0, FaceSide::Top);

		faces[2].connections.right = (3, FaceSide::Left);
		faces[2].connections.bottom = (4, FaceSide::Left);
		faces[2].connections.left = (1, FaceSide::Right);
		faces[2].connections.top = (0, FaceSide::Left);

		faces[3].connections.right = (5, FaceSide::Top);
		faces[3].connections.bottom = (4, FaceSide::Top);
		faces[3].connections.left = (2, FaceSide::Right);
		faces[3].connections.top = (0, FaceSide::Bottom);

		faces[4].connections.right = (5, FaceSide::Left);
		faces[4].connections.bottom = (1, FaceSide::Bottom);
		faces[4].connections.left = (2, FaceSide::Bottom);
		faces[4].connections.top = (3, FaceSide::Bottom);

		faces[5].connections.right = (0, FaceSide::Right);
		faces[5].connections.bottom = (1, FaceSide::Left);
		faces[5].connections.left = (4, FaceSide::Right);
		faces[5].connections.top = (3, FaceSide::Right);
	} else {
		// Real input
		faces[0].connections.right = (1, FaceSide::Left);
		faces[0].connections.bottom = (2, FaceSide::Top);
		faces[0].connections.left = (3, FaceSide::Left);
		faces[0].connections.top = (5, FaceSide::Left);

		faces[1].connections.right = (4, FaceSide::Right);
		faces[1].connections.bottom = (2, FaceSide::Right);
		faces[1].connections.left = (0, FaceSide::Right);
		faces[1].connections.top = (5, FaceSide::Bottom);

		faces[2].connections.right = (1, FaceSide::Bottom);
		faces[2].connections.bottom = (4, FaceSide::Top);
		faces[2].connections.left = (3, FaceSide::Top);
		faces[2].connections.top = (0, FaceSide::Bottom);

		faces[3].connections.right = (4, FaceSide::Left);
		faces[3].connections.bottom = (5, FaceSide::Top);
		faces[3].connections.left = (0, FaceSide::Left);
		faces[3].connections.top = (2, FaceSide::Left);

		faces[4].connections.right = (1, FaceSide::Right);
		faces[4].connections.bottom = (5, FaceSide::Right);
		faces[4].connections.left = (3, FaceSide::Right);
		faces[4].connections.top = (2, FaceSide::Bottom);

		faces[5].connections.right = (4, FaceSide::Bottom);
		faces[5].connections.bottom = (1, FaceSide::Top);
		faces[5].connections.left = (0, FaceSide::Top);
		faces[5].connections.top = (3, FaceSide::Bottom);
	}

	let mut pos = Position {
		face: 0,
		pos: (0, 0),
		facing: (0, 1),
	};

	for amount in directions_amounts {
		// println!("Face: {}, Pos: {} {}, Facing: {}, Moving {}", pos.face, pos.pos.0, pos.pos.1, facing_to_char(&pos.facing), amount);

		for _ in 0..*amount {
			let new_pos = advance_pos_cube(&pos, face_size as i32, &faces);

			if get_grid_value(&grid, new_pos.pos, &face_size, &faces[new_pos.face]) == '#' {
				break;
			}

			pos = new_pos;
		}

		if let Some(turn) = turn_iter.next() {
			if turn == &"L" {
				pos.facing = (-pos.facing.1, pos.facing.0);
			} else {
				pos.facing = (pos.facing.1, -pos.facing.0);
			}
		}
	}

	let facing_val = facing_value(&pos.facing);

	let (row, col) = get_row_col(pos.pos, &face_size, &faces[pos.face]);
	let part2 = (row + 1) * 1000 + (col + 1) * 4 + facing_val as usize;

	println!("Row: {}, Col: {}, Facing: {}", row, col, facing_to_char(&pos.facing));
	println!("Part 2: {}", part2);
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

	part1(&grid, &directions_amounts, &directions_turns);
	println!("");
	part2(&grid, &directions_amounts, &directions_turns);
}
