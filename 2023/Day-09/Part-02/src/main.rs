fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let sequences = input
		.split('\n')
		.map(|seq| seq.split(' ').map(|num| num.trim().parse::<i64>().unwrap()).rev().collect()) // Just reverse the sequences you are given and then use the solution to part 1
		.collect::<Vec<Vec<i64>>>();

	let mut total = 0;
	for sequence in sequences {
		let mut subsequences = vec![sequence.clone()];
		let mut last = 0;

		while !subsequences[last].iter().all(|&n| n == 0) {
			subsequences.push(Vec::new());
			last += 1;
			for i in 0..(subsequences[last - 1].len() - 1) {
				let i_1 = subsequences[last - 1][i + 1];
				let i_0 = subsequences[last - 1][i];
				subsequences[last].push(i_1 - i_0);
			}
		}
		for i in (0..(subsequences.len() - 1)).rev() {
			let len_i = subsequences[i].len();
			let len_i_1 = subsequences[i + 1].len();
			let n_i = subsequences[i][len_i - 1];
			let n_i_1 = subsequences[i + 1][len_i_1 - 1];
			subsequences[i].push(n_i + n_i_1);
		}
		total += subsequences[0][subsequences[0].len() - 1];
	}

	total
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
