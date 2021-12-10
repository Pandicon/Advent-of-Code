const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const target = 2020;
const input = readInput();
const numArr = input.split(/\r/).join("").split("\n").map(x => parseInt(x));
const sorted = numArr.sort((a, b) => a - b);
let i = 0;
let j = sorted.length-1;
while(i < j) {
	const currSum = sorted[i] + sorted[j];
	if(currSum == target) break;
	else if(currSum > target) j -= 1;
	else i += 1;
};
const result = sorted[i]*sorted[j];
saveOutput(result.toString());