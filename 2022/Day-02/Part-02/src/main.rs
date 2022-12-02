const A_ASCII: i32 = 'A' as i32;
const X_ASCII: i32 = 'X' as i32;

fn main() {
	let env_vars: Vec<String> = std::env::args().collect();
	let run_example = env_vars.len() > 1 && env_vars[1].to_lowercase() == "example";
	let input_path = if run_example { "./example-input.txt" } else { "./input.txt" };
	let input = std::fs::read_to_string(input_path).expect("Should have been able to read the file").replace("\r\n", "\n");

	let mut score = 0;
	for line in input.split('\n') {
		let plays = line.trim().split(' ').map(|x| x.chars().next().unwrap() as i32).collect::<Vec<i32>>();
		let opponent = plays[0] - A_ASCII;
		let expected_outcome = plays[1] - X_ASCII;
		// A bit of an explanation:
		// expected_outcome * 3 -> 0 points for losing, 3 for a draw, and 6 for winning
		// To win, you have to use a symbol above your opponent's, but if they use Paper, that would be 3, while Rock is 0 -> solved by .rem_euclid(3), which gives the modulo 3
		// To make a draw, you have to use the same symbol as your opponent
		// To lose, you have to use the symbol below your opponent's, but if they use Rock, that would be -1 -> solved by .rem_euclid(3), which gives the modulo 3 and makes -1 a 2
		// The shifting is done by adding expected outcome - 1 to the opponents symbol, since expected outcome is in the range of 0 - 2, while we need -1 - 1 (so that's why the 1 is subtracted)
		// Finally we add 1, since the symbol chosen is in the range 0 - 2, while the points for the symbols are in the range 1 - 3
		score += expected_outcome * 3 + (opponent + expected_outcome - 1).rem_euclid(3) + 1;
	}
	println!("{}", score);
}
