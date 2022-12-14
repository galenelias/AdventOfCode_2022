use itertools::Itertools;
use std::collections::{VecDeque, HashSet};

type Grid = Vec<Vec<usize>>;

fn get_adjacent_points((r, c): (usize, usize), grid: &Grid) -> Vec<(usize,usize)> {
	let mut result = Vec::new();
	if r > 0 { result.push((r - 1, c)); }
	if r < grid.len() - 1 { result.push((r + 1, c)); }
	if c > 0 { result.push((r, c - 1)); }
	if c < grid[0].len() - 1 { result.push((r, c + 1)); }
	return result;
}

fn shortest_path(heights: &Grid, start: (usize, usize), end: (usize, usize)) -> usize {
	let mut visited = HashSet::new();
	let mut q: VecDeque<((usize, usize), usize)> = VecDeque::new();
	q.push_back((start, 0));

	while !q.is_empty() {
		let ((r, c), steps) = q.pop_front().unwrap();

		if (r, c) == end {
			return steps;
		}

		if !visited.insert((r, c)) {
			continue;
		}

		for adjacent in get_adjacent_points((r, c), &heights) {
			if heights[adjacent.0][adjacent.1] <= heights[r][c] + 1 {
				q.push_back((adjacent, steps + 1));
			}
		}
	}

	return usize::MAX; // No path found
}

pub fn solve(inputs: Vec<String>) {
	let mut heights = Vec::new();
	let mut start: (usize, usize) = (0, 0);
	let mut end: (usize, usize) = (0, 0);

	for (r, input) in inputs.iter().enumerate() {
		let mut row = Vec::new();
		for (c, ch) in input.chars().enumerate() {
			match ch {
				'a'..='z' => {
					row.push(ch as usize - 'a' as usize);
				},
				'S' => {
					start = (r, c);
					row.push(0);
				},
				'E' => {
					end = (r, c);
					row.push(25);
				},
				_ => unreachable!("Unexpected input: {}", ch),
			}
		}
		heights.push(row);
	}

	println!("Part 1: {}", shortest_path(&heights, start, end));

	let possible_starts = heights.iter().enumerate()
		.flat_map(|(r, row)| row.iter().enumerate()
			.filter(|(_, &height)| height == 0)
			.map(move |(c, _)| (r, c)))
		.collect_vec();

	let part2 = possible_starts.iter()
		.map(|&start| shortest_path(&heights, start, end))
		.min()
		.unwrap();

	println!("Part 2: {}", part2);
}