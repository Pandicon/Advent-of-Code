#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Heading {
	Up,
	Right,
	Down,
	Left
}

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> usize {
    let mut map_chars = input.trim().split('\n').map(|line| line.trim()).filter(|line| !line.is_empty()).map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
	let mut pos_r = 0;
	let mut pos_c = 0;
	'outer: for i in 0..map_chars.len() {
		for j in 0..map_chars[i].len() {
			if map_chars[i][j] == '^' {
				pos_r = i;
				pos_c = j;
				break 'outer;
			}
		}
	}
	let start_r = pos_r;
	let start_c = pos_c;
	let mut res = 0;
	// A lazy solution - try putting obstacles into each spot one by one and check which spots create cycles
	for i in 0..map_chars.len() {
		for j in 0..map_chars[i].len() {
			if map_chars[i][j] == '^' || map_chars[i][j] == '#' {
				continue;
			}
			map_chars[i][j] = '#';
			let mut pos_r = start_r;
			let mut pos_c = start_c;
			let mut visited = std::collections::HashSet::new();
			let mut heading = Heading::Up;
			'outer: while pos_c < map_chars.len() && pos_r < map_chars[pos_c].len() {
				match heading {
					Heading::Up => {
						while pos_r > 0 {
							// A cycle happens when the guard moves along the same path twice -> is at the same spot and moves in the same direction twice
							if visited.contains(&(heading, [pos_r, pos_c])) {
								res += 1;
								break 'outer;
							}
							visited.insert((heading, [pos_r, pos_c]));
							if map_chars[pos_r-1][pos_c] == '#' {
								heading = Heading::Right;
								continue 'outer;
							}
							pos_r -= 1;
						}
						if pos_r == 0 {
							break 'outer;
						}
					},
					Heading::Down => {
						while pos_r < map_chars.len()-1 {
							if visited.contains(&(heading, [pos_r, pos_c])) {
								res += 1;
								break 'outer;
							}
							visited.insert((heading, [pos_r, pos_c]));
							if map_chars[pos_r+1][pos_c] == '#' {
								heading = Heading::Left;
								continue 'outer;
							}
							pos_r += 1;
						}
						if pos_r == map_chars.len()-1 {
							break 'outer;
						}
					},
					Heading::Left => {
						while pos_c > 0 {
							if visited.contains(&(heading, [pos_r, pos_c])) {
								res += 1;
								break 'outer;
							}
							visited.insert((heading, [pos_r, pos_c]));
							if map_chars[pos_r][pos_c-1] == '#' {
								heading = Heading::Up;
								continue 'outer;
							}
							pos_c -= 1;
						}
						if pos_c == 0 {
							break 'outer;
						}
					},
					Heading::Right => {
						while pos_c < map_chars[pos_r].len()-1 {
							if visited.contains(&(heading, [pos_r, pos_c])) {
								res += 1;
								break 'outer;
							}
							visited.insert((heading, [pos_r, pos_c]));
							if map_chars[pos_r][pos_c+1] == '#' {
								heading = Heading::Down;
								continue 'outer;
							}
							pos_c += 1;
						}
						if pos_c == map_chars[pos_r].len()-1 {
							break 'outer;
						}
					}
				};
			}
			map_chars[i][j] = '.';
		}
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
		assert_eq!(res, 6);
	}
}
