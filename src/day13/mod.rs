use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PacketElement
{
	Number(i32),
	List(Vec<PacketElement>),
}

impl PacketElement
{
	fn cmp(&self, other: &PacketElement) -> Ordering
	{
		match (self, other)
		{
			(PacketElement::Number(a), PacketElement::Number(b)) => a.cmp(b),
			(PacketElement::List(a), PacketElement::List(b)) => {
				for (left, right) in a.iter().zip(b.iter()) {
					match left.cmp(&right) {
						Ordering::Equal => continue,
						other => return other,
					}
				}

				return a.len().cmp(&b.len());
			},
			(PacketElement::Number(a), PacketElement::List(_b)) =>  {
				PacketElement::List(vec![PacketElement::Number(*a)]).cmp(other)
			}
			(PacketElement::List(_a), PacketElement::Number(b)) =>  {
				self.cmp(&PacketElement::List(vec![PacketElement::Number(*b)]))
			}
		}
	}

	fn parse(s: &str) -> (PacketElement, usize)
	{
		if s.starts_with('[') {
			let mut index = 1;
			let mut elements = Vec::new();

			loop {
				let ch_next = s.chars().nth(index).unwrap();
				if ch_next == ']' {
					return (PacketElement::List(elements), index + 1);
				} else if ch_next == ',' {
					index += 1;
				} else {
					let (next_element, consumed) = PacketElement::parse(&s[index..]);
					elements.push(next_element);
					index += consumed;
				}
			}
		} else {
			let num_len = s.chars().take_while(|c| c.is_numeric()).count();
			return (PacketElement::Number(s[..num_len].parse::<i32>().unwrap()), num_len);
		}
	}
}

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	let mut all_packets = Vec::new();

	for i in 0..(inputs.len() + 2) / 3 {
		let first = PacketElement::parse(&inputs[i * 3]).0;
		let second = PacketElement::parse(&inputs[i * 3 + 1]).0;

		if first.cmp(&second) == Ordering::Less {
			part1 += i + 1;
		}
		all_packets.push(first);
		all_packets.push(second);
	}

	println!("Part 1: {}", part1);

	let divider1 = PacketElement::parse("[[2]]").0;
	let divider2 = PacketElement::parse("[[6]]").0;
	all_packets.push(divider1.clone());
	all_packets.push(divider2.clone());
	all_packets.sort_by(|a, b| a.cmp(b));

	let divider_pos1 = all_packets.iter().position(|x| x == &divider1).unwrap() + 1;
	let divider_pos2 = all_packets.iter().position(|x| x == &divider2).unwrap() + 1;

	println!("Part 2: {}", divider_pos1 * divider_pos2);
}