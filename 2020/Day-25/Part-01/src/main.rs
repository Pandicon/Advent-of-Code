fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
	let mut keys = input.trim().split('\n').filter_map(|n| n.parse::<u64>().ok());
	let card_key = keys.next().unwrap();
	let door_key = keys.next().unwrap();
	let keys = [card_key, door_key];
	let mut iters = [0, 0];
	for i in 0..keys.len() {
		let key = keys[i];
		let start = 7;
		let mut value = 1;
		let mut iter = 0;
		while value != key {
			value *= start;
			value %= 20201227;
			iter += 1;
		}
		iters[i] = iter;
	}
	let mut handshakes = [0, 0];
	for i in 0..keys.len() {
		let key = keys[i];
		let iter = iters[1-i];
		let start = key;
		let mut value = 1;
		for _ in 0..iter {
			value *= start;
			value %= 20201227;
		}
		handshakes[i] = value;
	}
	assert_eq!(handshakes[0], handshakes[1], "The handshakes must match");
	handshakes[0]
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 14897079);
	}
}
