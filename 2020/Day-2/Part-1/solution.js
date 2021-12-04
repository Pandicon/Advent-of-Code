const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
let validPasswords = 0;
for(const line of lines) {
	const split = line.split(": ");
	const conditions = split[0].split(" ");
	const password = split[1];
	const letter = conditions[1];
	const minMax = conditions[0].split("-");
	const min = minMax[0];
	const max = minMax[1];
	const regex = new RegExp(letter, "g");
	const occurences = (password.match(regex) || []).length;
	if(occurences >= min && occurences <= max) validPasswords += 1;
};
saveOutput(validPasswords.toString());