use std::collections::HashMap;

pub fn solve(inputs: Vec<String>) {
	let mut cwd: Vec<String> = Vec::new();
	let mut dir_sizes: HashMap<String, usize> = HashMap::new();

	for input in inputs {
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
			}
		} else if input.starts_with("dir ") {
			// ignore directories, not needed
		} else {
			// file, accumulate size into all parent directories
			let (file_size_str, _file_name) = input.split_once(' ').unwrap();
			let file_size = file_size_str.parse::<usize>().unwrap();

			for i in 0..=cwd.len() {
				let cwd_str = cwd[0..i].join("/");
				(*dir_sizes.entry(cwd_str).or_insert(0)) += file_size;
			}
		}
	}

	let part1 = dir_sizes.values().filter(|&s| s <= &100000).sum::<usize>();
	println!("Part 1: {}", part1);

	let current_free_space = 70000000 - dir_sizes.get("").unwrap();
	const REQUIRED_FREE_SPACE: usize = 30000000;
	let space_needed = REQUIRED_FREE_SPACE - current_free_space;

	let part2 = dir_sizes.values().filter(|&s| s >= &space_needed).min().unwrap();
	println!("Part 2: {}", part2);
}
