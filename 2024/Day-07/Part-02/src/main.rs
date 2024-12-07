fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

// Look at and try to implement https://www.reddit.com/r/adventofcode/comments/1h8rzsp/comment/m0wcuvu/ (if I ever feel like it)
fn solve(input: String) -> u64 {
	let res_nums = input.trim().split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| {
		let mut spl = l.split(':');
		let res = spl.next().unwrap().parse::<u64>().unwrap();
		let nums = spl.next().unwrap().trim().split(' ').map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();
		(res, nums)
	}).collect::<Vec<(u64, Vec<u64>)>>();
    let mut res = 0;
	'lines: for (expected_res, nums) in res_nums {
		for operators in 0..3_u64.pow(nums.len() as u32-1) {
			let mut curr_operators = operators;
			let mut partial_res = nums[0];
			for j in 1..nums.len() {
				let m = curr_operators % 3;
				curr_operators /= 3;
				partial_res = if m == 0 {
					partial_res + nums[j]
				} else if m == 1 {
					partial_res * nums[j]
				} else {
					let mut n = nums[j];
					while n > 0 {
						n /= 10;
						partial_res *= 10;
					}
					partial_res + nums[j]
				};
			}
			if partial_res == expected_res {
				res += expected_res;
				continue 'lines;
			}
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
		assert_eq!(res, 11387);
	}

	#[test]
	fn manual_tests() {
		let res = solve("213897: 33 7 19 2 9 7 2 3".to_owned());
		assert_eq!(res, 213897);
		let res = solve("2138977: 33 7 19 2 9 7 2 3 7".to_owned());
		assert_eq!(res, 2138977);
		let res = solve("1739: 17 39".to_owned());
		assert_eq!(res, 1739);
	}
}
