use itertools::Itertools;
use std::collections::HashSet;

fn item_value(item: &char) -> u32 {
	match item {
		'a'..='z' => *item as u32 - 'a' as u32 + 1,
		'A'..='Z' => *item as u32 - 'A' as u32 + 27,
		_ => unreachable!("Unexpected item {}", item),
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	for input in &inputs{
		let chars = input.chars().collect_vec();

		let set_a: HashSet<_> = chars[0..chars.len()/2].iter().collect();
		let set_b: HashSet<_> = chars[chars.len()/2..].iter().collect();
		part1 += item_value(set_a.intersection(&set_b).next().unwrap());
	}

	let mut part2 = 0;
	for group in 0..inputs.len() / 3 {
		let set_a: HashSet<_> = inputs[group*3 + 0].chars().collect();
		let set_b: HashSet<_> = inputs[group*3 + 1].chars().collect();
		let set_c: HashSet<_> = inputs[group*3 + 2].chars().collect();

		let set_ab: HashSet<char> = set_a.intersection(&set_b).cloned().collect();
		let set_abc: HashSet<_> = set_ab.intersection(&set_c).cloned().collect();

		part2 += item_value(set_abc.iter().next().unwrap());
	}

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);
}
