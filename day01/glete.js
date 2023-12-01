const fs = require("fs");
const path = require("path");

const readFileLines = (filename) =>
  fs.readFileSync(filename).toString("UTF8").split("\r\n");

const filePath = path.resolve(__dirname, "info.txt");
let arr = readFileLines(filePath);
var result = 0;

const arr2 = arr.map((line) => {
  const firstAndLast = line.match(/\d/g);

  const firstChar = firstAndLast && firstAndLast[0] ? firstAndLast[0] : "";
  const lastChar =
    firstAndLast && firstAndLast.length > 1
      ? firstAndLast[firstAndLast.length - 1]
      : "";

  for (let i = 0; i <= line.length - 1; i++) {
    if (lastChar != "") {
      return parseInt(firstChar + lastChar);
    } else {
      return parseInt(firstChar + firstChar);
    }
  }
});

for (let i = 0; i <= arr2.length - 1; i++) {
  result += arr2[i];
}

//console.log(arr2);
//console.log(result);
