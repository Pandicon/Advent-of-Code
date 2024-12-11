fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input, 75);
	println!("{res}");
}

fn solve(input: String, repetitions: u64) -> u64 {
	let stones_in = input.trim().split(' ').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.parse().unwrap()).collect::<Vec<u64>>();
	let mut res = 0;
	let mut cache = std::collections::HashMap::new();
	fn simulate_stone(stone: u64, repetitions: u64, cache: &mut std::collections::HashMap<(u64, u64), u64>) -> u64 {
		if repetitions == 0 {
			return 1;
		}
		if let Some(cached_result) = cache.get(&(stone, repetitions)) {
			return *cached_result;
		}
		let stone_str = stone.to_string();
		if stone == 0 {
			let res = simulate_stone(1, repetitions-1, cache);
			cache.insert((1, repetitions-1), res);
			res
		} else if stone_str.len() % 2 == 0 {
			let stone_1 = stone_str[0..stone_str.len()/2].parse().unwrap();
			let total_1 = simulate_stone(stone_1, repetitions-1, cache);
			cache.insert((stone_1, repetitions-1), total_1);
			let stone_2 = stone_str[stone_str.len()/2..stone_str.len()].parse().unwrap();
			let total_2 = simulate_stone(stone_2, repetitions-1, cache);
			cache.insert((stone_2, repetitions-1), total_2);
			total_1 + total_2
		} else {
			let res =  simulate_stone(stone * 2024, repetitions-1, cache);
			cache.insert((stone * 2024, repetitions-1), res);
			res
		}
	}
	for stone_in in stones_in {
		res += simulate_stone(stone_in, repetitions, &mut cache);
	}
	res
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example_1() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input, 6);
		assert_eq!(res, 22);
	}

	#[test]
	fn example_2() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input, 25);
		assert_eq!(res, 55312);
	}
}
