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
	lines := strings.Split(input, "\n")
	nums := make([]int, len(lines))
	for i := range lines {
		nums[i] = parseLine(lines[i])
	}

	currentP1 := 50
	currentP2 := 50
	numZeros1 := 0
	numZeros2 := 0
	for _, n := range nums {
		currentP1 = (currentP1 + n) % 100
		if currentP1 == 0 {
			numZeros1 += 1
		}
		// Really naive way is to just break it down into individual adds.
		// if n < 0 {
		// 	for range -n {
		// 		currentP2 = (currentP2 - 1) % 100
		// 		if currentP2 == 0 {
		// 			numZeros2 += 1
		// 		}
		// 	}
		// } else {
		// 	for range n {
		// 		currentP2 = (currentP2 + 1) % 100
		// 		if currentP2 == 0 {
		// 			numZeros2 += 1
		// 		}
		// 	}
		// }
		// Otherwise we need to do some maths for edge cases each time it crosses zero.
		new := currentP2 + n
		num100s := abs(new / 100)
		// If the signs are different, then we crossed zero that wasn't taken into account in num 100s.
		if new == 0 || diffSign(currentP2, new) {
			numZeros2 += 1
		}
		numZeros2 += num100s
		currentP2 = new % 100
	}

	fmt.Printf("Part1: %d\n", numZeros1)
	fmt.Printf("Part2: %d\n", numZeros2)
}

func diffSign(a, b int) bool {
	return a > 0 && b < 0 || a < 0 && b > 0
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func parseLine(l string) int {
	num, err := strconv.Atoi(l[1:])
	if err != nil {
		panic("invalid number")
	}
	if strings.HasPrefix(l, "L") {
		num = -num
	}
	return num
}
