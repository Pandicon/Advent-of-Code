fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> usize {
	let heights = input.trim().split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect::<Vec<Vec<u8>>>();
	let mut zeros = Vec::new();
	for y in 0..heights.len() {
		for x in 0..heights[y].len() {
			if heights[y][x] == 0 {
				zeros.push([x, y]);
			}
		}
	}
	let mut res = 0;
	for [x_0, y_0] in zeros {
		let mut reached = std::collections::HashSet::new();
		let mut queue = vec![[x_0, y_0]];
		while let Some([x, y]) = queue.pop() {
			let val = heights[y][x];
			if val == 9 {
				reached.insert([x, y]);
				continue;
			}
			// Left
			if x > 0 && heights[y][x - 1] == val + 1 {
				queue.push([x - 1, y]);
			}
			// Up
			if y > 0 && heights[y - 1][x] == val + 1 {
				queue.push([x, y - 1]);
			}
			// Right
			if x < heights[y].len() - 1 && heights[y][x + 1] == val + 1 {
				queue.push([x + 1, y]);
			}
			// Down
			if y < heights.len() - 1 && heights[y + 1][x] == val + 1 {
				queue.push([x, y + 1]);
			}
		}
		res += reached.len();
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
		assert_eq!(res, 36);
	}
}
