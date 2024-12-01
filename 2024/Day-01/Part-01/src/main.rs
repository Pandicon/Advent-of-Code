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
	let mut list_2 = Vec::new();
	for line in lines {
		let mut spl = line.trim().split("   ").map(|num| num.trim().parse::<i64>().expect("All inputs should be numbers"));
		list_1.push(spl.next().unwrap());
		list_2.push(spl.next().unwrap());
	}
	list_1.sort();
	list_2.sort();
	let mut res = 0;
	for (x_1, x_2) in list_1.iter().zip(list_2) {
		res += (x_1 - x_2).abs();
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
		assert_eq!(res, 11);
	}
}
