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
    let map_chars = input.trim().split('\n').map(|line| line.trim()).filter(|line| !line.is_empty()).map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
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
	let mut visited = std::collections::HashSet::new();
	let mut heading = Heading::Up;
	'outer: while pos_c < map_chars.len() && pos_r < map_chars[pos_c].len() {
		match heading {
			Heading::Up => {
				while pos_r > 0 {
					visited.insert([pos_r, pos_c]);
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
					visited.insert([pos_r, pos_c]);
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
					visited.insert([pos_r, pos_c]);
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
					visited.insert([pos_r, pos_c]);
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
	visited.insert([pos_r, pos_c]);
    visited.len()
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 41);
	}
}
