use std::collections::HashSet;
use regex::Regex;

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

	let mut part1_coverage: HashSet<(i32, i32)> = HashSet::new();
	let mut sensors: Vec<((i32, i32), i32)> = Vec::new();

	for input in inputs {
		let caps = re_input.captures(&input).unwrap();

		let sensor = (caps[1].parse::<i32>().unwrap(), caps[2].parse::<i32>().unwrap());
		let beacon = (caps[3].parse::<i32>().unwrap(), caps[4].parse::<i32>().unwrap());

		let dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

		for x in sensor.0 - dist..=sensor.0+dist {
			let y = 2000000;
			if (x, y) != beacon && (sensor.0 - x).abs() + (sensor.1 - y).abs() <= dist {
				part1_coverage.insert((x, y));
			}
		}
		sensors.push((sensor, dist));
	}
	println!("Part 1: {}", part1_coverage.len());

	// For each sensor, walk the outline of it's coverage area, checking each point just outside the coverage area to see if it's covered by another sensor.
	'outer: for i in 0..sensors.len() {
		let sensor = sensors[i].0;
		let dist = sensors[i].1;

		let check_point = |x: i32, y: i32| {
			if x < 0 || y < 0 || x > 4000000 || y > 4000000 {
				return false;
			}

			let mut found = false;
			for j in 0..sensors.len() {
				if i == j {
					continue;
				}
	
				let sensor2 = sensors[j].0;
				let dist2 = sensors[j].1;
				if (sensor2.0 - x).abs() + (sensor2.1 - y).abs() <= dist2 {
					found = true;
					break;
				}
			}

			return !found;
		};

		// Walk upper left edge. This is almost certainly sufficient, as the answer will lay along at least one beacon's upper left sensor edge
		for d in 0..=dist+1 {
			let x = sensor.0 - dist - 1 + d;
			let y = sensor.1 - d;

			if check_point(x, y) {
				println!("Part 2: {}, {} = {}", x, y, x as i64 * 4000000 + y as i64);
				break 'outer;
			}
		}
	}

}