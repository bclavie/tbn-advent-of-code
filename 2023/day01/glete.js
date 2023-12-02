const fs = require("fs");
const path = require("path");

const readFileLines = (filename) =>
  fs.readFileSync(filename).toString("UTF8").split("\r\n");

const filePath = path.resolve(__dirname, "info.txt");
let arr = readFileLines(filePath);

var txtToNum = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};

const arr2 = arr.map((line) => {
  for (let i = 0; i < line.length; i++) {
    line = line
      .replace(/one/g, txtToNum.one)
      .replace(/two/g, txtToNum.two)
      .replace(/three/g, txtToNum.three)
      .replace(/four/g, txtToNum.four)
      .replace(/five/g, txtToNum.five)
      .replace(/six/g, txtToNum.six)
      .replace(/seven/g, txtToNum.seven)
      .replace(/eight/g, txtToNum.eight)
      .replace(/nine/g, txtToNum.nine)
      .replace(/tw1/g, "21")
      .replace(/eigh2/g, "82")
      .replace(/1ight/g, "18");
  }

  const firstAndLast = line.match(/\d/g);

  const firstChar = firstAndLast && firstAndLast[0] ? firstAndLast[0] : "";
  const lastChar =
    firstAndLast && firstAndLast.length > 1
      ? firstAndLast[firstAndLast.length - 1]
      : "";

  for (let i = 0; i < line.length; i++) {
    if (lastChar != "") {
      return parseInt(firstChar + lastChar);
    } else {
      return parseInt(firstChar + firstChar);
    }
  }
});

let result = 0;

for (let i = 0; i < arr2.length; i++) {
  result += parseInt(arr2[i]);
}
