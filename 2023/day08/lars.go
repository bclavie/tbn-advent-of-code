package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"slices"
)

var file string

type Node struct {
	left  string
	right string
}

// greatest common divisor (GCD) via Euclidean algorithm
func GCD(a, b int64) int64 {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// find Least Common Multiple (LCM) via GCD
func LCM(a, b int64, integers ...int64) int64 {
	result := a * b / GCD(a, b)

	for i := 0; i < len(integers); i++ {
		result = LCM(result, integers[i])
	}

	return result
}

func GoalAfter(node string, instructions string, nodes map[string]Node) int64 {
	steps := int64(1)
	current := node
	for i := 0; ; steps++ {
		move := string(instructions[i])
		if move == "L" {
			current = nodes[current].left
		} else {
			current = nodes[current].right
		}
		if endsOn(current, "Z") {
			return steps
		}
		if i < len(instructions)-1 {
			i++
		} else {
			i = 0
		}
	}
}

func calculateResultA(input []string) int64 {
	instructions, nodes, _ := ParseNodes(input)
	return GoalAfter("AAA", instructions, nodes)
}

func calculateResultB(input []string) int64 {
	result := int64(1)
	instructions, nodes, walkNodes := ParseNodes(input)
	repetitions := []int64{}
	for _, node := range walkNodes {
		goalDistance := GoalAfter(node, instructions, nodes)
		repetitions = append(repetitions, goalDistance)
	}
	slices.Sort(repetitions)
	fmt.Println(repetitions)
	result = LCM(repetitions[0], repetitions[1], repetitions[2:]...)
	return result
}

func endsOn(s, target string) bool {
	return string(s[len(s)-1]) == target
}

func ParseNodes(input []string) (string, map[string]Node, []string) {
	nodes := make(map[string]Node)
	nodeRe := regexp.MustCompile(`(...) = \((...), (...)\)`)
	instructions := input[0]
	startNodes := []string{}
	for _, line := range input {
		matches := nodeRe.FindStringSubmatch(line)
		if matches != nil {
			nodes[matches[1]] = Node{matches[2], matches[3]}
			if endsOn(matches[1], "A") {
				startNodes = append(startNodes, matches[1])
			}
		}
	}
	return instructions, nodes, startNodes
}

func getResult(part string) int64 {
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
