fn is_p1_sampled(cycle: i32) -> bool {
	return cycle >= 20 && ((cycle - 20) % 40) == 0
}

fn eval_pixel(cycle: i32, reg_x: i32) -> bool {
	(reg_x - ((cycle - 1) % 40)).abs() <= 1
}

pub fn solve(inputs: Vec<String>) {
	let mut reg_x: i32 = 1;
	let mut cycle = 1;
	let mut part1 = 0;

	let mut lcd = vec![false; 240];

	for input in inputs {
		lcd[(cycle - 1) as usize] = eval_pixel(cycle, reg_x);
		if is_p1_sampled(cycle) {
			part1 += cycle * reg_x;
		}

		if input.starts_with("noop") {
			cycle += 1;
			continue;
		} else if input.starts_with("addx") {
			let (_instr, num) = input.split_once(' ').unwrap();
			let num = num.parse::<i32>().unwrap();

			lcd[cycle as usize] = eval_pixel(cycle + 1, reg_x);
			if is_p1_sampled(cycle + 1) {
				part1 += (cycle + 1) * reg_x;
			}
	
			cycle += 2;
			reg_x += num;
		}
	}

	println!("Part 1: {}", part1);

	println!("Part 2:");
	for r in 0..6 {
		println!("{}", lcd[r*40..(r+1)*40].iter().map(|&b| if b { '#' } else { ' ' }).collect::<String>());
	}
}