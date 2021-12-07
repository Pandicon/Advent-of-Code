const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.replace(/\r/, "").split("\n");
const crabs = lines[0].split(",").map(Number).sort((a, b) => a-b);
const optimalPlace = crabs[Math.ceil(crabs.length/2)-1];
let result = 0;
for(const crabPosition of crabs) {
	const distance = Math.abs(crabPosition - optimalPlace)
	result += distance;
};
saveOutput(result.toString());