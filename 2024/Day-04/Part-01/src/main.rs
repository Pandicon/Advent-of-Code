fn main() {
	let input = std::fs::read_to_string("./input.txt").expect("Should have been able to read the file").replace("\r\n", "\n");
	let res = solve(input);
	println!("{res}");
}

fn count_occurrences(line: &str, pattern: &str) -> i64 {
    let mut occurrences = 0;
    let mut start_index = 0;
    while let Some(index) = line[start_index..].find(&pattern) {
        occurrences += 1;
        start_index = start_index + index + 1;
    }
    occurrences
}

fn solve(input: String) -> i64 {
    let lines_chars = input.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut strs = Vec::new();
    // Rows
    for l in 0..lines_chars.len() {
        let mut line = String::new();
        for c in 0..lines_chars[l].len() {
            line.push(lines_chars[l][c]);
        }
        strs.push(line);
    }
    // Columns
    for c in 0..lines_chars[0].len() {
        let mut line = String::new();
        for l in 0..lines_chars.len() {
            line.push(lines_chars[l][c]);
        }
        strs.push(line);
    }
    // Top-left -> bottom-right diagonal
    let mut l_start = lines_chars.len()-1;
    let mut c_start = 0;
    while l_start > 0 {
        let mut l = l_start;
        let mut c = c_start;
        let mut line = String::new();
        while l < lines_chars.len() && c < lines_chars[l].len() {
            line.push(lines_chars[l][c]);
            l += 1;
            c += 1;
        }
        strs.push(line);
        l_start -= 1;
    }
    while c_start < lines_chars[0].len() {
        let mut l = l_start;
        let mut c = c_start;
        let mut line = String::new();
        while l < lines_chars.len() && c < lines_chars[l].len() {
            line.push(lines_chars[l][c]);
            l += 1;
            c += 1;
        }
        strs.push(line);
        c_start += 1;
    }
    // Top-right -> bottom-left diagonal
    let mut l_start = lines_chars.len()-1;
    let mut c_start = lines_chars.len()-1;
    while l_start > 0 {
        let mut l = l_start;
        let mut c = c_start+1;
        let mut line = String::new();
        while l < lines_chars.len() && c > 0 {
            c -= 1;
            line.push(lines_chars[l][c]);
            l += 1;
        }
        strs.push(line);
        l_start -= 1;
    }
    while c_start > 0 {
        let mut l = l_start;
        let mut c = c_start+1;
        let mut line = String::new();
        while l < lines_chars.len() && c > 0 {
            c -= 1;
            line.push(lines_chars[l][c]);
            l += 1;
        }
        strs.push(line);
        c_start -= 1;
    }

    let patterns = ["XMAS", "SAMX"];
    let mut res = 0;
    for pattern in patterns {
        for line in &strs {
            res += count_occurrences(line, &pattern);
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
		assert_eq!(res, 18);
	}
}
