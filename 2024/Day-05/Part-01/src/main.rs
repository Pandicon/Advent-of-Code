fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
    let mut res = 0;
    let mut spl = input.trim().split("\n\n");
    let rules = spl.next().unwrap().trim().split('\n').filter(|l| !l.is_empty()).map(|l| {
		let mut spl = l.trim().split('|');
		let num_1 = spl.next().unwrap().parse::<i64>().unwrap();
		let num_2 = spl.next().unwrap().parse::<i64>().unwrap();
		[num_1, num_2]
	}).collect::<Vec<[i64; 2]>>();
    let updates = spl.next().unwrap().trim().split('\n').filter(|l| !l.is_empty())
		.map(|l| {
			let list = l.split(',').map(|n| n.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
			let mut set = std::collections::HashMap::new();
			for i in 0..list.len() {
				set.insert(list[i], i);
			}
			(list[(list.len()-1)/2], set)
		}).collect::<Vec<(i64, std::collections::HashMap<i64, usize>)>>();
	'update: for (middle, update) in updates {
		for &[num_1, num_2] in &rules {
			if let Some(i_1) = update.get(&num_1) {
				if let Some(i_2) = update.get(&num_2) {
					if i_1 >= i_2 {
						continue 'update;
					}
				}
			}
		}
		res += middle;
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
		assert_eq!(res, 143);
	}
}
