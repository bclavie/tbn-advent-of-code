package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var file string

var hashRe = regexp.MustCompile(`#+`)

type Row struct {
	Pattern string
	Groups  []int
}

func (r *Row) Expand() {
	slice := []string{r.Pattern, r.Pattern, r.Pattern, r.Pattern, r.Pattern}
	r.Pattern = strings.Join(slice, "?")
	var newGroups []int
	for i := 0; i < 5; i++ {
		newGroups = append(newGroups, r.Groups...)
	}
	r.Groups = newGroups
}

func solveRow(pattern string, groups []int) int {
	hash := strings.Replace(pattern, "?", "#", 1)
	dot := strings.Replace(pattern, "?", ".", 1)
	if hash == dot {
		if correctSolution(pattern, groups) {
			return 1
		} else {
			return 0
		}
	}
	dotOptions := solveRow(dot, groups)
	hashOptions := solveRow(hash, groups)
	return dotOptions + hashOptions
}

func correctSolution(pattern string, groups []int) bool {
	dotSplit := filterEmpty(strings.Split(pattern, "."))
	if len(dotSplit) == len(groups) {
		for i := 0; i < len(groups); i++ {
			if len(dotSplit[i]) != groups[i] {
				return false
			}
		}
		return true
	}
	return false
}

func filterEmpty(s []string) (filtered []string) {
	for _, el := range s {
		if el != "" {
			filtered = append(filtered, el)
		}
	}
	return
}

func calculateResultA(input []string) int {
	result := 0
	rows := ParseRows(input)
	for _, row := range rows {
		result += solveRow(row.Pattern, row.Groups)
	}
	return result
}

func calculateResultB(input []string) int {
	result := 0
	rows := ParseRows(input)
	for _, row := range rows {
		fmt.Println("Original ", row)
		row.Expand()
		fmt.Println("Solving for ", row)
		// result += solveRow(row.Pattern, row.Groups)
	}
	return result
}

func ParseRows(input []string) []Row {
	rows := make([]Row, len(input))
	for i, line := range input {
		split := strings.Split(line, " ")
		sGroups := strings.Split(split[1], ",")
		groups := make([]int, len(sGroups))
		for j, el := range sGroups {
			groups[j], _ = strconv.Atoi(el)
		}
		rows[i] = Row{Pattern: split[0], Groups: groups}
	}
	return rows
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
