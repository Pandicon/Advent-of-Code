const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n");
const lineLength = lines[0].length;

let numbers = lines;
for (let i = 0; i < lineLength; i++) {
	if(numbers.length == 1) break;
	let sum = 0;
	for (const number of numbers) {
		sum += parseInt(number[i]);
	};
	const mostCommon = Math.round(sum/numbers.length).toString();
	numbers = numbers.filter(number => number.split("")[i] == mostCommon);
};
const oxygenRating = parseInt(numbers[0], 2);

numbers = lines;
for (let i = 0; i < lineLength; i++) {
	if(numbers.length == 1) break;
	let sum = 0;
	for (const number of numbers) {
		sum += parseInt(number[i]);
	};
	const leastCommon = (1-Math.round(sum/numbers.length)).toString();
	numbers = numbers.filter(number => number.split("")[i] == leastCommon);
};
const CO2Rating = parseInt(numbers[0], 2);
const result = oxygenRating*CO2Rating;

saveOutput(result.toString());