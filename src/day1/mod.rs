pub fn solve(inputs: Vec<String>) {
	let mut current_calories = 0;
	let mut elf_calories = Vec::new();

	for input in inputs {
		if input.is_empty() {
			elf_calories.push(current_calories);
			current_calories = 0;
		} else {
			current_calories += input.parse::<i32>().unwrap();
		}
	}
	elf_calories.push(current_calories); // Ensure we get the last elf's calories

	elf_calories.sort();
	elf_calories.reverse();

	println!("Part 1: {}", elf_calories[0]);
	println!("Part 2: {}", elf_calories[0..3].iter().sum::<i32>());
}