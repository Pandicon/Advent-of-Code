fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let lines = input.trim().split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).collect::<Vec<&str>>();
	let buttons_re = regex::Regex::new(r".+: X(?P<x_val>\+?-?[0-9]+), Y(?P<y_val>\+?-?[0-9]+)").unwrap();
	let prize_re = regex::Regex::new(r".+: X=(?P<x_val>\+?-?[0-9]+), Y=(?P<y_val>\+?-?[0-9]+)").unwrap();
	let mut res = 0;
	for i in (0..lines.len()).step_by(3) {
		let cap_a = buttons_re.captures(lines[i]).unwrap();
		let (a_x, a_y) = (cap_a.name("x_val").unwrap().as_str().parse::<i64>().unwrap(), cap_a.name("y_val").unwrap().as_str().parse::<i64>().unwrap());
		let (a_x_f, a_y_f) = (a_x as f64, a_y as f64);
		let cap_b = buttons_re.captures(lines[i+1]).unwrap();
		let (b_x, b_y) = (cap_b.name("x_val").unwrap().as_str().parse::<i64>().unwrap(), cap_b.name("y_val").unwrap().as_str().parse::<i64>().unwrap());
		let (b_x_f, b_y_f) = (b_x as f64, b_y as f64);
		let cap_prize = prize_re.captures(lines[i+2]).unwrap();
		let (prize_x, prize_y) = (cap_prize.name("x_val").unwrap().as_str().parse::<i64>().unwrap() + 10000000000000, cap_prize.name("y_val").unwrap().as_str().parse::<i64>().unwrap() + 10000000000000);
		let (prize_x_f, prize_y_f) = (prize_x as f64, prize_y as f64);
		// We can treat [prize_x, prize_y] as a vector, with [a_x, a_y] and [b_x, b_y] being basis vectors.
		// So the task is to find the components of the prize vector. Unless the basis vectors are collinear, there is only one such set.
		// So we can find them by having a line going from the origin in the [a_x, a_y] direction and another line going from the prize point in the [b_x, b_y] direction.
		// These lines will intersect, and that is the point that determines how many times we use the 'a' vector and how many times the 'b' vector.

		// Check collinearity: a_y / a_x =? b_y / b_x <=> a_y * b_x =? b_y * a_x
		if a_y * b_x == b_y * a_x {
			// Collinear
			if a_y * prize_x != prize_y * a_x {
				// Prize and the basis vectors are not collinear -> unreachable
				continue;
			}
			// Hopefully this never happens
			unimplemented!("Collinear basis vectors");
			// After completing the task I can happily say it never happened :D
			// Else I would need to check which of them is more efficient (if len('a') > 3*len('b') then 'a' is more efficient, else 'b')
			// And then I would need to add them up correctly, trying to use as many of the more efficient vectors as possible
			// I could probably try fitting as many as possible without overshooting and try to fit the less efficient vectors into the remaining space.
			// If it was not possible to fit them, I could try removing some of the more efficient ones until it fits.
			// I could stop once I've removed k more efficient vectors such that k*'more efficient vector' is a multiple of the less efficient vector since the sizes of the remaining spaces will repeat after that point.
			// This limiting k*'more efficient vector' vector could be determined by taking the lcm(a_x, b_x) and this is the x component of the limiting vector, so k = lcm(a_x, b_x) / a_x (if 'a' was more efficient).
		}
		let dy_dx_a = a_y_f / a_x_f;
		let dy_dx_b = b_y_f / b_x_f;
		// Line a: y = dy_dx_a * x
		// Line b: y = prize_y + dy_dx_b * (x - prize_x)
		// Solve for x: dy_dx_a * x = prize_y + dy_dx_b * (x - prize_x) = prize_y + dy_dx_b * x - dy_dx_b * prize_x
		// (dy_dx_a - dy_dx_b) * x = prize_y - dy_dx_b * prize_x -> x = (prize_y - dy_dx_b * prize_x)/(dy_dx_a - dy_dx_b)
		let x_i = (prize_y_f - dy_dx_b * prize_x_f) / (dy_dx_a - dy_dx_b);
		let a = (x_i / a_x_f).round() as i64;
		let b = ((prize_x_f - x_i) / b_x_f).round() as i64;
		if a < 0 || b < 0 {
			continue;
		}
		if a*a_x + b*b_x != prize_x || a* a_y + b* b_y != prize_y {
			continue;
		}
		res += 3*a + 1*b;
	}
	res
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		// 'example-input-modified.txt' contains the example input but with 10000000000000 subtracted from each of the prize coordinates, so that after the transformation they are back to the original example state so I can reuse the example result from before (since no example result is given in part 2).
		let input = std::fs::read_to_string("./example-input-modified.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 480);
	}
}