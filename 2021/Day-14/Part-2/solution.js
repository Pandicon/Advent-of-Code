const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const insertNew = (object, key, value = 0) => object[key] = ((key in object) ? object[key] + value : value);

const endOfLine = require("os").EOL;

const input = readInput();
const spl = input.split(`${endOfLine}${endOfLine}`);
let polymer = spl[0];
const rulesArr = spl[1].split(endOfLine);

const rules = {};
for(const rule of rulesArr) {
	rules[rule.split(" -> ")[0]] = rule.split(" -> ")[1];
}

let vals = {};
for (let i = 0; i < polymer.length - 1; i++) insertNew(vals, polymer[i] + polymer[i+1], 1);

for (let i = 0, result = {}; i < 40; i++, result = {}){
    for (const val in vals) {
		insertNew(result, val[0]+rules[val], vals[val]);
		insertNew(result, rules[val]+val[1], vals[val]);
	}
    vals = result;
}

let resVals = {};
for(const key in vals) {
	insertNew(resVals, key[0], vals[key]/2);
	insertNew(resVals, key[1], vals[key]/2);
}
resVals[polymer[0]] += 0.5;
resVals[polymer[polymer.length - 1]] += 0.5;

const sorted = Object.values(resVals).sort((a, b) => b - a);

const result = sorted[0] - sorted[sorted.length-1];
saveOutput(result.toString());