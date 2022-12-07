use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(inputs: Vec<String>) {
	let mut cwd: Vec<String> = Vec::new();
	let mut i = 0;

	let mut dir_sizes: HashMap<String, usize> = HashMap::new();

	while i < inputs.len() {
		let input = &inputs[i];
		if input.starts_with("$ ") {
			let sub_cmd = input[2..].to_string();
			if sub_cmd.starts_with("cd ") {
				let dir = sub_cmd[3..].to_string();
				if dir == ".." {
					cwd.pop();
				} else if dir == "/" {
					cwd.clear();
				} else {
					cwd.push(dir);
				}
				i += 1;
			} else if sub_cmd.starts_with("ls") {
				i += 1;
				while i < inputs.len() {
					let line = &inputs[i];
					if line.starts_with("$ ") { // Next command
						break;
					} else if line.starts_with("dir ") {
						// unneeded
					} else {
						let parts = line.split_whitespace().collect_vec();
						let file_size = parts[0].parse::<usize>().unwrap();

						// Accumulate file size into all parent directories
						for cwd_i in 0..=cwd.len() {
							let cwd_str = cwd[0..cwd_i].join("/");
							let dir_size = dir_sizes.entry(cwd_str).or_insert(0);
							*dir_size += file_size;
						}
					}
					i += 1;
				}
			}
		}
	}

	let mut part1 = 0;
	for (_dir, size) in dir_sizes.iter() {
		if size <= &100000 {
			part1 += size;
		}
	}
	println!("Part 1: {}", part1);

	let mut part2 = 70000000; // Find min, so start with max
	let free_space = 70000000 - dir_sizes.get("").unwrap();
	const NEEDED_SPACE: usize = 30000000;
	let space_needed = NEEDED_SPACE - free_space;

	for (dir, size) in dir_sizes.iter() {
		if *size >= space_needed && *size < part2 {
			part2 = *size;
		}
	}

	println!("Part 2: {}", part2);
}
