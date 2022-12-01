use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let elf_calories = inputs
		.split(|i| i.is_empty())
		.map(|inventory| inventory.iter().map(|i| i.parse::<u32>().unwrap()).sum::<u32>())
		.sorted()
		.rev()
		.collect_vec();

	println!("Part 1: {}", elf_calories[0]);
	println!("Part 2: {}", elf_calories[0..3].iter().sum::<u32>());
}
