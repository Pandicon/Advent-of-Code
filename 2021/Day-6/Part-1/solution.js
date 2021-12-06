const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.replace(/\r/, "").split("\n");
let fishRaw = lines[0].split(",").map(x => parseInt(x));
const fish = new Array(9).fill(0);
for(const d of fishRaw) {
	fish[d] += 1;
};
const days = 80;
for(let i = 0; i < days; i++) {
	const reproducingFish = fish.shift();
	fish.push(reproducingFish);
	fish[6] += reproducingFish;
};
const result = fish.reduce((a, b) => a+b);
saveOutput(result.toString());