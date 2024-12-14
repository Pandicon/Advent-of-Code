fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	solve(input);
	// Now look through the output :D
}

const ROBOT_PRESENT: char = '#';
const MAYBE_TREE_PATTERN: &str = "#####"; // I expect at least 5 robots in a row if they are in any type of tree shape
const X_MAX: usize = 101;
const Y_MAX: usize = 103;

fn solve(input: String) {
	let p_vs = input.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| {
		let mut spl = l.split(' ');
		let mut p_part = spl.next().unwrap()[2..].split(',');
		let mut v_part = spl.next().unwrap()[2..].split(',');
		[
			[p_part.next().unwrap().parse::<i64>().unwrap(), p_part.next().unwrap().parse::<i64>().unwrap()],
			[v_part.next().unwrap().parse::<i64>().unwrap(), v_part.next().unwrap().parse::<i64>().unwrap()]
		]
	}).collect::<Vec<[[i64; 2]; 2]>>();
	let mut positions = p_vs.iter().map(|p_v| p_v[0]).collect::<Vec<[i64; 2]>>();
	let velocities = p_vs.iter().map(|p_v| p_v[1]).collect::<Vec<[i64; 2]>>();
	for t in 0..(X_MAX * Y_MAX) {
		let mut occupied = [[false; X_MAX]; Y_MAX];
		for i in 0..positions.len() {
			let pos = positions[i];
			occupied[pos[1] as usize][pos[0] as usize] = true;
			let [mut x, mut y] = pos;
			x += velocities[i][0];
			y += velocities[i][1];
			while x >= X_MAX as i64 {
				x -= X_MAX as i64;
			}
			while x < 0 {
				x += X_MAX as i64;
			}
			while y >= Y_MAX as i64 {
				y -= Y_MAX as i64;
			}
			while y < 0 {
				y += Y_MAX as i64;
			}
			positions[i] = [x, y];
		}
		let lines = occupied.map(|l| l.map(|o| if o { ROBOT_PRESENT } else { ' ' }).iter().collect::<String>());
		if lines.iter().any(|l| l.contains(MAYBE_TREE_PATTERN)) {
			println!("Time: {t}");
			println!("{}", lines.join("\n"));
			println!("{}", ['-'; X_MAX].iter().collect::<String>());
		}
	}
}
