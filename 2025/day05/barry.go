package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

//go:embed inputs
var input string

func main() {
	ranges, ids := parseInput(input)

	part1 := 0
	for _, id := range ids {
		for _, r := range ranges {
			if r.Contains(id) {
				part1++
				break
			}
		}
	}

	slices.SortFunc(ranges, func(a, b Range) int {
		if a.Lower < b.Lower {
			return -1
		}
		if a.Lower > b.Lower {
			return 1
		}
		return 0
	})

	mergedRange := []Range{}
	for _, r := range ranges {
		mergedRange = NewMergedRanges(mergedRange, r)
	}
	part2 := 0
	for _, r := range mergedRange {
		part2 += r.Len()
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

// Assumes that existing is sorted by lower bound.
func NewMergedRanges(existing []Range, new Range) []Range {
	// Find overlap of lower and upper.
	// If lower overlaps, merge with lower.
	// If upper overlaps, merge with upper.
	// If multiple overlaps, need to merge all ranges.
	lowerIndex := -1
	upperIndex := -1
	for i, e := range existing {
		if e.Contains(new.Lower) {
			lowerIndex = i
		}
		if e.Contains(new.Upper) {
			upperIndex = i
		}
	}

	if lowerIndex == -1 && upperIndex == -1 {
		for i := range existing {
			if existing[i].Lower > new.Upper {
				newRange := append(existing[:i], append([]Range{new}, existing[i:]...)...)
				return newRange
			}
		}
		return append(existing, new)
	}
	// Merge the whole range between lower and upper.
	if lowerIndex != -1 && upperIndex != -1 {
		merged := Range{
			Lower: existing[lowerIndex].Lower,
			Upper: existing[upperIndex].Upper,
		}

		newRange := append(existing[:lowerIndex], append([]Range{merged}, existing[upperIndex+1:]...)...)
		return newRange
	}

	// Otherwise merge with either lower or upper
	if lowerIndex == -1 {
		merged := Range{
			Lower: new.Lower,
			Upper: existing[upperIndex].Upper,
		}
		newRange := append(existing[:upperIndex], append([]Range{merged}, existing[upperIndex+1:]...)...)
		return newRange
	}

	if upperIndex == -1 {
		merged := Range{
			Lower: existing[lowerIndex].Lower,
			Upper: new.Upper,
		}
		newRange := append(existing[:lowerIndex], append([]Range{merged}, existing[lowerIndex+1:]...)...)
		return newRange
	}
	panic("unreachable")
}

type Range struct {
	Upper int
	Lower int
}

func (r Range) Contains(id int) bool {
	return id >= r.Lower && id <= r.Upper
}

func (r Range) Len() int {
	return r.Upper - r.Lower + 1
}

func parseInput(input string) ([]Range, []int) {
	spl := strings.Split(input, "\n\n")
	rangeString := spl[0]
	ranges := []Range{}
	for _, r := range strings.Split(rangeString, "\n") {
		bounds := strings.Split(r, "-")
		upper, _ := strconv.Atoi(bounds[1])
		lower, _ := strconv.Atoi(bounds[0])
		ranges = append(ranges, Range{Upper: upper, Lower: lower})
	}

	idStrings := spl[1]
	ids := make([]int, 0, len(idStrings))
	for _, idstr := range strings.Split(idStrings, "\n") {
		id, _ := strconv.Atoi(idstr)
		ids = append(ids, id)
	}

	return ranges, ids
}
