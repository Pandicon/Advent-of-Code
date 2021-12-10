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
	if(startX > endX) [startX, endX] = [endX, startX];
	if(startY > endY) [startY, endY] = [endY, startY];
	let [changingStart, changingEnd, notChanging] = [0, 0, 0];
	if(startX == endX) [changingStart, changingEnd, notChanging] = [startY, endY, startX];
	else if(startY == endY) [changingStart, changingEnd, notChanging] = [startX, endX, startY];
	else continue;
	for(let i = changingStart; i <= changingEnd; i++) {
		const pointId = startX == endX ? `${notChanging}|${i}` : `${i}|${notChanging}`;
		if(!linesCoveringPoints[pointId]) linesCoveringPoints[pointId] = 0;
		linesCoveringPoints[pointId] += 1;
	};
};

let result = 0;
for(const key in linesCoveringPoints) {
	const value = linesCoveringPoints[key];
	if(value >= 2) result += 1;
};

saveOutput(result.toString());