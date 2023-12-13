package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strings"
)

var file string

func DebugPrint(left, right []string) {
	maxLength := maxLength(left, right)
	for i := 0; i < maxLength; i++ {
		if i < len(left) {
			fmt.Printf("%s ", left[i])
		} else {
			fmt.Printf("%s ", strings.Repeat(" ", len(left[0])))
		}
		if i < len(right) {
			fmt.Printf("%s\n", right[i])
		} else {
			fmt.Printf("%s\n", strings.Repeat(" ", len(right[0])))
		}
	}
}

type Pattern struct {
	Rows       []string
	Columns    []string
	vertical   int
	horizontal int
}

func (p Pattern) Print(both bool) {
	length := len(p.Rows[0])
	if p.vertical > 0 {
		length++
	}
	for i, row := range p.Rows {
		if i > 0 && i == p.horizontal {
			fmt.Println(strings.Repeat("-", length))
		}
		if p.vertical > 0 {
			fmt.Printf("%s|%s\n", string(row[:p.vertical]), string(row[p.vertical:]))
		} else {
			fmt.Println(row)
		}
	}
	fmt.Printf("==> horizontal: %d , vertical: %d\n\n", p.horizontal, p.vertical)
}

func (p *Pattern) generateColumns() {
	p.Columns = make([]string, len(p.Rows[0]))
	for _, row := range p.Rows {
		for i, el := range row {
			p.Columns[i] = string(el) + p.Columns[i]
		}
	}
}

func (p *Pattern) FindMirrorLines(allowedErr int) {
	p.vertical = FindMirrorLine(p.Columns, allowedErr)
	p.horizontal = FindMirrorLine(p.Rows, allowedErr)
}

func calculateResult(input []string, allowedErr int) int {
	result := 0
	patterns := ParsePatterns(input)
	for _, pattern := range patterns {
		pattern.FindMirrorLines(allowedErr)
		result += (pattern.vertical + 100*pattern.horizontal)
		pattern.Print(false)
	}
	return result
}

func FindMirrorLine(pattern []string, allowedErr int) int {
	for i := 1; i < len(pattern); i++ {
		if CheckMirror(pattern[:i], pattern[i:], allowedErr) {
			return i
		}
	}
	return 0
}

func CheckMirror(left, right []string, allowedErr int) bool {
	// reverse for comparison
	slices.Reverse(left)
	errors := 0
	minLength := minLength(left, right)
	for i := 0; i < minLength; i++ {
		for j := range left[i] {
			if left[i][j] != right[i][j] {
				errors++
			}
		}
	}
	// reverse back
	slices.Reverse(left)
	return allowedErr == errors
}

func ParsePatterns(input []string) []Pattern {
	patterns := []Pattern{}
	rows := []string{}
	for _, line := range input {
		if len(line) == 0 {
			patterns = append(patterns, Pattern{Rows: rows})
			rows = []string{}
		} else {
			rows = append(rows, line)
		}
	}
	patterns = append(patterns, Pattern{Rows: rows})
	for i := range patterns {
		patterns[i].generateColumns()
	}
	return patterns
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResult(input, 0)
	}

	return calculateResult(input, 1)
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

func maxLength(a, b []string) int {
	return int(math.Max(float64(len(a)), float64(len(b))))
}

func minLength(a, b []string) int {
	return int(math.Min(float64(len(a)), float64(len(b))))
}
