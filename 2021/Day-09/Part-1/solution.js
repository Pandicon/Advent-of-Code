const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const lines = input.split(/\r/).join("").split("\n").map(x => x.split(""));
let result = 0;
for(let i = 0; i < lines.length; i++) {
	for(let j = 0; j < lines[0].length; j++) {
		const curr = +lines[i][j];
		if((!lines[i-1] || curr < +lines[i-1][j]) && (!lines[i+1] || curr < +lines[i+1][j]) && (!lines[i][j-1] || curr < +lines[i][j-1]) && (!lines[i][j+1] || curr < +lines[i][j+1])) result += curr+1;
	};
};
saveOutput(result.toString());