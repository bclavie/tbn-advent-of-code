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
	part1 := 0
	// Part1 - naive area comparison
	coords := parseInput(input)
	for i := range coords {
		for j := range i - 1 {
			area := coords[i].Area(coords[j])
			if area > part1 {
				part1 = area
			}
		}
	}
	fmt.Printf("Part1 %d\n", part1)

	// This doesn't work for all objects but
	// the input might be nice enough.
	linePath := map[Coord]struct{}{}
	// Draw the paths
	for i := range coords {
		for j := 0; j < i; j++ {
			if coords[i].X == coords[j].X {
				minY := min(coords[i].Y, coords[j].Y)
				maxY := max(coords[i].Y, coords[j].Y)
				for y := minY; y <= maxY; y++ {
					linePath[Coord{
						X: coords[i].X,
						Y: y,
					}] = struct{}{}
				}
			}
			if coords[i].Y == coords[j].Y {
				minX := min(coords[i].X, coords[j].X)
				maxX := max(coords[i].X, coords[j].X)
				for x := minX; x <= maxX; x++ {
					linePath[Coord{
						X: x,
						Y: coords[i].Y,
					}] = struct{}{}
				}
			}
		}
	}

	// So from each coordinate, draw the rectangle, check for intersections
	// on the path. If there is an intersection - it crosses outside
	// the bigger shape.
	part2 := 0
	for i := range coords {
		for j := 0; j < i; j++ {
			if hasIntersection(linePath, coords[i], coords[j]) {
				continue
			}
			area := coords[i].Area(coords[j])
			if area > part2 {
				part2 = area
			}
		}
	}
	fmt.Printf("Part2 %d\n", part2)
}

func hasIntersection(path map[Coord]struct{}, a, b Coord) bool {
	minX := min(a.X, b.X)
	minY := min(a.Y, b.Y)
	maxX := max(a.X, b.X)
	maxY := max(a.Y, b.Y)
	for c := range path {
		if c.X < maxX && c.X > minX && c.Y > minY && c.Y < maxY {
			return true
		}
	}
	return false
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func abs(a int) int {
	if a > 0 {
		return a
	}
	return -a
}

type Coord struct {
	X int
	Y int
}

func (c Coord) Area(other Coord) int {
	// Need to count area inclusive.
	return (abs(c.X-other.X) + 1) * (abs(c.Y-other.Y) + 1)
}

func parseInput(s string) []Coord {
	coords := []Coord{}
	for line := range strings.SplitSeq(s, "\n") {
		coordStr := strings.Split(line, ",")
		X, _ := strconv.Atoi(coordStr[0])
		Y, _ := strconv.Atoi(coordStr[1])
		coords = append(coords, Coord{
			X: X,
			Y: Y,
		})
	}

	return coords
}
