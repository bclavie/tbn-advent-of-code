package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var file string

var OperatorsA = []string{"+", "*"}
var OperatorsB = []string{"+", "*", "||"}

type Equation struct {
	Result    int
	Numbers   []int
	Operators []string
}

func (e Equation) Solve() bool {
	if len(e.Numbers) == 1 {
		return e.Numbers[0] == e.Result
	}
	for _, op := range e.Operators {
		newEq := Equation{
			Result:    e.Result,
			Numbers:   []int{compute(e.Numbers[0], e.Numbers[1], op)},
			Operators: e.Operators,
		}
		if len(e.Numbers) > 2 {
			newEq.Numbers = append(newEq.Numbers, e.Numbers[2:]...)
		}
		if newEq.Solve() {
			return true
		}
	}
	return false
}

func compute(a, b int, op string) int {
	switch op {
	case "+":
		return a + b
	case "*":
		return a * b
	case "||":
		concat := strconv.Itoa(a) + strconv.Itoa(b)
		newNum, _ := strconv.Atoi(concat)
		return newNum
	}
	return 0
}

func calculateResultA(input []Equation) int {
	result := 0

	for _, el := range input {
		el.Operators = OperatorsA
		if el.Solve() {
			result += el.Result
		}
	}

	return result
}

func calculateResultB(input []Equation) int {
	result := 0

	for _, el := range input {
		el.Operators = OperatorsB
		if el.Solve() {
			result += el.Result
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

func getInput() []Equation {
	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	equations := make([]Equation, 0)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, ": ")
		result, _ := strconv.Atoi(parts[0])
		numbers := strings.Split(parts[1], " ")
		equation := Equation{
			Result:  result,
			Numbers: make([]int, len(numbers)),
		}
		for i, el := range numbers {
			number, _ := strconv.Atoi(el)
			equation.Numbers[i] = number
		}
		equations = append(equations, equation)
	}

	return equations
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
