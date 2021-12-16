const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const input = readInput();

const hex_to_bin = {
    '0': ['0', '0', '0', '0'],
    '1': ['0', '0', '0', '1'],
    '2': ['0', '0', '1', '0'],
    '3': ['0', '0', '1', '1'],
    '4': ['0', '1', '0', '0'],
    '5': ['0', '1', '0', '1'],
    '6': ['0', '1', '1', '0'],
    '7': ['0', '1', '1', '1'],
    '8': ['1', '0', '0', '0'],
    '9': ['1', '0', '0', '1'],
    'A': ['1', '0', '1', '0'],
    'B': ['1', '0', '1', '1'],
    'C': ['1', '1', '0', '0'],
    'D': ['1', '1', '0', '1'],
    'E': ['1', '1', '1', '0'],
    'F': ['1', '1', '1', '1']
};

const operCodes = [
    function sum(...args) {
        return args.reduce(function (acc, cur) {
            return acc + cur;
        });
    },
    function product(...args) {
        return args.reduce(function (acc, cur) {
            return acc * cur;
        });
    },
    function min(...args) {
        return Math.min(...args);
    },
    function max(...args) {
        return Math.max(...args);
    },
    undefined, //...
    function gt(a, b) {
        return a > b ? 1 : 0;
    },
    function lt(a, b) {
        return a < b ? 1 : 0;
    },
    function eq(a, b) {
        return a === b ? 1 : 0;
    }];

function hexToBin(str) {
    const arr = [];
    for (let c = 0; c < str.length; c++) {
        arr.push(...hex_to_bin[str[c]]);
    }
    return arr;
}

function arrPartToDec(arr, start, end) {
    let strPart = '';
    for (let i = start; i <= end; i++) {
        strPart += arr[i];
    }
    return parseInt(strPart, 2);
}

function arrPart(arr, start, end) {
    let strPart = '';
    for (let i = start; i <= end; i++) {
        strPart += arr[i];
    }
    return strPart;
}

function parsePacket(signal, curPos, level) {
    const version = arrPartToDec(signal, curPos, curPos + 2);
    const typeID = arrPartToDec(signal, curPos + 3, curPos + 5);
    curPos = curPos + 6;
    if (typeID === 4) {
        //literal
        let result = '';
        let isNext = true;
        while (isNext) {
            if (signal[curPos] === '0') {
                isNext = false;
            }
            curPos++;
            result += arrPart(signal, curPos, curPos + 3);
            curPos += 4;
        }
        const value = parseInt(result, 2);
        packets.push({ version, typeID, value, level });
    } else {
        //operator
        level++;
        const typeLength = signal[curPos];
        curPos++;
        let length;
        if (typeLength === '0') {
            //15-bit length
            length = arrPartToDec(signal, curPos, curPos + 14);
            curPos += 15;
            let subPacketLength;
            const partEnd = length + curPos;
            do {
                subPacketLength = parsePacket(signal, curPos, level);
                curPos = subPacketLength;
            } while (subPacketLength < partEnd);
            curPos = subPacketLength;
        }
        else {
            //11-bit length
            length = arrPartToDec(signal, curPos, curPos + 10);
            curPos += 11;
            for (let d = 0; d < length; d++) {
                curPos = parsePacket(signal, curPos, level);
            }
        }
        packets.push({ version, typeID, typeLength, length, level });

    }
    return curPos;
}

function countVersions(arr) {
    let sum = 0;
    arr.forEach(e => {
        sum += e.version;
    });
    return sum;
}

const sampleStrArr = hexToBin(input);
const packets = [];

parsePacket(sampleStrArr, 0, 0);

const result = countVersions(packets);
saveOutput(result.toString());