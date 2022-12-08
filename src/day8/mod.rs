use itertools::Itertools;

fn calculate_visible(grid: &Vec<Vec<i32>>, visible: &mut Vec<Vec<bool>>, r: usize, c: usize, dr: i32, dc: i32) {
	let rows = grid.len() as i32;
	let cols = grid[0].len() as i32;

	let mut max_tree = -1;
	let mut r = r as i32;
	let mut c = c as i32;

	while r >= 0 && c >= 0 && r < rows && c < cols {
		if grid[r as usize][c as usize] > max_tree {
			visible[r as usize][c as usize] = true;
			max_tree = grid[r as usize][c as usize];
		}

		r += dr;
		c += dc;
	}
}

fn viewing_distance(grid: &Vec<Vec<i32>>, r: usize, c: usize, dr: i32, dc: i32) -> usize{
	let rows = grid.len() as i32;
	let cols = grid[0].len() as i32;

	let start_height = grid[r][c];
	let mut r = r as i32 + dr;
	let mut c = c as i32 + dc;
	let mut dist = 0;

	while r >= 0 && c >= 0 && r < rows && c < cols {
		dist += 1;
		if grid[r as usize][c as usize] >= start_height {
			break;
		}

		r += dr;
		c += dc;
	}
	return dist;
}

fn compute_scenic_score(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> usize {
	let left_score = viewing_distance(grid, r, c, 0, -1);
	let right_score = viewing_distance(grid, r, c, 0, 1);
	let top_score = viewing_distance(grid, r, c, -1, 0);
	let bottom_score = viewing_distance(grid, r, c, 1, 0);
	return left_score * right_score * top_score * bottom_score;
}

pub fn solve(inputs: Vec<String>) {
	let grid = inputs
		.iter()
		.map(|s| {
			s.chars()
				.map(|c| c.to_digit(10).unwrap() as i32)
				.collect_vec()
		})
		.collect_vec();
	let rows = grid.len();
	let cols = grid[0].len();

	let mut visible = vec![vec![false; grid[0].len()]; rows];

	// Walk from left, right, top, and bottom keeping track of highest tree seen from any direction
	for r in 0..rows {
		calculate_visible(&grid, &mut visible, r, 0, 0, 1);
		calculate_visible(&grid, &mut visible, r, cols - 1, 0, -1);
	}
	for c in 0..cols {
		calculate_visible(&grid, &mut visible, 0, c, 1, 0);
		calculate_visible(&grid, &mut visible, rows - 1, c, -1, 0);
	}

	let part1 = visible.iter().map(|v| v.iter().filter(|b| **b).count()).sum::<usize>();
	println!("Part 1: {}", part1);

	let part2 = (0..rows)
		.map(|r| {
			(0..cols)
				.map(|c| compute_scenic_score(&grid, r, c)).max().unwrap()
		})
		.max().unwrap();
	println!("Part 2: {}", part2);
}
