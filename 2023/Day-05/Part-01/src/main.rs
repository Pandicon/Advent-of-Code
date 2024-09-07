struct MapPart {
	pub start: i64,
	pub end: i64,
	pub transform: i64,
}

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
	let groups = input
		.split("\n\n")
		.map(|group| group.split('\n').filter_map(|line| if line.is_empty() { None } else { Some(line.trim()) }).collect::<Vec<&str>>())
		.collect::<Vec<Vec<&str>>>(); // [seeds, seed-to-soil map, soil-to-fertilizer map, fertilizer-to-water map, fertilizer-to-water map, water-to-light map, light-to-temperature map, temperature-to-humidity map, humidity-to-location map]
	let seeds = groups[0][0]
		.split(": ")
		.last()
		.unwrap()
		.split(' ')
		.filter_map(|num| {
			let trimmed = num.trim();
			if trimmed.is_empty() {
				None
			} else {
				Some(trimmed.parse::<i64>().unwrap())
			}
		})
		.collect::<Vec<i64>>();

	let mut min_location = i64::MAX;
	let maps = groups.iter().skip(1).map(|raw_map| raw_map_to_map(raw_map)).collect::<Vec<Vec<MapPart>>>();
	for seed in seeds {
		let mut val = seed;

		'map_loop: for map in &maps {
			for map_part in map {
				if val >= map_part.start && val <= map_part.end {
					val += map_part.transform;
					continue 'map_loop;
				}
			}
		}

		if val < min_location {
			min_location = val;
		}
	}

	min_location
}

fn raw_map_to_map(raw_map: &Vec<&str>) -> Vec<MapPart> {
	let mut map = Vec::with_capacity(raw_map.len() - 1);
	for line in raw_map.iter().skip(1) {
		let vals = line
			.split(' ')
			.filter_map(|num| {
				let trimmed = num.trim();
				if trimmed.is_empty() {
					None
				} else {
					Some(trimmed.parse::<i64>().unwrap())
				}
			})
			.collect::<Vec<i64>>();
		map.push(MapPart {
			start: vals[1],
			end: vals[1] + vals[2] - 1,
			transform: vals[0] - vals[1],
		});
	}
	map
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 35);
	}
}
