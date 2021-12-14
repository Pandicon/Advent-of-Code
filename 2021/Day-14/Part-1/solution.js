const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const endOfLine = require("os").EOL;

const input = readInput();
const spl = input.split(`${endOfLine}${endOfLine}`);
let polymer = spl[0];
const rulesArr = spl[1].split(endOfLine);

const rules = {};
for(const rule of rulesArr) {
	rules[rule.split(" -> ")[0]] = rule.split(" -> ")[1];
}

for(let i = 0; i < 10; i++) {
	const currPolymer = polymer;
	let newPolymer = [currPolymer[0]];
	for(let i = 1; i < currPolymer.length; i++) {
		let pair = `${currPolymer[i-1]}${currPolymer[i]}`;
		if(rules[pair]) {
			newPolymer.push(rules[pair]);
		}
		newPolymer.push(currPolymer[i]);
	}
	polymer = newPolymer.join("");
}
let min = [polymer.length, ""];
let max = [0, ""];
const letterCounts = {};
for(let i = 0; i < polymer.length; i++) {
	const letter = polymer[i];
	if(!letterCounts[letter]) letterCounts[letter] = 0;
	letterCounts[letter] += 1;
	if(letterCounts[letter] > max[0]) max = [letterCounts[letter], letter];
}
for(const letter in letterCounts) {
	if(letterCounts[letter] < min[0]) min = [letterCounts[letter], letter];
}

const result = max[0] - min[0];
saveOutput(result.toString());