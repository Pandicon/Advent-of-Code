#[derive(PartialEq, Clone, Copy)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

const DIRECTIONS: [[Direction; 2]; 6] = [
	[Direction::Up, Direction::Down],    // |
	[Direction::Left, Direction::Right], // -
	[Direction::Up, Direction::Right],   // L
	[Direction::Up, Direction::Left],    // J
	[Direction::Down, Direction::Left],  // 7
	[Direction::Down, Direction::Right], // F
];

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
	let grid = input.split('\n').map(|line| line.trim().chars().collect()).collect::<Vec<Vec<char>>>();
	let mut pos = [0, 0];
	for (y, line) in grid.iter().enumerate() {
		for (x, c) in line.iter().enumerate() {
			if *c == 'S' {
				pos = [x, y];
			}
		}
	}
	let mut last_dir = Direction::Up;
	for dir in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
		let move_in = move_in_direction(dir);
		if pos[0] == 0 && move_in[0] == -1 || pos[0] == grid[pos[1]].len() - 1 && move_in[0] == 1 || pos[1] == 0 && move_in[1] == -1 || pos[1] == grid.len() - 1 && move_in[1] == 1 {
			continue;
		}
		let [new_x, new_y] = [((pos[0] as isize) + move_in[0]) as usize, ((pos[1] as isize) + move_in[1]) as usize];
		if let Some(i) = char_to_index(grid[new_y][new_x]) {
			if DIRECTIONS[i].contains(&invert_direction(dir)) {
				pos = [new_x, new_y];
				last_dir = dir;
			}
		}
	}
	let mut len = 1;
	while grid[pos[1]][pos[0]] != 'S' {
		let dirs = DIRECTIONS[char_to_index(grid[pos[1]][pos[0]]).expect("The loop should be continuous")];
		let move_in_dir = dirs.iter().filter(|&dir| *dir != invert_direction(last_dir)).next().expect("One of the directions should be valid");
		let move_in = move_in_direction(*move_in_dir);
		let [new_x, new_y] = [((pos[0] as isize) + move_in[0]) as usize, ((pos[1] as isize) + move_in[1]) as usize];
		pos = [new_x, new_y];
		last_dir = *move_in_dir;
		len += 1;
	}

	len / 2
}

fn char_to_index(c: char) -> Option<usize> {
	match c {
		'|' => Some(0),
		'-' => Some(1),
		'L' => Some(2),
		'J' => Some(3),
		'7' => Some(4),
		'F' => Some(5),
		_ => None,
	}
}

fn invert_direction(direction: Direction) -> Direction {
	match direction {
		Direction::Up => Direction::Down,
		Direction::Right => Direction::Left,
		Direction::Down => Direction::Up,
		Direction::Left => Direction::Right,
	}
}

fn move_in_direction(direction: Direction) -> [isize; 2] {
	match direction {
		Direction::Up => [0, -1],
		Direction::Right => [1, 0],
		Direction::Down => [0, 1],
		Direction::Left => [-1, 0],
	}
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example_1() {
		let input = std::fs::read_to_string("./example-input-1.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 4);
	}

	#[test]
	fn example_2() {
		let input = std::fs::read_to_string("./example-input-2.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 8);
	}
}
