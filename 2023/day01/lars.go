package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var file string

var intToStrings = map[int][]string{
	1: {"1", "one"},
	2: {"2", "two"},
	3: {"3", "three"},
	4: {"4", "four"},
	5: {"5", "five"},
	6: {"6", "six"},
	7: {"7", "seven"},
	8: {"8", "eight"},
	9: {"9", "nine"},
}

func calculateResultA(input []string) int {
	result := 0
	for _, line := range input {
		result += GetLineValue(line, false)
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0
	for _, line := range input {
		result += GetLineValue(line, true)
	}
	return result

}

func GetLineValue(line string, withLetters bool) int {
	lowestIndex := len(line)
	var firstDigit, lastDigit string
	highestIndex := -1
	for i := 1; i <= 9; i++ {
		for _, el := range intToStrings[i] {
			if !withLetters && len(el) > 1 {
				break
			}
			firstIndex := strings.Index(line, el)
			if firstIndex > -1 && firstIndex < lowestIndex {
				firstDigit = strconv.Itoa(i)
				lowestIndex = firstIndex
			}
			lastIndex := strings.LastIndex(line, el)
			if lastIndex > -1 && lastIndex > highestIndex {
				lastDigit = strconv.Itoa(i)
				highestIndex = lastIndex
			}
		}
	}
	value, _ := strconv.Atoi(firstDigit + lastDigit)
	return value
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() []string {
	input := []string{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		input = append(input, line)
	}

	return input
}

func main() {
	argsWithProg := os.Args

	var part string
	if len(argsWithProg) < 2 {
		part = "A"
	} else {
		part = argsWithProg[1]
	}

	fmt.Println(getResult(part))
}
