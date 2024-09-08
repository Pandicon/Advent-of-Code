use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
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

	let mut current_places = paths.keys().filter(|start| start.ends_with('A')).cloned().collect::<Vec<String>>();

	let mut loop_afters = Vec::new();
	for i in 0..current_places.len() {
		let mut a = 0;
		let mut visited = Vec::new();
		// Here I construct a sequence of codes that occur until it 100% starts looping over - meaning when I encounter the same code while being at the same instructions index as the last time
		while a == 0 || visited.iter().filter(|(p, m, _)| p == &current_places[i] && *m == (a % (instructions.len()))).count() == 0 {
			visited.push((current_places[i].clone(), a % (instructions.len()), a));
			let possible_paths = paths.get(&current_places[i]).unwrap();
			current_places[i] = possible_paths[instructions[a % (instructions.len())]].clone();
			a += 1;
		}
		let (_code, _m, step) = visited.iter().filter(|(p, m, _)| p == &current_places[i] && *m == (a % (instructions.len()))).next().unwrap().clone();
		visited.push((current_places[i].clone(), a % (instructions.len()), a));
		let loop_after = a - step;
		// Now I know there are codes that end with Z at these indexes + k*loop_after
		// let _end_with_z_indexes = visited.iter().filter(|(p, _, _)| p.ends_with('Z')).map(|(_, _, a)| *a).collect::<Vec<usize>>();

		// println!("{}", loop_after);
		// println!("{:?}", end_with_z_indexes);

		// Here the solution gets pretty random - I printed out (see the above commented out code) the loop_after and end_with_z_indexes for each start field and noticed that end_with_z_indexes == [loop_after]
		// So I have equations in the form of x_i = loop_after_i + k * loop_after_i
		// So I can get the LCM of all the "loop_afters" as the solution
		loop_afters.push(loop_after as u64);
	}
	// Using a * b = LCM(a, b) * GCD(a, b) -> LCM(a, b) = a * b / GCD(a, b)
	// And also LCM(a, b, c) = LCM(a, LCM(b, c))
	let mut lcm = loop_afters[0];
	for num in loop_afters {
		lcm = lcm * num / gcd(num, lcm);
	}

	lcm
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	if a == b {
		return a;
	}
	if b > a {
		std::mem::swap(&mut a, &mut b);
	}
	while b > 0 {
		let temp = a;
		a = b;
		b = temp % b;
	}
	return a;
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
