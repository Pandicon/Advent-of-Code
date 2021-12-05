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
	const firstPerson = groupAnswers[0];
	const all = firstPerson.filter(answer => groupAnswers.every(personAnswers => personAnswers.includes(answer)));
	totalAnswers += all.length;
};
saveOutput(totalAnswers.toString());