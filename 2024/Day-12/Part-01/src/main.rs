fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> usize {
	let letters = input.trim().split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
	let mut visited = std::collections::HashSet::new();
	let mut new_regions = vec![[0, 0]];
	let mut res = 0;
	while let Some(start) = new_regions.pop() {
		if visited.contains(&start) {
			continue;
		}
		let letter = letters[start[1]][start[0]];
		let mut to_visit = vec![start];
		let mut area = 0;
		let mut perimeter = 0;
		while let Some(pos) = to_visit.pop() {
			if visited.contains(&pos) {
				continue;
			}
			visited.insert(pos);
			area += 1;
			let [x, y] = pos;
			if x > 0 {
				if letters[y][x-1] == letter {
					to_visit.push([x-1, y]);
				} else {
					new_regions.push([x-1, y]);
					perimeter += 1;
				}
			} else {
				perimeter += 1;
			}
			if x < letters[y].len()-1 {
				if letters[y][x+1] == letter {
					to_visit.push([x+1, y]);
				} else {
					new_regions.push([x+1, y]);
					perimeter += 1;
				}
			} else {
				perimeter += 1;
			}
			if y > 0 {
				if letters[y-1][x] == letter {
					to_visit.push([x, y-1]);
				} else {
					new_regions.push([x, y-1]);
					perimeter += 1;
				}
			} else {
				perimeter += 1;
			}
			if y < letters.len()-1 {
				if letters[y+1][x] == letter {
					to_visit.push([x, y+1]);
				} else {
					new_regions.push([x, y+1]);
					perimeter += 1;
				}
			} else {
				perimeter += 1;
			}
		}
		res += area*perimeter;
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
		assert_eq!(res, 1930);
	}
}
