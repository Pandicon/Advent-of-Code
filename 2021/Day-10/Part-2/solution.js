const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const opening = ["(", "[", "{", "<"];
const closing = [")", "]", "}", ">"];
const points = {
	")": 1,
	"]": 2,
	"}": 3,
	">": 4
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
let results = [];
for(const line of lines) {
	let expectedClosings = [];
	let validLine = true;
	for(const char of line.split("")) {
		if(opening.includes(char)) {
			expectedClosings = [closing[opening.indexOf(char)], ...expectedClosings];
		} else {
			const expectedClosing = expectedClosings.shift();
			if(expectedClosing != char) validLine = false;
		};
	};
	if(!validLine) continue;
	if(expectedClosings.length > 0) {
		let lineResult = 0;
		for(const char of expectedClosings) {
			lineResult *= 5;
			lineResult += points[char];
		};
		results.push(lineResult);
	};
};
results.sort((a, b) => a - b);
const result = results[Math.floor(results.length/2)];
saveOutput(result.toString());