fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

#[derive(Debug)]
struct Gap {
	pub start: i64,
	pub end: i64
}

#[derive(Debug)]
struct Number {
	pub num: i64,
	pub count: i64,
	pub start: i64
}

fn solve(input: String) -> i64 {
	let lengths = input.trim().chars().map(|c| c as i64 - '0' as i64).collect::<Vec<i64>>();
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
			numbers.push(Number { num: i as i64 / 2, count: lengths[i], start: s });
		}
		s += lengths[i];
	}
	for i in (0..numbers.len()).rev() {
		let number = &numbers[i];
		let mut gap_id = None;
		for j in 0..gaps.len() {
			if gaps[j].end + 1 - gaps[j].start >= number.count {
				gap_id = Some(j);
				break;
			}
		}
		if gap_id.is_none() {
			res += number.num * ((number.start + number.count - 1)*(number.start + number.count)-(number.start-1)*number.start) / 2;
			continue;
		}
		let gap_id = gap_id.unwrap();
		if gaps[gap_id].start > number.start {
			res += number.num * ((number.start + number.count - 1)*(number.start + number.count)-(number.start-1)*number.start) / 2;
			continue;
		}
		res += number.num * ((gaps[gap_id].start + number.count - 1)*(gaps[gap_id].start + number.count)-(gaps[gap_id].start-1)*gaps[gap_id].start) / 2;
		gaps[gap_id].start += number.count;
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
		assert_eq!(res, 2858);
	}
}
