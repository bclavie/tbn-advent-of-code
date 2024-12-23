package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var file string

var width = 101
var height = 103

var RoboRegex = regexp.MustCompile(`p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)`)

type Robot struct {
	ID int
	P  [2]int
	V  [2]int
}

func (r *Robot) Move(time, x, y int) {
	r.P[0] = (r.P[0] + (time * r.V[0]) + (time * x)) % x
	r.P[1] = (r.P[1] + (time * r.V[1]) + (time * y)) % y
}

func (r Robot) Quadrant(x, y int) int {
	if r.P[0] < x/2 {
		if r.P[1] < y/2 {
			return 0
		} else if r.P[1] > y/2 {
			return 2
		}
	} else if r.P[0] > x/2 {
		if r.P[1] < y/2 {
			return 1
		} else if r.P[1] > y/2 {
			return 3
		}
	}
	return -1
}

func (r Robot) HasAdjacent(grid [][]string) bool {
	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			if (x != 0 || y != 0) && !OutOfBounds(grid, r.P[0]+x, r.P[1]+y) && grid[r.P[1]+y][r.P[0]+x] != "." {
				// if r.ID == 345 {
				// 	fmt.Printf("Robot %v has robot %v adjacent\n", r.ID, grid[r.P[1]+y][r.P[0]+x])
				// }
				return true
			}
		}
	}
	return false
}

func OutOfBounds(grid [][]string, x, y int) bool {
	return x < 0 || y < 0 || y > len(grid)-1 || x > len(grid[y])-1
}

func DrawRobots(robots []Robot, time int) {
	grid := make([][]string, height)
	for y := 0; y < height; y++ {
		grid[y] = make([]string, width)
		for x := 0; x < width; x++ {
			grid[y][x] = "."
		}
	}
	for _, r := range robots {
		grid[r.P[1]][r.P[0]] = "X"
	}
	hasAdjacent := 0
	for _, r := range robots {
		if r.HasAdjacent(grid) {
			hasAdjacent++
		}
	}
	percentAdjacent := float64(hasAdjacent) / float64(len(robots))
	if percentAdjacent > 0.6 {
		fmt.Printf("===== %vs (%v) =====\n", time, percentAdjacent)
		for _, row := range grid {
			fmt.Println(row)
		}
	}
}

func calculateResultA(input []Robot) int {
	result := 1

	quadrants := [4]int{0, 0, 0, 0}

	for _, el := range input {
		el.Move(100, width, height)
		quad := el.Quadrant(width, height)
		if quad > -1 {
			quadrants[el.Quadrant(width, height)]++
		}
	}

	for _, el := range quadrants {
		result *= el
	}

	return result
}

func calculateResultB(input []Robot) int {
	result := 0

	maxTime := 10000
	for time := 1; time <= maxTime; time++ {
		for i := range input {
			input[i].Move(1, width, height)
		}
		DrawRobots(input, time)
	}

	return result
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() []Robot {
	input := []Robot{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	id := 0
	for scanner.Scan() {
		line := scanner.Text()
		matches := RoboRegex.FindStringSubmatch(line)
		px, _ := strconv.Atoi(matches[1])
		py, _ := strconv.Atoi(matches[2])
		vx, _ := strconv.Atoi(matches[3])
		vy, _ := strconv.Atoi(matches[4])
		robot := Robot{
			ID: id,
			P:  [2]int{px, py},
			V:  [2]int{vx, vy},
		}
		id++
		input = append(input, robot)
	}

	return input
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
