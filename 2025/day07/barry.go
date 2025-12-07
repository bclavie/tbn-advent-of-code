package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed inputs
var input string

func main() {
	grid := parseInput(input)
	part1 := Simulate(grid)

	fmt.Printf("Part1: %d\n", part1)

	part2 := SimulateQuantum(grid)
	fmt.Printf("Part2: %d\n", part2)
}

const Beam = "|"
const Start = "S"
const Splitter = "^"

func parseInput(input string) [][]string {
	grid := [][]string{}
	for line := range strings.SplitSeq(input, "\n") {
		lineOfGrid := []string{}
		for _, s := range line {
			lineOfGrid = append(lineOfGrid, string(s))
		}
		grid = append(grid, lineOfGrid)
	}
	return grid
}

func Simulate(grid [][]string) int {
	numberOfSplits := 0
	for i, line := range grid {
		if i == len(grid)-1 {
			break
		}
		beamIndexesInNextLine := map[int]struct{}{}
		for j, s := range line {
			// Just looking at the input, we don't need to worry about out of bounds.
			if s == Start || s == Beam {
				if grid[i+1][j] == Splitter {
					beamIndexesInNextLine[j-1] = struct{}{}
					beamIndexesInNextLine[j+1] = struct{}{}
					numberOfSplits += 1
				} else {
					beamIndexesInNextLine[j] = struct{}{}
				}
			}
		}
		for idx := range beamIndexesInNextLine {
			grid[i+1][idx] = Beam
		}
	}
	return numberOfSplits
}

func SimulateQuantum(grid [][]string) int {
	// Simulate but keep track of the number of paths each
	// grid point has been hit by a path.
	// Each path from the start should be unique due to the layout
	// of the grid.
	pathsForCoord := [][]int{}
	for _, line := range grid {
		zeroes := make([]int, len(line))
		pathsForCoord = append(pathsForCoord, zeroes)
	}

	startLine := grid[0]
	for i, s := range startLine {
		if s == Start {
			pathsForCoord[0][i] = 1
		}
	}

	uniquePaths := 0
	for i := range pathsForCoord {
		for j := range pathsForCoord[i] {
			// We're at the end - so the unique paths is the total number of paths
			// in the final row.
			if i == len(pathsForCoord)-1 {
				uniquePaths += pathsForCoord[i][j]
			} else {
				if pathsForCoord[i][j] > 0 {
					// Simulate as before
					if grid[i+1][j] == Splitter {
						pathsForCoord[i+1][j+1] += pathsForCoord[i][j]
						pathsForCoord[i+1][j-1] += pathsForCoord[i][j]
					} else {
						pathsForCoord[i+1][j] += pathsForCoord[i][j]
					}
				}
			}

		}
	}

	return uniquePaths
}

type Coord struct {
	X int
	Y int
}
