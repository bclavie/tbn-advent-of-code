package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed inputs
var input string

type Bank []int

// We can recursively find the max by applying a algorithm of part1
// recursively on a smaller slice using the same method of breaking out.
func findMaxRecursive(ints []int, remainder int) int {
	if remainder == 0 {
		return 0
	}
	lowerIdx := 0
	maxDigit := 0

	for i := 0; i < len(ints)-(remainder-1); i++ {
		current := ints[i]
		if current > maxDigit {
			lowerIdx = i
			maxDigit = current
		}
	}
	result := powInt(10, remainder-1)*maxDigit + findMaxRecursive(ints[lowerIdx+1:], remainder-1)
	return result
}

func powInt(base, exp int) int {
	if exp == 0 {
		return 1
	}
	if exp == 1 {
		return base
	}
	result := base
	for i := 2; i <= exp; i++ {
		result *= base
	}
	return result
}

func (b Bank) findMaxPart2() int {
	return findMaxRecursive(b, 12)
}

func (b Bank) findMaxPart1() int {
	return findMaxRecursive(b, 2)
}

func parseInput(input string) []Bank {
	banks := []Bank{}
	for line := range strings.SplitSeq(input, "\n") {
		bank := Bank{}
		for _, s := range line {
			i, err := strconv.Atoi(string(s))
			if err != nil {
				panic("invalid num")
			}
			bank = append(bank, i)
		}
		banks = append(banks, bank)
	}

	return banks
}

func main() {
	banks := parseInput(input)

	part1, part2 := 0, 0
	for _, b := range banks {
		part1 += b.findMaxPart1()
		part2 += b.findMaxPart2()
	}

	fmt.Printf("part1: %d\n", part1)
	fmt.Printf("part2: %d\n", part2)
}
