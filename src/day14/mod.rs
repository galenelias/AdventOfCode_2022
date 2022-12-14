use itertools::Itertools;

fn drip(y_start: usize, x_start: usize, grid: &mut Vec<Vec<char>>) -> bool {
	if grid[y_start][x_start] == 'o' {
		return false;
	}

	let mut x = x_start;
	for y in y_start..grid.len()-1 {
		if grid[y+1][x] == '.' {
			continue;
		}

		// Try to flow down and to the left
		if grid[y+1][x - 1] == '.' {
			x -= 1;
		} else if grid[y+1][x+1] == '.' { // Try to flow down and to the right
			x += 1;
		} else {
			grid[y][x] = 'o';
			return true;
		}
	}
	return false;
}

fn print_grid(grid: &Vec<Vec<char>>, is_part2: bool) {
	let grid_iter = if is_part2 {
		grid[..grid.len()-2].iter()
	} else {
		grid[..grid.len()-2].iter()
		// grid.iter()
	};
	let points = grid_iter.enumerate().flat_map(|(y, row)| row.iter().enumerate().filter(|(_, c)| **c != '.').map(move |(x, _)| (y, x))).collect_vec();
	let min_y = points.iter().map(|(y, _)| *y).min().unwrap();
	let max_y = points.iter().map(|(y, _)| *y).max().unwrap();
	let min_x = points.iter().filter(|(y, _)| !is_part2 || *y != max_y).map(|(_, x)| *x).min().unwrap();
	let max_x = points.iter().filter(|(y, _)| !is_part2 || *y != max_y).map(|(_, x)| *x).max().unwrap();

	for r in min_y..=max_y {
		println!("{}", grid[r][min_x..=max_x].iter().collect::<String>());
	}
	println!("");
}

fn sub_solve(mut grid: Vec<Vec<char>>, debug_draw: bool, is_part2: bool) -> usize {
	if is_part2 {
		let max_y = grid.iter().enumerate().rev().find(|(_, row)| row.iter().any(|c| *c != '.')).unwrap().0;
		for x in 0..1000 {
			grid[max_y + 2][x] = '#';
		}
	}

	loop {
		if !drip(0, 500, &mut grid) {
			break;
		}

		if debug_draw {
			print_grid(&grid, is_part2);
		}
	}

	if debug_draw {
		print_grid(&grid, is_part2);
	}

	let sand_count = grid.iter().map(|row| row.iter().filter(|&c| c == &'o').count()).sum::<usize>();
	return sand_count
}

pub fn solve(inputs : Vec<String>) {
	let debug_draw = true;
	let mut grid = vec![vec!['.'; 1000]; 1000];

	for input in inputs {
		let points = input.split(" -> ").map(|s| s.split(",").map(|s| s.parse::<usize>().unwrap()).collect_vec()).collect_vec();

		for i in 0..points.len()-1 {
			let mut pt1 = (points[i][0], points[i][1]);
			let mut pt2 = (points[i+1][0], points[i+1][1]);
			if pt2.0 < pt1.0 || pt2.1 < pt1.1 {
				(pt2, pt1) = (pt1, pt2);
			}

			if pt1.0 == pt2.0 { // vertical
				for y in pt1.1..=pt2.1 {
					grid[y][pt1.0] = '#';
				}
			} else { // horizontal
				for x in pt1.0..=pt2.0 {
					grid[pt1.1][x] = '#';
				}
			}
		}
	}

	println!("Part 1: {}", sub_solve(grid.clone(), debug_draw, false));
	println!("Part 2: {}", sub_solve(grid, debug_draw, true));
}