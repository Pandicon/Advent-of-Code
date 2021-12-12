const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const endOfLine = require("os").EOL;

const input = readInput();
const ways = input.split(endOfLine).map(way => way.split("-"));
const directWays = {};
for (const [start, end] of ways){
    if (!(start in directWays)) directWays[start] = new Set();
    if (!(end in directWays)) directWays[end] = new Set();
    directWays[start].add(end);
	directWays[end].add(start);
};

const findWays = (start = "start", beenOn = new Set()) => {
	if (start === "end") return 1;

    let seen = beenOn;
    if (start === start.toLowerCase()) seen = new Set([...beenOn, start]);

    let count = 0;
    directWays[start].forEach(end => { 
		if(!seen.has(end)) count += findWays(end, seen);
	});
    return count;
}

const result = findWays();
saveOutput(result.toString());