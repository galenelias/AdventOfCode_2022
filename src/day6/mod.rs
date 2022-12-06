use itertools::Itertools;

fn find_unique_sequence(input: &[char], seq_length: usize) -> Option<usize> {
	for i in 0..=input.len()-seq_length {
		if input[i..i+seq_length].iter().unique().count() == seq_length {
			return Some(i+seq_length);
		}
	}
	return None;
}

pub fn solve(inputs: Vec<String>) {
	let input = inputs[0].chars().collect_vec();

	println!("Part 1: {}", find_unique_sequence(&input, 4).unwrap());
	println!("Part 2: {}", find_unique_sequence(&input, 14).unwrap());
}
