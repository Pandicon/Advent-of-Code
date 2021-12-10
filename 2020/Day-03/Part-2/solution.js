const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const slopes = [
	[1, 1],
	[3, 1],
	[5, 1],
	[7, 1],
	[1, 2]
];

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
const lineLength = lines[0].length;
const linesAmount = lines.length;
let output = 1;
for(const slope of slopes) {
	let treesPassed = 0;
	let x = 0;
	let y = 0;
	while(y < linesAmount) {
		let xMod = x % lineLength;
		if(lines[y][xMod] == "#") treesPassed += 1;
		x += slope[0];
		y += slope[1];
	};
	output *= treesPassed;
};
saveOutput(output.toString());