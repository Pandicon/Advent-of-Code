const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
let boardsRaw = input.split(/\r/).join("").split("\n\n");
const nums = boardsRaw.shift().split(",");
const boardHeight = 5;
const boardWidth = 5;
const boards = []
const boardsBools = [];
const defaultBoolBoard = [];
for(let i = 0; i < boardHeight*boardWidth; i++) {
	defaultBoolBoard.push(false);
};

for(const board of boardsRaw) {
	boards.push(board.split(/\n| +/).filter(x => x != ""));
	boardsBools.push(defaultBoolBoard.slice());
};

const winnerInfo = getWinningBoard(nums, boards, boardsBools, boardHeight, boardWidth);

const result = getScore(winnerInfo[0], winnerInfo[1], winnerInfo[2]);

saveOutput(result.toString());

function isBoardWinning(changedIndex, boardBools, boardHeight, boardWidth) {
	let x = changedIndex % boardWidth;
	let y = Math.floor(changedIndex/boardHeight);
	let rowIndexes = [];
	let columnIndexes = [];
	for(let i = 0; i < boardWidth; i++) {
		rowIndexes.push(i+y*boardWidth);
	};
	for(let i = 0; i < boardHeight; i++) {
		columnIndexes.push(i*boardHeight+x);
	};
	let done = [true, true];
	for(const i of rowIndexes) {
		if(!boardBools[i]) {
			done[0] = false;
			break;
		};
	};
	for(const i of columnIndexes) {
		if(!boardBools[i]) {
			done[1] = false;
			break;
		};
	};
	return done[0] || done[1];
};

function getWinningBoard(nums, boards, boardsBools, boardHeight, boardWidth) {
	for(const num of nums) {
		for(const boardIndex in boards) {
			const board = boards[boardIndex];
			const numIndex = board.indexOf(num);
			if(numIndex == -1) continue;
			boardsBools[boardIndex][numIndex] = true;
			if(isBoardWinning(numIndex, boardsBools[boardIndex], boardHeight, boardWidth)) return [board, boardsBools[boardIndex], num];
		};
	};
};

function getScore(board, boardBools, finalNumber) {
	let sumOfUnselected = 0;
	for(const i in board) {
		if(!boardBools[i]) sumOfUnselected += parseInt(board[i]);
	};
	return sumOfUnselected*parseInt(finalNumber);
};