const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.replace(/\r/, "").split("\n");
const characters = input.split(/\n|\r/).join("").split("");

const lineLength = lines[0].length;
const linesAmount = lines.length;

const sums = [];
for(let i = 0; i < lineLength; i++) sums.push(0);

for(let i = 0; i < characters.length; i++) {
	const character = characters[i];
	const num = parseInt(character);
	if(isNaN(num)) continue;
	sums[i%lineLength] += num;
};

let gammaRateBin = "";
let epsilonRateBin = "";

for(let i = 0; i < lineLength; i++) {
	const roundedVal = Math.round(sums[i]/linesAmount);
	gammaRateBin += roundedVal.toString();
	epsilonRateBin += (1-roundedVal).toString();
};

const gammaRate = parseInt(gammaRateBin, 2);
const epsilonRate = parseInt(epsilonRateBin, 2);
const result = gammaRate*epsilonRate;

saveOutput(result.toString());