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

const input = readInput();
const passports = input.split(/\r/).join("").split("\n\n").map(passport => passport.split(/ +|\n/).join(" "));
let validPassports = 0;
for(const passport of passports) {
	const fields = passport.split(" ").map(fieldVal => fieldVal.split(":")[0]);
	let valid = true;
	for(const requiredField of requiredFields) {
		if(!fields.includes(requiredField)) {
			valid = false;
			break;
		};
	};
	if(valid) validPassports += 1;
};
saveOutput(validPassports.toString());