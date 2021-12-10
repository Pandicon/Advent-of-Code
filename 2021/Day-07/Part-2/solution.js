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
const distances = [];
for(let i = 0; i < crabs[crabs.length-1]; i++) {
	distances[i] = 0;
	crabs.forEach((distance, j) => {
		distance = Math.abs(distance - i);
		distances[i] += distance/2*(distance+1);
	});
};
const result = Math.min(...distances);
saveOutput(result.toString());