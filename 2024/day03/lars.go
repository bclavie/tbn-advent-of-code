package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var file string
var mulRegex = regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)
var mulandDosRegex = regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)`)

func calculateResultA(input []string) int {
	result := 0
	for _, line := range input {
		matches := mulRegex.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			a, _ := strconv.Atoi(match[1])
			b, _ := strconv.Atoi(match[2])
			result += a * b
		}
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0
	enabled := true
	for _, line := range input {
		matches := mulandDosRegex.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			if match[0] == "do()" {
				enabled = true
			}
			if match[0] == "don't()" {
				enabled = false
			}
			if enabled && len(match) == 3 {
				a, _ := strconv.Atoi(match[1])
				b, _ := strconv.Atoi(match[2])
				result += a * b
			}
		}
	}

	return result
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
