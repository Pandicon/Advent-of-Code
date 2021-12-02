const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const directions = {
	"forward": [1, 1, 0],
	"up": [0, 0, -1],
	"down": [0, 0, 1]
}

const input = readInput();
const lines = input.split("\n");
let position = [0, 0, 0];
for(line of lines) {
	let splitLine = line.split(" ");
	let direction = splitLine[0];
	let x = parseInt(splitLine[1]);
	
	position[0] += directions[direction][0]*x;
	position[1] += directions[direction][1]*position[2]*x;
	position[2] += directions[direction][2]*x;
};
let result = position[0]*position[1];
saveOutput(result.toString());