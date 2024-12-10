fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

#[derive(Debug)]
struct Gap {
	pub start: u64,
	pub end: u64
}

#[derive(Debug)]
struct Number {
	pub num: u64,
	pub count: u64,
	pub start: u64
}

fn solve(input: String) -> u64 {
	let lengths = input.trim().chars().map(|c| c as u64 - '0' as u64).collect::<Vec<u64>>();
	let mut gaps = Vec::new();
	let mut numbers = Vec::new();
	let mut res = 0;
	let mut s = 0;
	for i in 0..lengths.len() {
		if i % 2 == 1 {
			if lengths[i] > 0 {
				gaps.push(Gap { start: s, end: s+lengths[i]-1 });
			}
		} else {
			numbers.push(Number { num: i as u64 / 2, count: lengths[i], start: s });
		}
		s += lengths[i];
	}
	'gaps: for i in 0..gaps.len() {
		let mut gap_len = gaps[i].end - gaps[i].start + 1;
		while gap_len > 0 {
			if numbers.is_empty() {
				break 'gaps;
			}
			let last_num_i = numbers.len() - 1;
			let num_count = numbers[last_num_i].count.min(gap_len);
			if gaps[i].start >= numbers[last_num_i].start + numbers[last_num_i].count {
				break 'gaps;
			}
			// 3*6 + 3*7 + 3*8 + 3*9 = 3*(6+7+8+9) = 3*(9*(9+1) - 5*(5+1))/2
			let start = gaps[i].end + 1 - gap_len;
			let end = start + num_count - 1;
			res += numbers[last_num_i].num * (end*(end+1)-(start-1)*start) / 2;
			gap_len -= num_count;
			if num_count < numbers[last_num_i].count {
				numbers[last_num_i].count -= num_count;
			} else {
				numbers.pop();
			}
		}
	}
	// No need to account for 0
	for number in numbers.iter().skip(1) {
		res += number.num * ((number.start + number.count - 1)*(number.start + number.count)-(number.start-1)*number.start) / 2;
	}
	res
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 1928);
	}

	#[test]
	fn other_tests() {
		let res = solve(String::from("12345"));
		assert_eq!(res, 60);
	}
}
