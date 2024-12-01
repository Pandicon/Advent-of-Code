fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let lines = input
		.split('\n')
		.filter(|s| !s.is_empty());
	let mut list_1 = Vec::new();
	let mut list_2_counts = std::collections::HashMap::new();
	for line in lines {
		let mut spl = line.trim().split("   ").map(|num| num.trim().parse::<i64>().expect("All inputs should be numbers"));
		list_1.push(spl.next().unwrap());
		let x_2 = spl.next().unwrap();
		let entry = list_2_counts.entry(x_2).or_default();
		*entry += 1;
	}
	let mut res = 0;
	for x_1 in list_1 {
		res += x_1 * *list_2_counts.get(&x_1).unwrap_or(&0);
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
		assert_eq!(res, 31);
	}
}
