const A_UPPER_ASCII: i32 = 'A' as i32;
const A_LOWER_ASCII: i32 = 'a' as i32;

fn main() {
	let env_vars: Vec<String> = std::env::args().collect();
	let run_example = env_vars.len() > 1 && env_vars[1].to_lowercase() == "example";
	let input_path = if run_example { "./example-input.txt" } else { "./input.txt" };
	let input = std::fs::read_to_string(input_path).expect("Should have been able to read the file").replace("\r\n", "\n");

	let mut score = 0;
	for line in input.split('\n') {
		let priorities = line
			.chars()
			.map(|c| if c.is_lowercase() { c as i32 - A_LOWER_ASCII + 1 } else { c as i32 - A_UPPER_ASCII + 27 })
			.collect::<Vec<i32>>();
		let halves = priorities
			.chunks((priorities.len() + 1) / 2)
			.map(|chunk| std::collections::HashSet::from_iter(chunk.iter()))
			.collect::<Vec<std::collections::HashSet<&i32>>>();
		let overlap = halves[0].intersection(&halves[1]);
		let val = overlap.map(|v| **v).sum::<i32>();
		score += val;
	}
	println!("{}", score);
}
