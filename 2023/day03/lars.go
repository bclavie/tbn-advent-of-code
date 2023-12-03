package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var file string

var numRe = regexp.MustCompile(`\d+`)
var symRe = regexp.MustCompile(`[^\.|\d]`)
var gearRe = regexp.MustCompile(`\*`)

type Number struct {
	value int
	x0    int
	x1    int
	y     int
}

func (n Number) HasAdjacentSymbol(symbols []Symbol) bool {
	for _, symbol := range symbols {
		if n.y-1 <= symbol.y && symbol.y <= n.y+1 {
			if n.x0-1 <= symbol.x && symbol.x <= n.x1+1 {
				return true
			}
		}
	}
	return false
}

type Symbol struct {
	x int
	y int
}

func (s Symbol) GearRatio(numbers []Number) int {
	adjacents := []int{}
	for _, num := range numbers {
		if num.y-1 <= s.y && s.y <= num.y+1 {
			if num.x0-1 <= s.x && s.x <= num.x1+1 {
				adjacents = append(adjacents, num.value)
			}
		}
	}
	if len(adjacents) == 2 {
		return adjacents[0] * adjacents[1]
	}
	return 0
}

func calculateResultA(input []string) int {
	result := 0
	numbers, symbols := parseInput(input, symRe)

	for _, num := range numbers {
		if num.HasAdjacentSymbol(symbols) {
			result += num.value
		}
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0
	numbers, gears := parseInput(input, gearRe)

	for _, gear := range gears {
		result += gear.GearRatio(numbers)
	}

	return result
}

func parseInput(input []string, symbolRe *regexp.Regexp) ([]Number, []Symbol) {
	numbers := []Number{}
	symbols := []Symbol{}
	for y, line := range input {
		numberMatches := numRe.FindAllStringIndex(line, len(line))
		symbolMatches := symbolRe.FindAllStringIndex(line, len(line))
		for _, coord := range symbolMatches {
			symbols = append(symbols, Symbol{coord[0], y})
		}
		for _, coord := range numberMatches {
			value, _ := strconv.Atoi(line[coord[0]:coord[1]])
			numbers = append(numbers, Number{value, coord[0], coord[1] - 1, y})
		}
	}
	return numbers, symbols
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
