const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const linesCoveringPoints = {};

const input = readInput();
const lines = input.split(/\r/).join("").split(/\n/);
for(const line of lines) {
	const [start, end] = line.split(" -> ");
	let [startX, startY] = start.split(",");
	let [endX, endY] = end.split(",");
	[startX, startY, endX, endY] = [parseInt(startX), parseInt(startY), parseInt(endX), parseInt(endY)];
	let add = [0, 0];
	if(startX == endX) add = [0, Math.sign(endY - startY)];
	else if(startY == endY) add = [Math.sign(endX - startX), 0];
	else if(Math.abs(endX - startX) == Math.abs(endY - startY)) add = [Math.sign(endX - startX), Math.sign(endY - startY)];
	else continue;
	let [x, y] = [startX, startY];
	let reachedEnd = false;
	while(!reachedEnd) {
		reachedEnd = x == endX && y == endY;
		const pointId = `${x}|${y}`;
		if(!linesCoveringPoints[pointId]) linesCoveringPoints[pointId] = 0;
		linesCoveringPoints[pointId] += 1;
		x += add[0];
		y += add[1];
	};
};

let result = 0;
for(const key in linesCoveringPoints) {
	const value = linesCoveringPoints[key];
	if(value >= 2) result += 1;
};

saveOutput(result.toString());