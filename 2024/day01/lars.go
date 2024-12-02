package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

var file string

func calculateResultA(left []int, right []int) int {
	slices.Sort(left)
	slices.Sort(right)
	result := 0
	for i := range left {
		result += int(math.Abs(float64(left[i]) - float64(right[i])))
	}

	return result
}

func calculateResultB(left []int, right []int) int {
	result := 0
	rightMap := make(map[int]int)
	for _, el := range left {
		rightMap[el] = 0
	}
	for _, el := range right {
		if _, ok := rightMap[el]; !ok {
			rightMap[el] = 1
		} else {
			rightMap[el] += 1
		}
	}

	for _, el := range left {
		result += el * rightMap[el]
	}

	return result
}

func getResult(part string) int {
	left, right := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(left, right)
	}

	return calculateResultB(left, right)
}

func getInput() ([]int, []int) {
	left := []int{}
	right := []int{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "   ")
		first, _ := strconv.Atoi(parts[0])
		second, _ := strconv.Atoi(parts[1])
		left = append(left, first)
		right = append(right, second)
	}

	return left, right
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
