fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input, 101, 103);
	println!("{res}");
}

fn solve(input: String, x_max: i64, y_max: i64) -> usize {
	let steps = 100;
	let p_vs = input.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| {
		let mut spl = l.split(' ');
		let mut p_part = spl.next().unwrap()[2..].split(',');
		let mut v_part = spl.next().unwrap()[2..].split(',');
		[
			[p_part.next().unwrap().parse::<i64>().unwrap(), p_part.next().unwrap().parse::<i64>().unwrap()],
			[v_part.next().unwrap().parse::<i64>().unwrap(), v_part.next().unwrap().parse::<i64>().unwrap()]
		]
	}).collect::<Vec<[[i64; 2]; 2]>>();
	let mut robots_in_quadrants = [0, 0, 0, 0];
	for [[p_x, p_y], [v_x, v_y]] in p_vs {
		let final_x_raw = p_x + v_x * steps;
		let final_y_raw = p_y + v_y * steps;
		let mul_x = (final_x_raw as f64 / x_max as f64).floor() as i64;
		let mul_y = (final_y_raw as f64 / y_max as f64).floor() as i64;
		let final_x = final_x_raw - mul_x * x_max;
		let final_y = final_y_raw - mul_y * y_max;
		let left = if (0..x_max/2).contains(&final_x) { 0 } else if final_x > x_max / 2 { 1 } else { 2 };
		let top = if (0..y_max/2).contains(&final_y) { 0 } else if final_y > y_max / 2 { 1 } else { 2 };
		if left == 2 || top == 2 {
			continue;
		}
		robots_in_quadrants[2*top + left] += 1;
	}
	robots_in_quadrants.iter().product()
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input, 11, 7);
		assert_eq!(res, 12);
	}
}
