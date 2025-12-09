package main

import (
	_ "embed"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

//go:embed inputs
var input string

func main() {
	coordinates := parseInput(input)
	distancePairs := []DistancePair{}
	for i := range coordinates {
		for j := range coordinates {
			if i >= j {
				continue
			}
			distance := Distance(coordinates[i], coordinates[j])
			distancePairs = append(distancePairs, DistancePair{
				LowerIdx: i,
				UpperIdx: j,
				Distance: distance,
			})
		}
	}

	slices.SortFunc(distancePairs, func(a, b DistancePair) int {
		if a.Distance < b.Distance {
			return -1
		}
		if a.Distance > b.Distance {
			return 1
		}
		return 0
	})

	targetLength := len(coordinates)
	coordinateToGroupingIndex := map[int]int{}
	groupsToCoordinates := map[int][]int{}
	for i := range coordinates {
		groupsToCoordinates[i] = []int{i}
		coordinateToGroupingIndex[i] = i
	}

	for i, pairs := range distancePairs {
		if i == 1000 {
			circuitSizes := []int{}
			for _, group := range groupsToCoordinates {
				circuitSizes = append(circuitSizes, len(group))
			}
			slices.SortFunc(circuitSizes, func(a, b int) int {
				if a > b {
					return -1
				}
				if a < b {
					return 1
				}
				return 0
			})

			part1 := 1
			for i := range 3 {
				part1 *= circuitSizes[i]
			}
			fmt.Printf("Part1 %d\n", part1)
		}

		upperCoordGroupIdx := coordinateToGroupingIndex[pairs.UpperIdx]
		lowerCoordGroupIdx := coordinateToGroupingIndex[pairs.LowerIdx]
		if upperCoordGroupIdx == lowerCoordGroupIdx {
			continue
		}
		groupIndexToUse := lowerCoordGroupIdx
		groupIdxToMerge := upperCoordGroupIdx
		if upperCoordGroupIdx < groupIndexToUse {
			groupIndexToUse = upperCoordGroupIdx
			groupIdxToMerge = lowerCoordGroupIdx
		}

		existingGroupToMerge := groupsToCoordinates[groupIdxToMerge]
		groupsToCoordinates[groupIndexToUse] = append(groupsToCoordinates[groupIndexToUse], existingGroupToMerge...)
		groupsToCoordinates[groupIdxToMerge] = nil
		newCoordsInGroup := groupsToCoordinates[groupIndexToUse]
		for _, coordIdx := range newCoordsInGroup {
			coordinateToGroupingIndex[coordIdx] = groupIndexToUse
		}

		if targetLength == len(newCoordsInGroup) {
			upperCoord := coordinates[pairs.UpperIdx]
			lowerCoord := coordinates[pairs.LowerIdx]
			part2 := int(upperCoord.X) * int(lowerCoord.X)
			fmt.Printf("Part2 %d\n", part2)
			break
		}
	}

}

// Always index by lower index first.
type DistancePair struct {
	LowerIdx int
	UpperIdx int
	Distance float64
}

func parseInput(input string) []Coord {
	coordinates := []Coord{}
	for line := range strings.SplitSeq(input, "\n") {
		coordString := strings.Split(line, ",")
		x, _ := strconv.ParseFloat(coordString[0], 64)
		y, _ := strconv.ParseFloat(coordString[1], 64)
		z, _ := strconv.ParseFloat(coordString[2], 64)
		coord := Coord{
			X: x,
			Y: y,
			Z: z,
		}
		coordinates = append(coordinates, coord)
	}
	return coordinates
}

type Coord struct {
	X float64
	Y float64
	Z float64
}

func Distance(a, b Coord) float64 {
	return math.Pow(math.Pow(a.X-b.X, 2)+math.Pow(a.Y-b.Y, 2)+math.Pow(a.Z-b.Z, 2), 0.5)
}
