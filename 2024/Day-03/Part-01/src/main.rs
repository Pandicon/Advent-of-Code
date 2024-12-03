fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
	let mut res = 0;
	for (_, [num1, num2]) in re.captures_iter(input.trim()).map(|c| c.extract()) {
		res += num1.parse::<i64>().unwrap()*num2.parse::<i64>().unwrap();
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
		assert_eq!(res, 161);
	}
}
