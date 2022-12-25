
fn snafu_to_decimal(snafu: &str) -> i64 {
	let mut result = 0;
	for (i, ch) in snafu.chars().rev().enumerate() {
		result += 5i64.pow(i as u32) * match ch {
			'0' => 0,
			'1' => 1,
			'2' => 2,
			'-' => -1,
			'=' => -2,
			_ => panic!("Invalid character {} in snafu string", ch),
		};
	}
	result
}

fn decimal_to_snafu(mut input: i64) -> String {
	let mut result = String::new();

	let mut max_exp = 0;
	while 2 * 5i64.pow(max_exp) <= input {
		max_exp += 1;
	}

	for exp in (0..=max_exp).rev() {
		let digit = input as f32 / 5i64.pow(exp) as f32;
		if digit > 1.5 {
			result.push('2');
			input -= 2 * 5i64.pow(exp);
		} else if digit > 0.5 {
			result.push('1');
			input -= 5i64.pow(exp);
		} else if digit > -0.5 {
			result.push('0');
		}
		else if digit > -1.5 {
			result.push('-');
			input += 5i64.pow(exp);
		}
		else if digit > -2.5 {
			result.push('=');
			input += 2 * 5i64.pow(exp);
		} else {
			panic!("Invalid digit {} for input {}", digit, input);
		}
	}

	result
}

pub fn solve(inputs: Vec<String>) {

	let mut part1_dec = 0;
	for input in &inputs {
		let dec = snafu_to_decimal(input);
		part1_dec += dec;
	}

	let part1_snafu = decimal_to_snafu(part1_dec);
	println!("Part 1: {} (decimal: {})", part1_snafu, part1_dec);

}