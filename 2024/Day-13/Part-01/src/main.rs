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
		let cap_b = buttons_re.captures(lines[i+1]).unwrap();
		let (b_x, b_y) = (cap_b.name("x_val").unwrap().as_str().parse::<i64>().unwrap(), cap_b.name("y_val").unwrap().as_str().parse::<i64>().unwrap());
		let cap_prize = prize_re.captures(lines[i+2]).unwrap();
		let (prize_x, prize_y) = (cap_prize.name("x_val").unwrap().as_str().parse::<i64>().unwrap(), cap_prize.name("y_val").unwrap().as_str().parse::<i64>().unwrap());
		let mut min_price = i64::MAX;
		for a in 0..=100 {
			for b in 0..=100 {
				if prize_x == a*a_x + b*b_x && prize_y == a*a_y + b*b_y {
					min_price = std::cmp::min(min_price, 3*a + 1*b);
				}
			}
		}
		if min_price < i64::MAX {
			res += min_price;
		}
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
		assert_eq!(res, 480);
	}
}
