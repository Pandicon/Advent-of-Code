const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const endOfLine = require("os").EOL;

const input = readInput();
const split = input.split(`${endOfLine}${endOfLine}`).map(x => x.split(endOfLine));
const dots = split[0].map(a => a.split(",").map(Number));
const instructions = split[1];
const instruction = instructions[0].split(" ")[2].split("=");
const axis = instruction[0];
const coord = instruction[1];
if(axis == "x") {
	for(const i in dots) {
		const dot = dots[i];
		if(dot[0] > coord) dots[i][0] = coord - Math.abs(coord - dot[0])
	}
} else {
	for(const i in dots) {
		const dot = dots[i];
		if(dot[1] > coord) dots[i][1] = coord - Math.abs(coord - dot[1])
	}
}
let seen = [];
for(const dot of dots) {
	let id = `${dot[0]}|${dot[1]}`;
	if(!seen.includes(id)) {
		seen.push(id);
	}
}
const result = seen.length;
saveOutput(result.toString());