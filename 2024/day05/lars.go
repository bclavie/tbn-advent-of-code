package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

var file string

type Rules map[int][]int

func calculateResultA(rules Rules, updates [][]int) int {
	result := 0

	for _, update := range updates {
		if confirmRules(rules, update) {
			result += update[len(update)/2]
		}
	}

	return result
}

func calculateResultB(rules Rules, updates [][]int) int {
	result := 0

	for _, update := range updates {
		if !confirmRules(rules, update) {
			newOrder := orderCorrectly(rules, update)
			result += update[len(newOrder)/2]
		}
	}

	return result
}

func confirmRules(rules Rules, update []int) bool {
	if len(update) < 1 {
		return true
	}
	first, rest := update[0], update[1:]
	for _, el := range rest {
		if !slices.Contains(rules[first], el) {
			return false
		}
	}
	return confirmRules(rules, rest)
}

func orderCorrectly(rules Rules, update []int) []int {
	newOrder := make([]int, len(update))

	slices.SortFunc(update, func(a, b int) int {
		if slices.Contains(rules[a], b) {
			return -1
		}
		return 1
	})

	return newOrder
}

func getResult(part string) int {
	rules, updates := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(rules, updates)
	}

	return calculateResultB(rules, updates)
}

func getInput() (rules Rules, updates [][]int) {

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	rules = make(Rules)
	updates = make([][]int, 0)
	rulesDone := false
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			rulesDone = true
		} else if !rulesDone {
			parts := strings.Split(line, "|")
			front, _ := strconv.Atoi(parts[0])
			back, _ := strconv.Atoi(parts[1])
			rules[front] = append(rules[front], back)
		} else if rulesDone {
			parts := strings.Split(line, ",")
			numbers := []int{}
			for _, el := range parts {
				number, _ := strconv.Atoi(el)
				numbers = append(numbers, number)
			}
			updates = append(updates, numbers)
		}
	}

	return rules, updates
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
