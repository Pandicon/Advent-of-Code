const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();
const area = input.split(": ")[1].split(", ").map(x => x.slice(2));
const [xArea, yArea] = area;
const xAreaNum = xArea.split("..").map(Number);
const [minX, maxX] = [Math.min(...xAreaNum), Math.max(...xAreaNum)];
const yAreaNum = yArea.split("..").map(Number);
const [minY, maxY] = [Math.min(...yAreaNum), Math.max(...yAreaNum)];

let count = 0;
for (let y = minY; y <= 1000; y += 1) {
	for (let x = 0; x <= maxX; x += 1) {
		count += hits(x, y);
	}
}

saveOutput(count.toString());

function hits(startX, startY) {
	let currentX = 0;
	let currentY = 0;
	let velocityX = startX;
	let velocityY = startY;
	while (currentX <= maxX && currentY >= minY) {
		currentX += velocityX;
		currentY += velocityY;
		velocityX -= Math.sign(velocityX);
		velocityY -= 1;
		if (currentX >= minX && currentX <= maxX && currentY >= minY && currentY <= maxY) return 1;
	}
	return 0;
}