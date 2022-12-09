use std::collections::HashSet;

use itertools::Itertools;

fn sub_solve(inputs: &Vec<(char, usize)>, rope_len: usize) -> usize {
	let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_len];
	let mut tail_positions = HashSet::new();

	tail_positions.insert(rope.last().unwrap().clone());

	for &(dir, num) in inputs {
		for _ in 0..num {
			match dir {
				'L' => rope[0].1 -= 1, 
				'R' => rope[0].1 += 1,
				'U' => rope[0].0 -= 1,
				'D' => rope[0].0 += 1,
				_ => panic!("Invalid input"),
			}

			for i in 1..rope.len() {
				if (rope[i-1].0 - rope[i].0).abs() == 2 || (rope[i-1].1 - rope[i].1).abs() == 2 {
					let dr = (rope[i-1].0 - rope[i].0).signum();
					let dc = (rope[i-1].1 - rope[i].1).signum();

					rope[i].0 += dr;
					rope[i].1 += dc;
				}
			}
			tail_positions.insert(rope.last().unwrap().clone());
		}
	}

	return tail_positions.len();
}

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs
		.iter()
		.map(|s| s.split_once(' ').unwrap())
		.map(|(ch, num)| (ch.chars().next().unwrap(), num.parse::<usize>().unwrap()))
		.collect_vec();

	println!("Part 1: {}", sub_solve(&inputs, 2));
	println!("Part 2: {}", sub_solve(&inputs, 10));
}
