const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
const digits = lines.map(x => x.split(" | ")[1].split(" "));
let result = 0;
for(const inputDigits of digits) {
	for(const digit of inputDigits) {
		if([2, 3, 4, 7].includes(digit.length)) result += 1;
	};
};
saveOutput(result.toString());