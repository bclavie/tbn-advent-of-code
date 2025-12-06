package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed inputs
var input string

func main() {
	collections := parseInputPart1(input)
	part1 := 0
	for _, c := range collections {
		res := c.DoOperation()
		part1 += res
	}
	fmt.Printf("Part1: %d\n", part1)

	collections2 := parseInputPart2(input)
	part2 := 0
	for _, c := range collections2 {
		res := c.DoOperation()
		part2 += res
	}
	fmt.Printf("Part2: %d\n", part2)
}

type Operation string

const (
	Add      Operation = "+"
	Multiply Operation = "*"
)

type Collection struct {
	nums      []int
	operation Operation
}

func (c *Collection) DoOperation() int {
	res := c.nums[0]
	if c.operation == Add {
		for i := 1; i < len(c.nums); i++ {
			res += c.nums[i]
		}
	} else {
		for i := 1; i < len(c.nums); i++ {
			res *= c.nums[i]
		}
	}
	return res
}

func parseInputPart1(input string) []Collection {
	collections := []Collection{}
	lines := strings.Split(input, "\n")
	for i, line := range lines {
		for j, s := range strings.Fields(line) {
			if i == 0 {
				parsedInt, _ := strconv.Atoi(s)
				c := Collection{
					nums:      []int{parsedInt},
					operation: "",
				}
				collections = append(collections, c)
			} else {
				if i < len(lines)-1 {
					parsedInt, _ := strconv.Atoi(s)
					collections[j].nums = append(collections[j].nums, parsedInt)
				} else {
					collections[j].operation = Operation(s)
				}
			}
		}
	}

	return collections
}

// Arithmetic is the same but the parsing becomes trickier.
func parseInputPart2(input string) []Collection {
	// Use the last line as an anchor for each problem to get width.
	// Then go back to each line of numbers to parse per problem.
	lines := strings.Split(input, "\n")
	lastLine := lines[len(lines)-1]

	i := 0
	intermediates := []Intermediate{}
	for {
		width := 1
		operation := Operation(lastLine[i])
		i++
		for i < len(lastLine) && string(lastLine[i]) == " " {
			i++
			width++
		}

		intermediates = append(intermediates, Intermediate{
			Op:    operation,
			Width: width,
		})

		if i >= len(lastLine) {
			break
		}
	}

	// Now for each intermediate - go back and parse the numbers (units on the bottom).
	collections := []Collection{}
	for _, i := range intermediates {
		collections = append(collections, Collection{
			nums:      []int{},
			operation: i.Op,
		})
	}
	numberedLines := lines[:len(lines)-1]
	currentOffset := 0
	for k, intermediate := range intermediates {
		for i := intermediate.Width - 1; i >= 0; i-- {
			numberedStr := ""
			for lineNo := range numberedLines {
				stringToParse := numberedLines[lineNo][currentOffset+i]
				if string(stringToParse) == " " {
					continue
				}
				numberedStr += string(stringToParse)
			}
			if numberedStr != "" {
				parsedInt, _ := strconv.Atoi(numberedStr)
				collections[k].nums = append(collections[k].nums, parsedInt)
			}
		}

		currentOffset += intermediate.Width
	}

	return collections
}

type Intermediate struct {
	Op    Operation
	Width int
}
