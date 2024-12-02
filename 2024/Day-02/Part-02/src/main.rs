fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let lines = input
		.split('\n')
		.filter(|s| !s.is_empty());
	let mut res = 0;
	'outer: for line in lines {
		let spl_orig = line.trim().split(" ").map(|num| num.trim().parse::<i64>().expect("All inputs should be numbers")).collect::<Vec<i64>>();
		// An extremely lazy approach - since the sequences are very short, I can just try removing each of the elements one at a time (including the "out of bounds" element, effectively removing no elements) and check if that forms a valid sequence.
		'variants: for i in (0..=spl_orig.len()).rev() {
			let mut spl = Vec::with_capacity(spl_orig.len()-1);
			for j in 0..spl_orig.len() {
				if j == i {
					continue;
				}
				spl.push(spl_orig[j]);
			}
			let increasing = spl[0] - spl[1] > 0;
			for i in 0..spl.len()-1 {
				let diff = spl[i] - spl[i+1];
				if diff == 0 || (diff > 0) != increasing || diff.abs() > 3 {
					continue 'variants;
				}
			}
			res += 1;
			continue 'outer;
		}
	}
	res
	/*'outer: for line in lines {
		let mut spl = line.trim().split(" ").map(|num| num.trim().parse::<i64>().expect("All inputs should be numbers")).collect::<Vec<i64>>();
		let mut increasing = spl[0] - spl[1] > 0;
		let mut faults_tolerated = 0;
		for i in 0..spl.len()-1 {
			let diff = spl[i] - spl[i+1];
			if diff == 0 || (diff > 0) != increasing || diff.abs() > 3 {
				if faults_tolerated > 0 {
					continue 'outer;
				} else {
					faults_tolerated += 1;
					// If a fault occurs on index i, we can fix it by:
					// - Removing the element at index i+1 -> check if i and i+2 work
					// - Removing the element at index i -> check if i-1 and i+1 work
					if i == spl.len()-2 {
						// Remove the last element to fix the issue
						continue;
					}
					if i == 0 {
						// Remove the first element to fix the issue
						increasing = spl[1] - spl[2] > 0;
						continue;
					}
					if i < spl.len() - 2 {
						let diff = spl[i] - spl[i+2];
						if diff != 0 && (diff > 0) == increasing && diff.abs() <= 3 {
							spl[i+1] = spl[i];
							continue;
						}
					}
					if i > 0 {
						let diff = spl[i-1] - spl[i];
						if diff != 0 && (diff > 0) == increasing && diff.abs() <= 3 {
							continue;
						}
					}
					continue 'outer;
				}
			}
		}
		res += 1;
	}*/
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 4);
	}
}
