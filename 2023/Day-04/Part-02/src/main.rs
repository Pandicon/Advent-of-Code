fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
	let nums = input
		.split('\n')
		.filter_map(|line| if line.is_empty() { None } else { line.split(": ").last() })
		.map(|line| {
			let mut iter = line
				.split(" | ")
				.map(|spl| spl.split(' ').filter(|ch| !ch.is_empty()).map(|num| num.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>());
			let mut winning = iter.next().unwrap();
			winning.sort();
			let mut have = iter.last().unwrap();
			have.sort();
			[winning, have]
		})
		.collect::<Vec<[Vec<u64>; 2]>>();

	let mut quantities = vec![1; nums.len()];
	for (i, [winning, have]) in nums.iter().enumerate() {
		let mut have_winning = 0;
		let mut w_i = 0;
		let mut h_i = 0;
		while w_i < winning.len() && h_i < have.len() {
			if winning[w_i] < have[h_i] {
				w_i += 1;
				continue;
			}
			if winning[w_i] > have[h_i] {
				h_i += 1;
				continue;
			}
			have_winning += 1;
			h_i += 1;
		}
		for j in 0..have_winning {
			quantities[i + j + 1] += quantities[i];
		}
	}
	quantities.iter().sum()
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 30);
	}
}
