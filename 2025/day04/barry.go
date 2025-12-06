package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed inputs
var input string

func main() {
	problem := parseInput(input)

	_, part1 := problem.getAndRemoveAccessibleLocations()

	part2 := 0
	for {
		newGrid, removed := problem.getAndRemoveAccessibleLocations()
		part2 += removed
		if removed == 0 {
			break
		}
		problem = newGrid
	}

	fmt.Printf("Part1: %d\n", part1)
	fmt.Printf("Part2: %d\n", part2)
}

type Problem struct {
	grid [][]string
}

func parseInput(s string) Problem {
	grid := [][]string{}

	for line := range strings.SplitSeq(s, "\n") {
		row := []string{}
		for _, c := range line {
			row = append(row, string(c))
		}

		grid = append(grid, row)
	}
	return Problem{grid}
}

const paper = "@"
const empty = "."

func (p Problem) getAndRemoveAccessibleLocations() (Problem, int) {
	total := 0
	newGrid := make([][]string, len(p.grid))
	for i := range newGrid {
		newGrid[i] = make([]string, len(p.grid[0]))
		for j := range newGrid[i] {
			newGrid[i][j] = empty
		}
	}
	for i := range p.grid {
		for j := range p.grid[i] {
			if p.grid[i][j] == paper {
				adjacentPaper := 0

				// Top Left
				if i > 0 {
					if j > 0 {
						if p.grid[i-1][j-1] == paper {
							adjacentPaper += 1
						}
					}

					// Top
					if p.grid[i-1][j] == paper {
						adjacentPaper += 1
					}
					// Top Right
					if j < len(p.grid[i])-1 {
						if p.grid[i-1][j+1] == paper {
							adjacentPaper += 1
						}
					}
				}

				// Left
				if j > 0 {
					if p.grid[i][j-1] == paper {
						adjacentPaper += 1
					}
				}

				// Right
				if j < len(p.grid[i])-1 {
					if p.grid[i][j+1] == paper {
						adjacentPaper += 1
					}
				}

				// Bottom Left
				if i < len(p.grid)-1 {
					if j > 0 {
						if p.grid[i+1][j-1] == paper {
							adjacentPaper += 1
						}
					}

					if p.grid[i+1][j] == paper {
						adjacentPaper += 1
					}

					if j < len(p.grid[i])-1 {
						if p.grid[i+1][j+1] == paper {
							adjacentPaper += 1
						}
					}
				}

				if adjacentPaper < 4 {
					total += 1
				} else {
					newGrid[i][j] = paper
				}
			}
		}
	}
	return Problem{grid: newGrid}, total
}
