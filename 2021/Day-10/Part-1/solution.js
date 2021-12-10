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
	")": 3,
	"]": 57,
	"}": 1197,
	">": 25137
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
let result = 0;
for(const line of lines) {
	let expectedClosings = [];
	for(const char of line.split("")) {
		if(opening.includes(char)) {
			expectedClosings = [closing[opening.indexOf(char)], ...expectedClosings];
		} else {
			const expectedClosing = expectedClosings.shift();
			if(expectedClosing != char) result += points[char];
		};
	};
};
saveOutput(result.toString());