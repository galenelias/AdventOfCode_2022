use itertools::Itertools;

fn compute_scenic_score(grid: &Vec<Vec<i32>>, r: usize, c: usize) -> usize {
	let rows = grid.len();
	let cols = grid[0].len();

	let mut left_score = c;
	for i in 1..=c {
		if grid[r][c - i] >= grid[r][c] {
			left_score = i;
			break;
		}
	}

	let mut right_score = cols - c - 1;
	for i in 1..=cols - c - 1 {
		if grid[r][c + i] >= grid[r][c] {
			right_score = i;
			break;
		}
	}

	let mut top_score = r;
	for i in 1..=r {
		if grid[r - i][c] >= grid[r][c] {
			top_score = i;
			break;
		}
	}

	let mut bottom_score = rows - r - 1;
	for i in 1..=rows - r - 1 {
		if grid[r + i][c] >= grid[r][c] {
			bottom_score = i;
			break;
		}
	}

	let score = left_score * right_score * top_score * bottom_score;

	return score;
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
		let mut max_tree_left = -1;
		let mut max_tree_right = -1;
		for c in 0..cols {
			if grid[r][c] > max_tree_left {
				visible[r][c] = true;
				max_tree_left = grid[r][c];
			}
			if grid[r][cols - c - 1] > max_tree_right {
				visible[r][cols - c - 1] = true;
				max_tree_right = grid[r][cols - c - 1];
			}
		}
	}

	for c in 0..cols {
		let mut max_tree_top = -1;
		let mut max_tree_bottom = -1;
		for r in 0..rows {
			if grid[r][c] > max_tree_top {
				visible[r][c] = true;
				max_tree_top = grid[r][c];
			}
			if grid[rows - r - 1][c] > max_tree_bottom {
				visible[rows - r - 1][c] = true;
				max_tree_bottom = grid[rows - r - 1][c];
			}
		}
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
