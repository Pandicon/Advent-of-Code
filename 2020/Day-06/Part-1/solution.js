const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const groups = input.split(/\r/).join("").split("\n\n");
const answers = groups.map(group => group.split(/ +|\n|/));
let totalAnswers = 0;
for(const groupAnswers of answers) {
	const included = [];
	for(answer of groupAnswers) {
		if(!included.includes(answer)) included.push(answer);
		if(included.length == 26) break;
	};
	totalAnswers += included.length;
};
saveOutput(totalAnswers.toString());