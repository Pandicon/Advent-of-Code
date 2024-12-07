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
			set
		}).collect::<Vec<std::collections::HashMap<i64, usize>>>();
	for update in updates {
		let mut used_rules = Vec::new();
		let mut incorrect = false;
		for &[num_1, num_2] in &rules {
			if let Some(i_1) = update.get(&num_1) {
				if let Some(i_2) = update.get(&num_2) {
					used_rules.push([num_1, num_2]);
					if i_1 >= i_2 {
						incorrect = true;
					}
				}
			}
		}
		if !incorrect {
			continue;
		}
		let mut values = update.keys().cloned().collect::<Vec<i64>>();
		for (&val, &i) in update.iter() {
			values[i] = val;
		}
		let mut indices = vec![values.len() as i32 - 1; values.len()];
		let mut solved = Vec::new();
		while solved.len() < values.len() {
			let (last_num_id, _previous) = {
				let mut last_num_id = 0;
				let mut previous = Vec::new();
				'outer: for i in 0..values.len() {
					let val = values[i];
					if solved.contains(&val) {
						continue;
					}
					let mut prev = Vec::new();
					for rule in &used_rules {
						if rule[0] == val && !solved.contains(&rule[1]) {
							continue 'outer;
						}
						prev.push(rule[0]);
					}
					last_num_id = i;
					previous = prev;
					break;
				}
				(last_num_id, previous)
			};
			for rule in &used_rules {
				if rule[1] == values[last_num_id] {
					let checked_val_id = *update.get(&rule[0]).unwrap();
					if indices[checked_val_id] >= indices[last_num_id] {
						indices[checked_val_id] = indices[last_num_id] - 1;
					}
				}
			}
			solved.push(values[last_num_id]);
		}
		let min_id = *indices.iter().min().unwrap();
		for i in 0..indices.len() {
			indices[i] -= min_id;
		}
		let mut sorted = vec![0; values.len()];
		for i in 0..indices.len() {
			let id = indices[i] as usize;
			sorted[id] = values[i];
		}
		res += sorted[(sorted.len()-1)/2];
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
		assert_eq!(res, 123);
	}
}
