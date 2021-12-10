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
	const indexes = conditions[0].split("-");
	const i = indexes[0];
	const j = indexes[1];
	if(password[i-1] == letter ^ password[j-1] == letter) validPasswords += 1;
};
saveOutput(validPasswords.toString());