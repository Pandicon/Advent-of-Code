const fs = require('fs');

function readInput(path = "input.txt", encoding = "utf8", flag = "r") {
	return fs.readFileSync(path, { encoding, flag });
};

function saveOutput(data, path = "output.txt", encoding = "utf8", flag = "w") {
	fs.writeFileSync(path, data, { encoding, flag });
};

const requiredFields = [
	"byr",
	"iyr",
	"eyr",
	"hgt",
	"hcl",
	"ecl",
	"pid"
];

const boundaries = {
	"byr": [1920, 2002],
	"iyr": [2010, 2020],
	"eyr": [2020, 2030],
	"hgt": {
		"cm": [150, 193],
		"in": [59, 76]
	}
};

const eyeColours = [
	"amb",
	"blu",
	"brn",
	"gry",
	"grn",
	"hzl",
	"oth"
];

const input = readInput();
const passports = input.split(/\r/).join("").split("\n\n").map(passport => passport.split(/ +|\n/).join(" "));
let validPassports = 0;
for(const passport of passports) {
	const fields = passport.split(" ").map(fieldVal => fieldVal.split(":")[0]);
	const values = passport.split(" ").map(fieldVal => fieldVal.split(":")[1]);
	let valid = true;
	for(const requiredField of requiredFields) {
		if(!fields.includes(requiredField)) {
			valid = false;
			break;
		};
	};
	if(!valid) continue;
	for(const i in fields) {
		let field = fields[i];
		let value = values[i];
		switch (field) {
			case "byr":
			case "iyr":
			case "eyr":
				valid = validateNumber(field, value);
				break;
			case "ecl":
				valid = validateEcl(value);
				break;
			case "hcl":
				valid = validateHcl(value);
				break;
			case "hgt":
				valid = validateHgt(value);
				break;
			case "pid":
				valid = validatePid(value);
				break;
			default:
				break;
		};
		if(!valid) {
			break;
		};
	};
	if(valid) validPassports += 1;
};
saveOutput(validPassports.toString());

function validateNumber(type, num) {
	num = parseInt(num);
	if(isNaN(num)) return false;
	const bounds = boundaries[type];
	if(num < bounds[0] || num > bounds[1]) return false;
	return true;
};

function validateHgt(hgt) {
	if(!hgt.match(/^[0-9]{1,3}cm|^[0-9]{1,2}in/)) return false;
	const num = parseInt(hgt.slice(0, -2));
	const units = hgt.slice(hgt.length-2);
	const bounds = boundaries["hgt"][units];
	if(num < bounds[0] || num > bounds[1]) return false;
	return true;
};

function validateHcl(hcl) {
	return hcl.match(/^#[0-9a-f]{6}$/);
};

function validateEcl(ecl) {
	return eyeColours.includes(ecl);
};

function validatePid(pid) {
	return pid.match(/^[0-9]{9}$/);
};