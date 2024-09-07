#[derive(Debug)]
struct Hand {
	pub hand: [char; 5],
	pub hand_sorted: [char; 5],
	pub bid: u64,
}

fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> u64 {
	let mut hands_bids = input
		.split("\n")
		.map(|hand_bid| {
			let mut hand_bid = hand_bid.trim().split(" ");
			let mut hand_chars = hand_bid.next().unwrap().chars();
			let bid = hand_bid.next().unwrap().parse::<u64>().unwrap();
			let hand = [
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
			];
			let mut hand_sorted = hand.clone();
			hand_sorted.sort();
			Hand { hand, hand_sorted, bid }
		})
		.collect::<Vec<Hand>>();
	hands_bids.sort_by(|a, b| score_hand(a).cmp(&score_hand(b)));
	let mut total = 0;
	for (i, hand) in hands_bids.iter().enumerate() {
		total += (i as u64 + 1) * hand.bid;
	}
	total
}

fn score_hand(hand: &Hand) -> u64 {
	let hand_score = score_hand_only(hand);
	let mut letters_scores = hand.hand.map(|c| score_letter(c));
	letters_scores.reverse();
	let mut letters_score = 0;
	for (i, s) in letters_scores.iter().enumerate() {
		letters_score += 14_u64.pow(i as u32) * s; // The 14 is here as the score for the strongest letter +1
	}
	// Maximum score for letters is [13, 13, 13, 13, 13] -> 13 + 14*13 + 14^2*13 + 14^3*13 + 14^4*13 = 14^5 - 1 = about 540000
	let total_score = hand_score * 1_000_000_u64 + letters_score; // This makes sure that it does not matter what you have in your hand if you have a stronger type
	total_score
}

fn score_hand_only(hand: &Hand) -> u64 {
	let mut counts = Vec::new();
	let mut count = 1;
	let mut prev = hand.hand_sorted[0];
	for i in 1..hand.hand_sorted.len() {
		if prev == hand.hand_sorted[i] {
			count += 1;
		} else {
			counts.push(count);
			prev = hand.hand_sorted[i];
			count = 1;
		}
	}
	if count != 0 {
		counts.push(count);
	}
	counts.sort();
	match counts[..] {
		[5] => 6,
		[1, 4] => 5,
		[2, 3] => 4,
		[1, 1, 3] => 3,
		[1, 2, 2] => 2,
		[1, 1, 1, 2] => 1,
		[1, 1, 1, 1, 1] => 0,
		_ => {
			println!("{:?}", counts);
			println!("{:#?}", hand);
			unimplemented!();
		}
	}
}

fn score_letter(letter: char) -> u64 {
	match letter {
		'A' => 13,
		'K' => 12,
		'Q' => 11,
		'J' => 10,
		'T' => 9,
		'9' => 8,
		'8' => 7,
		'7' => 6,
		'6' => 5,
		'5' => 4,
		'4' => 3,
		'3' => 2,
		'2' => 1,
		_ => {
			println!("{:?}", letter);
			unimplemented!();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::solve;

	#[test]
	fn example() {
		let input = std::fs::read_to_string("./example-input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
		let res = solve(input);
		assert_eq!(res, 6440);
	}
}
