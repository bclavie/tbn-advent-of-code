package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

var file string

func calculateResultA(input [][]int) int {
	result := 0
	for _, report := range input {
		if isSafe(report, 0) {
			result += 1
		}
	}

	return result
}

func calculateResultB(input [][]int) int {
	result := 0
	for _, report := range input {
		if isSafe(report, 1) {
			result += 1
		}
	}

	return result
}

func isSafe(report []int, retries int) bool {
	ascending := report[0] < report[1]
	for i := 0; i < len(report)-1; i++ {
		diff := int(math.Abs(float64(report[i]) - float64(report[i+1])))
		if diff < 1 || diff > 3 {
			if retries > 0 {
				return isSafe(sliceWithout(report, i-1), retries-1) ||
					isSafe(sliceWithout(report, i), retries-1) ||
					isSafe(sliceWithout(report, i+1), retries-1)
			}
			return false
		}
		if (ascending && report[i] > report[i+1]) || (!ascending && report[i] < report[i+1]) {
			if retries > 0 {
				return isSafe(sliceWithout(report, i-1), retries-1) ||
					isSafe(sliceWithout(report, i), retries-1) ||
					isSafe(sliceWithout(report, i+1), retries-1)
			}
			return false
		}
	}
	return true
}

func sliceWithout(in []int, remove int) []int {
	if remove < 0 {
		return in
	}
	return append(append([]int{}, in[:remove]...), in[remove+1:]...)
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() [][]int {
	input := [][]int{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")
		report := make([]int, len(parts))
		for i := range parts {
			report[i], _ = strconv.Atoi(parts[i])
		}
		input = append(input, report)
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
