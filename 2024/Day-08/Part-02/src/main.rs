fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn is_antenna(l: char) -> bool {
	('0'..='9').contains(&l) || ('A'..='Z').contains(&l) || ('a'..='z').contains(&l)
}

fn solve(input: String) -> u64 {
	let chars_map = input.trim().split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
	let mut locations = std::collections::HashMap::new();
	for y in 0..chars_map.len() {
		for x in 0..chars_map[y].len() {
			let c = chars_map[y][x];
			if !is_antenna(c) {
				continue;
			}
			let locs: &mut Vec<[isize; 2]> = locations.entry(c).or_default();
			locs.push([x as isize, y as isize]);
		}
	}
	let mut occupied = vec![vec![false; chars_map[0].len()]; chars_map.len()];
	for type_locs in locations.values() {
		for i in 0..type_locs.len() {
			for j in (i+1)..type_locs.len() {
				let [x_1, y_1] = type_locs[i];
				let [x_2, y_2] = type_locs[j];
				let dx = x_2 - x_1;
				let dy = y_2 - y_1;
				let x_0 = x_1;
				let y_0 = y_1;
				let mut i = 0;
				loop {
					let y = y_0 + i*dy;
					let x = x_0 + i*dx;
					if (0..chars_map.len() as isize).contains(&y) {
						let y = y as usize;
						if (0..chars_map[y].len() as isize).contains(&x) {
							let x = x as usize;
							occupied[y][x] = true;
							i += 1;
							continue;
						}
					}
					break;
				}
				let mut i = -1;
				loop {
					let y = y_0 + i*dy;
					let x = x_0 + i*dx;
					if (0..chars_map.len() as isize).contains(&y) {
						let y = y as usize;
						if (0..chars_map[y].len() as isize).contains(&x) {
							let x = x as usize;
							occupied[y][x] = true;
							i -= 1;
							continue;
						}
					}
					break;
				}
			}
		}
	}
	occupied.iter().map(|l| l.iter().filter(|p| **p).count() as u64).sum::<u64>()
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 34);
	}
}
