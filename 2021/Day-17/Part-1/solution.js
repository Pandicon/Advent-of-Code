const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const area = input.split(": ")[1].split(", ").map(x => x.slice(2));
const yArea = area[1];
const yAreaNum = yArea.split("..").map(Number);
let minY = Math.min(...yAreaNum);
const result = -minY*(-minY-1)/2;
saveOutput(result.toString());