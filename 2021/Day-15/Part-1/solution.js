const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

function getNeighbours(x, y, lines) {
	const neighbours = [];
	if(x - 1 > 0) neighbours.push([x - 1, y]);
	if(x + 1 < lines[y].length) neighbours.push([x + 1, y]);
	if(y - 1 > 0) neighbours.push([x, y - 1]);
	if(y + 1 < lines.length) neighbours.push([x, y + 1]);
	return neighbours;
}

const endOfLine = require("os").EOL;

const input = readInput();
const lines = input.split(endOfLine);
const end = [lines[lines.length - 1].length - 1, lines.length - 1];

const queue = [];
for(let y = 0; y < lines.length; y++) {
	for(let x = 0; x < lines[y].length; x++) {
		let totalRisk = x + y == 0 ? 0 : Number.MAX_SAFE_INTEGER;
		queue.push([x, y, +lines[y][x], totalRisk]);
	}	
}

let endNode = [];
while(queue.length > 1) {
	queue.sort((a, b) => a[3] - b[3]);
	const curr = queue.shift();
	const [x, y, risk, totalRisk] = curr;
	if(x == end[0] && y == end[1]) {
		endNode = curr;
		break;
	}
	const neighbours = getNeighbours(x, y, lines);
	for(const neighbourPos of neighbours) {
		const neighbourIndex = queue.findIndex(element => element[0] == neighbourPos[0] && element[1] == neighbourPos[1]);
		if(neighbourIndex == -1) continue;
		const neighbour = queue[neighbourIndex];
		queue[neighbourIndex][3] = Math.min(totalRisk + neighbour[2], neighbour[3]);
	}
}

const result = endNode[3];
saveOutput(result.toString());