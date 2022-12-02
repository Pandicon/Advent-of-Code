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
		let me = plays[1] - X_ASCII;
		// A bit of an explanation:
		// me + 1 -> 1 point for X, 2 for Y, and 3 for Z
		// Since we have 0 as Rock, 1 as Paper, and 2 as Scissors, you win by using the one above your opponent
		// However, 2 + 1 is 3, which doesn't exist and you need a rock which is a 0
		// So if you subtract your move from your opponent's move, you will get 1 apart from when you use rock and your opponent uses scissors
		// Then you will get -2 -> .rem_euclid(3) gives the modulo of 3, which makes -2 a 1
		// So if you win, you always get a 1 from that
		// If there is a draw, you used the same symbols -> always a 0
		// If you lose, you had to use the symbol below your opponent's one, which will always give you a 2
		// If you now add a 1 to this, you will get: Lose - 3, Draw - 1, Win - 2
		// Now modulo 3 that is: Lose - 0, Draw - 1, Win - 2
		// If you multiply that by 3, you get the points for the outcomes :D
		score += me + 1 + (((me - opponent).rem_euclid(3) + 1) % 3) * 3;
	}
	println!("{}", score);
}
