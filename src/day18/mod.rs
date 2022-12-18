use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq)]
enum Type {
	Cube,
	Outside
}

pub fn solve(inputs: Vec<String>) {

	let inputs = inputs.iter().map(|s| s.split(",").map(|v| v.parse::<i32>().unwrap()).collect_vec()).collect_vec();
	let mut points = HashMap::new();

	for pts in &inputs {
		points.insert((pts[0], pts[1], pts[2]), Type::Cube);
	}

	let mut part1 = 0;
	for pts in &inputs {
		if !points.contains_key(&(pts[0] - 1, pts[1], pts[2])) {
			part1 += 1;
		}
		if !points.contains_key(&(pts[0] + 1, pts[1], pts[2])) {
			part1 += 1;
		}
		if !points.contains_key(&(pts[0], pts[1] - 1, pts[2])) {
			part1 += 1;
		}
		if !points.contains_key(&(pts[0], pts[1] + 1, pts[2])) {
			part1 += 1;
		}
		if !points.contains_key(&(pts[0], pts[1], pts[2] - 1)) {
			part1 += 1;
		}
		if !points.contains_key(&(pts[0], pts[1], pts[2] + 1)) {
			part1 += 1;
		}
	}

	let min_x = inputs.iter().map(|v| v[0]).min().unwrap() - 1;
	let max_x = inputs.iter().map(|v| v[0]).max().unwrap() + 1;
	let min_y = inputs.iter().map(|v| v[1]).min().unwrap() - 1;
	let max_y = inputs.iter().map(|v| v[1]).max().unwrap() + 1;
	let min_z = inputs.iter().map(|v| v[2]).min().unwrap() - 1;
	let max_z = inputs.iter().map(|v| v[2]).max().unwrap() + 1;

	// Flood fill from any point outside, filling any touched square with an 'Outside' marker
	let mut queue = VecDeque::new();
	queue.push_back((min_x, min_y, min_z));
	while !queue.is_empty() {
		let pt = queue.pop_front().unwrap();
		if pt.0 < min_x || pt.0 > max_x || pt.1 < min_y || pt.1 > max_y || pt.2 < min_z || pt.2 > max_z {
			continue;
		}

		if points.contains_key(&pt) {
			continue;
		}

		points.insert(pt, Type::Outside);
		queue.push_back((pt.0 - 1, pt.1, pt.2));
		queue.push_back((pt.0 + 1, pt.1, pt.2));
		queue.push_back((pt.0, pt.1 - 1, pt.2));
		queue.push_back((pt.0, pt.1 + 1, pt.2));
		queue.push_back((pt.0, pt.1, pt.2 - 1));
		queue.push_back((pt.0, pt.1, pt.2 + 1));
	}

	// Now total squares which touch the Outside explicitly
	let mut part2 = 0;
	for pts in &inputs {
		if points.get(&(pts[0] - 1, pts[1], pts[2])) == Some(&Type::Outside) {
			part2 += 1;
		}
		if points.get(&(pts[0] + 1, pts[1], pts[2])) == Some(&Type::Outside) {
			part2 += 1;
		}
		if points.get(&(pts[0], pts[1] - 1, pts[2])) == Some(&Type::Outside) {
			part2 += 1;
		}
		if points.get(&(pts[0], pts[1] + 1, pts[2])) == Some(&Type::Outside) {
			part2 += 1;
		}
		if points.get(&(pts[0], pts[1], pts[2] - 1)) == Some(&Type::Outside) {
			part2 += 1;
		}
		if points.get(&(pts[0], pts[1], pts[2] + 1)) == Some(&Type::Outside) {
			part2 += 1;
		}
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}