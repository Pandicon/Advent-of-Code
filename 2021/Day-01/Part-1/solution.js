const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const numArr = input.split("\n").map(x => parseInt(x));
let increasedAmount = 0;
for(let i = 1; i < numArr.length; i++) {
	if(numArr[i-1] < numArr[i]) increasedAmount++;
};
saveOutput(increasedAmount.toString());