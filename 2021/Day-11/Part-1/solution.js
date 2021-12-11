const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
const octopuses = input.split(/\r/).join("").split("\n").join("").split("").map(Number);
const days = 100;
const octopusesOnLine = lines[0].length;
let result = 0;
for(let i = 0; i < days; i++) {
	let flashed = [];
	for(let j = 0; j < octopuses.length; j++) {
		octopuses[j] += 1;
	}
	let flash = octopuses.findIndex(function(number) {
		return number > 9;
	});
	while(flash > -1) {
		if(flashed.includes(flash)) {
			octopuses[flash] = 0;
			flash = octopuses.findIndex(function(number) {
				return number > 9;
			});
			continue;
		}
		const neighbours = [
			flash-octopusesOnLine,
			flash+octopusesOnLine
		];
		if(flash % octopusesOnLine != 0) neighbours.push(flash-octopusesOnLine-1, flash-1, flash+octopusesOnLine-1);
		if(flash % octopusesOnLine != octopusesOnLine-1) neighbours.push(flash-octopusesOnLine+1, flash+1, flash+octopusesOnLine+1);
		for(const neighbour of neighbours) {
			if(neighbour < 0 || neighbour >= octopuses.length) continue;
			octopuses[neighbour] += 1;
		};
		flashed.push(flash);
		result += 1;
		flash = octopuses.findIndex(function(number) {
			return number > 9;
		});
	}
	for(const i of flashed) octopuses[i] = 0;
}
saveOutput(result.toString());