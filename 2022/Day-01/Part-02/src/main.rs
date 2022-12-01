fn main() {
	let env_vars: Vec<String> = std::env::args().collect();
	let run_example = env_vars.len() > 1 && env_vars[1].to_lowercase() == "example";
	let input_path = if run_example { "./example-input.txt" } else { "./input.txt" };
	let input = std::fs::read_to_string(input_path).expect("Should have been able to read the file").replace("\r\n", "\n");

	let mut elves_calories: Vec<i32> = input
		.split("\n\n")
		.map(|elf_inv| elf_inv.split("\n").map(|snack_calories| snack_calories.parse::<i32>().unwrap()).sum::<i32>())
		.collect();
	elves_calories.sort_by(|a, b| b.cmp(a));
	println!("{}", elves_calories[0] + elves_calories[1] + elves_calories[2]);
}
