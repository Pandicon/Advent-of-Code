enum Action {
	Enable,
	Disable,
	Multiply(i64, i64)
}

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let patterns = ["do()", "don't()"];
	let mut actions = Vec::new();
	for pattern in patterns {
		let mut start_index = 0;
		while let Some(index) = input[start_index..].find(&pattern) {
			let abs_index = start_index + index;
			actions.push((abs_index, if pattern == "do()" {Action::Enable} else {Action::Disable}));
			start_index = abs_index + pattern.len();
		}
	}

	let re = regex::Regex::new(r"mul\((?P<num1>[0-9]{1,3}),(?P<num2>[0-9]{1,3})\)").unwrap();
	for caps in re.captures_iter(input.as_str()) {
		if let Some(num_1) = caps.name("num1") {
			if let Some(num_2) = caps.name("num2") {
				let n_1 = num_1.as_str().parse::<i64>().unwrap();
				let n_2 = num_2.as_str().parse::<i64>().unwrap();
				actions.push((num_1.start(), Action::Multiply(n_1, n_2)));}
		}
	}
	actions.sort_by(|a, b| a.0.cmp(&b.0));
	let mut enabled = true;
	let mut res = 0;
	for (_, action) in actions {
		match action {
			Action::Enable => enabled = true,
			Action::Disable => enabled = false,
			Action::Multiply(num_1, num_2) => {
				if enabled {
					res += num_1 * num_2;
				}
			}
		}
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
		assert_eq!(res, 48);
	}
}
