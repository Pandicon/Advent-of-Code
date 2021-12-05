const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const groups = input.split(/\r/).join("").split("\n\n");
const answersPeople = groups.map(group => group.split(/\n/).map(person => person.split("")));
let totalAnswers = 0;
for(const groupAnswers of answersPeople) {
	let all = [];
	const firstPerson = groupAnswers[0];
	for(const answer of firstPerson) {
		if(!all.includes(answer)) all.push(answer);
	};
	for(const person of groupAnswers) {
		let currAll = all.slice();
		for(const answer of all) {
			if(!person.includes(answer) && currAll.includes(answer)) currAll.splice(currAll.indexOf(answer), 1);
			if(currAll.length == 0) break;
		};
		all = currAll.slice();
		if(all.length == 0) break;
	};
	totalAnswers += all.length;
};
saveOutput(totalAnswers.toString());