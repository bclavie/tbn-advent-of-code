const fs = require("fs");
const path = require("path");

const readFileLines = (filename) =>
  fs.readFileSync(filename).toString("UTF8").split("\r\n");

const filePath = path.resolve(__dirname, "info2.txt");
let arr = readFileLines(filePath);
let result = 0;

let arr2 = arr.filter((line) => {
  let numbers = line.match(/(\d+)/g);
  let colours = line.match(/([a-zA-Z])+/g);
  for (let i = 0; i < numbers.length; i++) {
    if (
      (colours[i] == "red" && numbers[i] > 12) ||
      (colours[i] == "green" && numbers[i] > 13) ||
      (colours[i] == "blue" && numbers[i] > 14)
    ) {
      return false;
    }
  }
  return true;
});

arr2.map((line) => {
  let numbers = line.match(/(\d+)/);
  result += parseInt(numbers);
});
console.log(result);
