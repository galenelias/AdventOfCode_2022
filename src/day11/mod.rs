use itertools::Itertools;
use std::collections::VecDeque;
use num::integer::lcm;

#[derive(Debug, Clone)]
enum OperationTarget {
	Number(i64),
	Old,
}

#[derive(Debug, Clone)]
enum Operation {
	Multiply(OperationTarget),
	Add(OperationTarget),
}

#[derive(Debug, Clone)]
struct Monkey {
	items: VecDeque<i64>,
	operation: Operation,
	test_op_divisible: i64,
	test_target_true: usize,
	test_target_false: usize,
	inspection_count: usize,
}

impl Monkey {
	fn new() -> Monkey {
		Monkey {
			items: VecDeque::new(),
			operation: Operation::Multiply(OperationTarget::Old),
			test_op_divisible: 0,
			test_target_true: 0,
			test_target_false: 0,
			inspection_count: 0,
		}
	}
}

fn sub_solve(mut monkeys: Vec<Monkey>, rounds: usize, is_part1: bool) -> usize {
	let mut monkey_lcm = 1;
	for monkey in &monkeys {
		monkey_lcm = lcm(monkey_lcm, monkey.test_op_divisible);
	}

	for _round in 0..rounds {
		for i in 0..monkeys.len() {
			while !monkeys[i].items.is_empty() {
				monkeys[i].inspection_count += 1;
				let old = monkeys[i].items.pop_front().unwrap();

				let mut new = match &monkeys[i].operation {
					Operation::Multiply(target) => match target {
						OperationTarget::Old => old * old,
						OperationTarget::Number(n) => old * n,
					},
					Operation::Add(target) => match target {
						OperationTarget::Old => old + old,
						OperationTarget::Number(n) => old + n,
					},
				};

				if is_part1 {
					new /= 3;
				}

				new %= monkey_lcm;

				let target_monkey = if new % monkeys[i].test_op_divisible == 0 {
					monkeys[i].test_target_true as usize
				} else {
					monkeys[i].test_target_false as usize
				};

				monkeys[target_monkey].items.push_back(new);
			}
		}
	}

	let inspection_counts = monkeys.iter().map(|m| m.inspection_count).sorted().rev().collect_vec();
	return inspection_counts[0] * inspection_counts[1];
}

pub fn solve(mut inputs: Vec<String>) {
	let mut monkeys = Vec::new();

	const PREFIX_STARTING_ITEMS: &str = "  Starting items: ";
	const PREFIX_OPERATION: &str = "  Operation: new = old ";
	const PREFIX_TEST: &str = "  Test: divisible by ";
	const PREFIX_TEST_TRUE: &str = "    If true: throw to monkey ";
	const PREFIX_TEST_FALSE: &str = "    If false: throw to monkey ";

	// Ensure blank line at end of input
	if !inputs.last().unwrap().is_empty() {
		inputs.push(String::new());
	}

	// Seed items
	let mut current_monkey = Monkey::new();
	for input in &inputs {
		if input.starts_with(PREFIX_STARTING_ITEMS) {
			current_monkey.items = input[PREFIX_STARTING_ITEMS.len()..]
				.split(", ")
				.map(|s| s.parse::<i64>().unwrap())
				.collect();
		} else if input.starts_with(PREFIX_OPERATION) {
			let (op, amt) = input[PREFIX_OPERATION.len()..].split_once(" ").unwrap();
			let target = match amt {
				"old" => OperationTarget::Old,
				_ => OperationTarget::Number(amt.parse::<i64>().unwrap()),
			};
			current_monkey.operation = match op {
				"*" => Operation::Multiply(target),
				"+" => Operation::Add(target),
				_ => panic!("Unknown operation: {}", op),
			};
		} else if input.starts_with(PREFIX_TEST) {
			current_monkey.test_op_divisible = input[PREFIX_TEST.len()..].parse::<i64>().unwrap();
		} else if input.starts_with(PREFIX_TEST_TRUE) {
			current_monkey.test_target_true = input[PREFIX_TEST_TRUE.len()..].parse::<usize>().unwrap();
		} else if input.starts_with(PREFIX_TEST_FALSE) {
			current_monkey.test_target_false = input[PREFIX_TEST_FALSE.len()..].parse::<usize>().unwrap();
		} else if input.is_empty() {
			monkeys.push(current_monkey);
			current_monkey = Monkey::new();
		}
	}

	println!("Part 1: {}", sub_solve(monkeys.clone(), 20, true));
	println!("Part 2: {}", sub_solve(monkeys, 10_000, false));
}
