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
var debug = false

var hashRe = regexp.MustCompile(`#+`)

func DebugPrintln(a ...any) {
	if debug {
		fmt.Println(a...)
	}
}

func DebugPrintf(s string, a ...any) {
	if debug {
		fmt.Printf(s, a...)
	}
}

func MemoryKey(pattern string, groups []int, inGroup bool, needDot bool) string {
	sGroups := []string{}
	for _, el := range groups {
		sGroups = append(sGroups, strconv.Itoa(el))
	}
	return fmt.Sprintf("%s-%s-%v-%v", pattern, strings.Join(sGroups, ","), inGroup, needDot)
}

type Row struct {
	Pattern string
	Groups  []int
}

func (r Row) Expand(factor int) Row {
	newPattern := []string{}
	var newGroups []int
	for i := 0; i < factor; i++ {
		newPattern = append(newPattern, r.Pattern)
		newGroups = append(newGroups, r.Groups...)
	}
	return Row{
		Pattern: strings.Join(newPattern, "?"),
		Groups:  newGroups,
	}
}

func solveRow(pattern string, groups []int, inGroup bool, needDot bool, memory map[string]int) int {
	DebugPrintln("Solve for ", pattern, groups, inGroup, needDot)
	// end recursion when no pattern left
	if len(pattern) == 0 {
		if len(groups) == 0 {
			return 1
		}
		return 0
	}
	// Use memory to see if we know the solution
	memoryKey := MemoryKey(pattern, groups, inGroup, needDot)
	known, ok := memory[memoryKey]
	if ok {
		fmt.Println("memory was useful for", pattern, groups, inGroup, needDot)
		return known
	}
	// Work on next character
	result := 0
	next := pattern[0]
	switch next {
	case '?':
		// Replace questionmark with both options and try again
		dot := strings.Replace(pattern, "?", ".", 1)
		hash := strings.Replace(pattern, "?", "#", 1)
		dotOptions := solveRow(dot, groups, inGroup, needDot, memory)
		hashOptions := solveRow(hash, groups, inGroup, needDot, memory)
		result = dotOptions + hashOptions
	case '.':
		if inGroup {
			result = 0
		} else {
			result = solveRow(strings.Clone(pattern[1:]), groups, false, false, memory)
		}
	case '#':
		if needDot || len(groups) == 0 || groups[0] == 0 {
			result = 0
		} else {
			newGroups := DeepClone(groups)
			if newGroups[0] > 1 {
				newGroups[0]--
				result = solveRow(strings.Clone(pattern[1:]), newGroups, true, false, memory)
			} else {
				result = solveRow(strings.Clone(pattern[1:]), newGroups[1:], false, true, memory)
			}
		}
	}
	memory[memoryKey] = result
	DebugPrintln("Add result for ", memoryKey, "==>", result)
	return result
}

func DeepClone(a []int) []int {
	newA := make([]int, len(a))
	for i, el := range a {
		newA[i] = el
	}
	return newA
}

func calculateResultA(input []string) int {
	result := 0
	rows := ParseRows(input)
	for _, row := range rows {
		rowResult := solveRow(row.Pattern, row.Groups, false, false, make(map[string]int))
		fmt.Println(row, " ==> ", rowResult)
		result += rowResult
	}
	return result
}

func calculateResultB(input []string) int {
	result := 0
	rows := ParseRows(input)
	for _, row := range rows {
		expanded := row.Expand(5)
		rowResult := solveRow(expanded.Pattern, expanded.Groups, false, false, make(map[string]int))
		fmt.Println(expanded, " ==> ", rowResult)
		result += rowResult
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
