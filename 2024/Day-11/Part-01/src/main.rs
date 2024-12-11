fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> usize {
	let stones_in = input.trim().split(' ').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.parse().unwrap()).collect::<Vec<u64>>();
	let mut res = 0;
	// The stones do not affect each other, so I can process them one at a time
	// I didn't look at the actual input and was worried it would be extremely long, making it impossible to simulate the whole thing at once, but I was wrong...
	for stone_in in stones_in {
		let mut stones = vec![stone_in];
		for _ in 0..25 {
			for i in (0..stones.len()).rev() {
				let stone = stones[i];
				let stone_str = stone.to_string();
				if stone == 0 {
					stones[i] = 1;
				} else if stone_str.len() % 2 == 0 {
					stones[i] = stone_str[0..stone_str.len()/2].parse().unwrap();
					stones.insert(i+1, stone_str[stone_str.len()/2..stone_str.len()].parse().unwrap());
				} else {
					stones[i] *= 2024;
				}
			}
		}
		res += stones.len();
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
		assert_eq!(res, 55312);
	}
}
