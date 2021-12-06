const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.replace(/\r/, "").split("\n");
let fish = lines[0].split(",").map(x => parseInt(x));
const days = 80;
for(let i = 0; i < days; i++) {
	fish = fish.map(x => x-1);
	let k = fish.indexOf(-1);
	while(k >= 0) {
		fish[k] = 6;
		fish.push(8);
		k = fish.indexOf(-1);
	};
};
const result = fish.length;
saveOutput(result.toString());