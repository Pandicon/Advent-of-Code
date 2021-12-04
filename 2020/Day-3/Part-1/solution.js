const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
const lineLength = lines[0].length;
const linesAmount = lines.length;
let treesPassed = 0;
let x = 0;
let y = 0;
while(y < linesAmount) {
	let xMod = x % lineLength;
	if(lines[y][xMod] == "#") treesPassed += 1;
	x += 3;
	y += 1;
};
saveOutput(treesPassed.toString());