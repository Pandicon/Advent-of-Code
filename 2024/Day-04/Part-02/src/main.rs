fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn solve(input: String) -> i64 {
    let lines_chars = input.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut res = 0;
    // Whatever the pattern, A is not in its top or bottom row
    for l in 1..lines_chars.len()-1 {
        // Whatever the pattern, A is not in its left or right column
        for c in 1..lines_chars[l].len()-1 {
            if lines_chars[l][c] != 'A' {
                continue;
            }
            let diag_1 = (lines_chars[l-1][c-1] == 'M' && lines_chars[l+1][c+1] == 'S') || (lines_chars[l-1][c-1] == 'S' && lines_chars[l+1][c+1] == 'M');
            let diag_2 = (lines_chars[l+1][c-1] == 'M' && lines_chars[l-1][c+1] == 'S') || (lines_chars[l+1][c-1] == 'S' && lines_chars[l-1][c+1] == 'M');
            if diag_1 && diag_2 {
                res += 1;
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
		assert_eq!(res, 9);
	}
}
