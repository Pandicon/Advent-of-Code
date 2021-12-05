const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const seatCodes = input.split(/\r/).join("").split("\n");
let highestId = -1;
for(const seatCode of seatCodes) {
	const rowCode = seatCode.slice(0, seatCode.length-3);
	const columnCode = seatCode.slice(-3);
	const rowId = parseInt(rowCode.replace(/F/g, "0").replace(/B/g, "1"), 2);
	const columnId = parseInt(columnCode.replace(/L/g, "0").replace(/R/g, "1"), 2);
	const seatId = rowId*8+columnId;
	highestId = Math.max(highestId, seatId);
};
saveOutput(highestId.toString());