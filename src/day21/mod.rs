use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct MathOperation {
	op: char,
	var1: String,
	var2: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum MonkeyJob {
	YellNumber(i64),
	YellOperation(MathOperation),
}

fn resolve_monkey(monkey: &str, monkeys: &HashMap<String, MonkeyJob>, human_value: Option<i64>) -> Option<i64> {
	let monkey_job = monkeys.get(monkey).unwrap();

	if monkey == "humn" {
		return human_value;
	}

	match monkey_job {
		MonkeyJob::YellNumber(number) => Some(*number),
		MonkeyJob::YellOperation(operation) => {
			let var1 = resolve_monkey(&operation.var1, monkeys, human_value);
			let var2 = resolve_monkey(&operation.var2, monkeys, human_value);

			match (var1, var2) {
				(Some(var1), Some(var2)) => {
					match operation.op {
						'+' => Some(var1 + var2),
						'-' => Some(var1 - var2),
						'*' => Some(var1 * var2),
						'/' => Some(var1 / var2),
						_ => panic!("Unknown operation {}", operation.op),
					}
				},
				_ => None,
			}
		}
	}
}

pub fn solve(inputs: Vec<String>) {
	let monkeys: HashMap<String, MonkeyJob> = inputs.iter().map(|input| {
		let (name, str_job) = input.split_once(": ").unwrap();
		let job = if let Ok(number) = str_job.parse::<i64>() {
			MonkeyJob::YellNumber(number)
		} else {
			let parts = str_job.split(" ").collect_vec();
			MonkeyJob::YellOperation(MathOperation {
				op: parts[1].chars().next().unwrap(),
				var1: parts[0].to_string(),
				var2: parts[2].to_string(),
			})
		};

		(name.to_string(), job)
	}).collect();

	let root_job = monkeys.get("root").unwrap();
	let human_job = monkeys.get("humn").unwrap();
	let human_value = match human_job { MonkeyJob::YellNumber(a) => a, _ => panic!("Root job is not a yell operation")};

	println!("Part 1: {}", resolve_monkey("root", &monkeys, Some(*human_value)).unwrap());

	// Part 2
	let root_job = match root_job { MonkeyJob::YellOperation(a) => a, _ => panic!("Root job is not a yell operation")};

	let left = resolve_monkey(&root_job.var1, &monkeys, None);
	let right = resolve_monkey(&root_job.var2, &monkeys, None);

	let (target, human_branch) = match (left, right) {
		(None, Some(value)) => (value, root_job.var1.clone()),
		(Some(value), None) => (value, root_job.var2.clone()),
		(_, _) => panic!("Exactly one branch should be None"),
	};

	let probe1 = resolve_monkey(&human_branch, &monkeys, Some(0)).unwrap();
	let probe2 = resolve_monkey(&human_branch, &monkeys, Some(1000)).unwrap();
	let gradient = (probe1 - probe2).signum();

	let mut low: i64 = 0;
	let mut high: i64 = i64::MAX / 1000; // STart a bit lower than i64 max, otherwise early multiplication will overflow
	while low <= high {
		let mid = (high - low) / 2 + low;
		let val = resolve_monkey(&human_branch, &monkeys, Some(mid)).unwrap();

		if (val - target).signum() == gradient {
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}

	println!("Part 2: {}", low);
}