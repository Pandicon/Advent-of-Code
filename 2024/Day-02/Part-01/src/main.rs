fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let lines = input
		.split('\n')
		.filter(|s| !s.is_empty());
	let mut res = 0;
	'outer: for line in lines {
		let spl = line.trim().split(" ").map(|num| num.trim().parse::<i64>().expect("All inputs should be numbers")).collect::<Vec<i64>>();
		let increasing = spl[0] - spl[1] > 0;
		for i in 0..spl.len()-1 {
			let diff = spl[i] - spl[i+1];
			if diff == 0 || (diff > 0) != increasing || diff.abs() > 3 {
				continue 'outer;
			}
		}
		res += 1;
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
		assert_eq!(res, 2);
	}
}
