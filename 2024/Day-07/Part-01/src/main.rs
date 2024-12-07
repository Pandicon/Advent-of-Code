fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
	let res_nums = input.trim().split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| {
		let mut spl = l.split(':');
		let res = spl.next().unwrap().parse::<u64>().unwrap();
		let nums = spl.next().unwrap().trim().split(' ').map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();
		(res, nums)
	}).collect::<Vec<(u64, Vec<u64>)>>();
    let mut res = 0;
	'lines: for (expected_res, nums) in res_nums {
		for operators in 0..2_u64.pow(nums.len() as u32-1) {
			let mut partial_res = nums[0];
			for j in 1..nums.len() {
				partial_res = if (operators >> (j-1)) & 1 == 0 {
					partial_res + nums[j]
				} else {
					partial_res * nums[j]
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
		assert_eq!(res, 3749);
	}
}
