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
let j = 0;
let k = 0;
let l = 0;
for (let i = 0; i < numArr.length; i++) {
	const newTarget = target-numArr[i];
	let solution = getSummingNumbers(newTarget, sorted);
	if(solution) {
		j = i;
		k = solution[0];
		l = solution[1];
		break;
	};
};

const result = sorted[j]*sorted[k]*sorted[l];
saveOutput(result.toString());

function getSummingNumbers(targetNum, sortedNums) {
	let i = 0;
	let j = sortedNums.length-1;
	while(i < j) {
		const currSum = sortedNums[i] + sortedNums[j];
		if(currSum == targetNum) return [i, j];
		else if(currSum > targetNum) j -= 1;
		else i += 1;
	};
	return null;
};