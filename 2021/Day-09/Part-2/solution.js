const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n").map(x => x.split(""));

let point = "0|0";
const visited = [];
const basinSizes = [];
while(point != "") {
	point = ""
	for(let i = 0; i < lines.length; i++) {
		for(let j = 0; j < lines[0].length; j++) {
			const curr = +lines[i][j];
			if(curr != 9 && !visited.includes(`${j}|${i}`)) point = `${j}|${i}`;
		};
	};
	if(point == "") break;
	const basin = getBasin(point, lines);
	visited.push(...basin);
	basinSizes.push(basin.length);
};
const sorted = basinSizes.sort((a, b) => b - a);
const result = sorted[0]*sorted[1]*sorted[2];
saveOutput(result.toString());

function getBasin(point, lines) {
	const basin = [];
	const queue = [point];
	const visited = {};
	visited[point] = true;
	while(queue.length > 0) {
		const curr = queue.shift();
		const [x, y] = curr.split("|").map(Number);
		basin.push(curr);
		const val = +lines[y][x];
		if(val == 9) continue;
		const neighbours = getNeighbours(x, y, lines);
		for(const neighbour of neighbours) {
			const [x2, y2] = neighbour.split("|").map(Number);
			const neighbourVal = +lines[y2][x2];
			if(neighbourVal == 9 || visited[neighbour]) continue;
			queue.push(neighbour);
			visited[neighbour] = true;
		}
	}
	return basin
}

function getNeighbours(x, y, lines) {
	const neighbours = [];
	if(x + 1 < +lines[0].length) neighbours.push(`${x+1}|${y}`);
	if(x - 1 >= 0) neighbours.push(`${x-1}|${y}`);
	if(y + 1 < +lines.length) neighbours.push(`${x}|${y+1}`);
	if(y - 1 >= 0) neighbours.push(`${x}|${y-1}`);
	return neighbours;
}