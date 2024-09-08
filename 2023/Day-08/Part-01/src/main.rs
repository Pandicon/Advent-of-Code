use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> usize {
	let mut instructions_paths = input.split("\n\n");
	let instructions = instructions_paths
		.next()
		.unwrap()
		.trim()
		.chars()
		.map(|c| match c {
			'L' => 0,
			'R' => 1,
			_ => {
				println!("{c}");
				unimplemented!()
			}
		})
		.collect::<Vec<usize>>();
	let paths_raw = instructions_paths.next().unwrap().split('\n');
	let mut paths = HashMap::new();
	for path_raw in paths_raw {
		let cleaned = path_raw.replace('(', "").replace(')', "");
		let mut spl = cleaned.split(" = ");
		let start = spl.next().unwrap();
		let start = start.to_owned();
		let paths_from_start = spl.next().unwrap().split(", ").map(|a| a.to_owned()).collect::<Vec<String>>();
		paths.insert(start, [paths_from_start[0].clone(), paths_from_start[1].clone()]);
	}

	let mut current = String::from("AAA");
	let mut i = 0;
	while &current != "ZZZ" {
		let possible_paths = paths.get(&current).unwrap();
		current = possible_paths[instructions[i % (instructions.len())]].clone();
		i += 1;
	}

	i
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
