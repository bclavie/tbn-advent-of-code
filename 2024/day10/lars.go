package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var file string

type Point struct {
	X         int
	Y         int
	Height    int
	Rating    int
	Reachable map[[2]int]bool
}

func PrintGrid(grid [][]*Point) {
	for _, row := range grid {
		for _, el := range row {
			fmt.Print(el.Height)
		}
		fmt.Println()
	}
}

func OutOfBounds(grid [][]*Point, x, y int) bool {
	return x < 0 || y < 0 || y > len(grid)-1 || x > len(grid[y])-1
}

func GetPoint(grid [][]*Point, x, y int) *Point {
	if OutOfBounds(grid, x, y) {
		return nil
	}
	return grid[y][x]
}

func calculateTrails(grid [][]*Point, heights map[int][]*Point) {
	// For each tile determine the number of 9s they connect to, going down from 9
	for i := 9; i > 0; i-- {
		for _, el := range heights[i] {
			// Search for next lower number that is reachable and then increment its number of reachable 9s by own count
			if left := GetPoint(grid, el.X-1, el.Y); left != nil && left.Height == el.Height-1 {
				left.Rating += el.Rating
				for point := range el.Reachable {
					left.Reachable[point] = true
				}
			}
			if right := GetPoint(grid, el.X+1, el.Y); right != nil && right.Height == el.Height-1 {
				right.Rating += el.Rating
				for point := range el.Reachable {
					right.Reachable[point] = true
				}
			}
			if top := GetPoint(grid, el.X, el.Y-1); top != nil && top.Height == el.Height-1 {
				top.Rating += el.Rating
				for point := range el.Reachable {
					top.Reachable[point] = true
				}
			}
			if bottom := GetPoint(grid, el.X, el.Y+1); bottom != nil && bottom.Height == el.Height-1 {
				bottom.Rating += el.Rating
				for point := range el.Reachable {
					bottom.Reachable[point] = true
				}
			}
		}
	}
}

func calculateResultA(grid [][]*Point, heights map[int][]*Point) int {
	result := 0

	calculateTrails(grid, heights)
	for _, el := range heights[0] {
		result += len(el.Reachable)
	}

	return result
}

func calculateResultB(grid [][]*Point, heights map[int][]*Point) int {
	result := 0
	calculateTrails(grid, heights)
	for _, el := range heights[0] {
		result += el.Rating
	}

	return result
}

func getResult(part string) int {
	grid, heights := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(grid, heights)
	}

	return calculateResultB(grid, heights)
}

func getInput() (grid [][]*Point, heights map[int][]*Point) {
	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	heights = make(map[int][]*Point)
	grid = make([][]*Point, 0)
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		row := make([]*Point, len(line))
		for i, el := range line {
			height, _ := strconv.Atoi(string(el))
			point := Point{
				X:         i,
				Y:         y,
				Height:    height,
				Reachable: make(map[[2]int]bool),
			}
			if height == 9 {
				point.Reachable[[2]int{point.X, point.Y}] = true
				point.Rating = 1
			}
			row[i] = &point
			heights[height] = append(heights[height], &point)
		}
		grid = append(grid, row)
		y++
	}

	return
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
