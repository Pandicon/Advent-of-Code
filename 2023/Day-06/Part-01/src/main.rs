struct Race {
	pub duration: u64,
	pub record: u64,
}

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
	let mut times_distances = input.split("\n").map(|group| {
		group
			.split(": ")
			.last()
			.unwrap()
			.trim()
			.split(' ')
			.filter_map(|num| if num.is_empty() { None } else { Some(num.trim().parse::<u64>().unwrap()) })
			.collect::<Vec<u64>>()
	});
	let times = times_distances.next().unwrap();
	let records = times_distances.next().unwrap();

	let races = times.iter().zip(records.iter()).map(|(time, record)| Race { duration: *time, record: *record }).collect::<Vec<Race>>();

	let mut prod = 1;
	for race in races {
		let duration = race.duration;
		let record = race.record;

		// The distance we can go is t(T - t), where t is the time spent pressing the button and T is the total race duration
		// The maximum of this happens at t = T/2, so we can check at this time to see if we can even beat the record
		// T^2 / 4 >= R, where R is the record -> T^2 >= 4*R

		// T^2 <= 4*R -> 0 ways to BEAT the record (we could at most tie it) -> return 0 as the result is the product of all ways
		if duration.pow(2) <= 4 * record {
			return 0;
		}

		if duration.pow(2) > 4 * record {
			// Now solve t(T - t) = R and round the lower solution up and the higher solution down
			// -t^2 + tT - R = 0 -> t = (-T +- sqrt(T^2 - 4R))/(-2) = (T +- sqrt(T^2 - 4R))/2
			// Now the number of ways is the number of integer solutions, so floor(T/2 + sqrt(T^2 - 4R)/2) - ceil(T/2 - sqrt(T^2 - 4R)/2) + 1 (14, 15, 16 -> 16 - 14 + 1 = 3 as wanted)
			let sqrt = ((duration.pow(2) - 4 * record) as f64).powf(0.5);
			let higher = ((duration as f64 + sqrt) / 2.0).floor() as u64;
			let lower = ((duration as f64 - sqrt) / 2.0).ceil() as u64;
			let mut options = (higher - lower) as u64 + 1;
			// Check if we actually BEAT the record as if the higher and/or lower solution is an integer, we can only make a tie with it
			if higher * (duration - higher) <= record {
				options -= 1;
			}
			// In case higher == lower, we don't have to check
			if higher > lower && lower * (duration - lower) <= record {
				options -= 1;
			}
			if options == 0 {
				return 0;
			}
			prod *= options;
		}
	}

	prod
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 288);
	}
}
