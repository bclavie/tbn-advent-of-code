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
	ranges := parse(input)
	invalidSumPart1 := 0
	invalidSumPart2 := 0
	for _, r := range ranges {
		for i := r.Lower; i <= r.Upper; i++ {
			if isInvalidID(i, 2) {
				invalidSumPart1 += i
			}

			digits := getDigits(i)
			digitDivisors := getNonOneIntDivisors(digits)
			for _, d := range digitDivisors {
				if isInvalidID(i, d) {
					invalidSumPart2 += i
					break
				}
			}

		}
	}

	fmt.Printf("part1: %d\n", invalidSumPart1)
	fmt.Printf("part2: %d\n", invalidSumPart2)
}

func getDigits(orig int) int {
	digits := 0
	curr := orig
	for curr > 0 {
		digits += 1
		curr = curr / 10
	}
	return digits
}

func getNonOneIntDivisors(n int) []int {
	divs := []int{}
	for i := 2; i <= n; i++ {
		if n%i == 0 {
			divs = append(divs, i)
		}
	}

	return divs
}

func isInvalidID(orig int, splits int) bool {
	digits := getDigits(orig)

	baseDigits := digits / splits
	if baseDigits == 0 {
		return false
	}
	baseNum := orig % (intPow(10, baseDigits))
	finalNum := 0
	for i := range splits {
		finalNum += baseNum * (intPow(10, baseDigits*i))
	}
	return finalNum == orig
}

func intPow(base, exp int) int {
	if exp == 0 {
		return 1
	}
	if exp == 1 {
		return base
	}
	res := base
	for i := 2; i <= exp; i++ {
		res = res * base
	}
	return res
}

type Range struct {
	Lower int
	Upper int
}

func parse(input string) []Range {
	strRanges := strings.Split(input, ",")
	ranges := make([]Range, len(strRanges))
	for i, s := range strRanges {
		r := strings.SplitN(s, "-", 2)
		lower, err := strconv.Atoi(r[0])
		if err != nil {
			panic(fmt.Sprintf("invalid num %v", err))
		}
		upper, err := strconv.Atoi(r[1])
		if err != nil {
			panic(fmt.Sprintf("invalid num %v", err))
		}
		ranges[i] = Range{
			Lower: lower,
			Upper: upper,
		}
	}
	return ranges
}
