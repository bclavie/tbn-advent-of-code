package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

var file string

func calculateResultA(input []string) int {
	result := 0
	xLocations := findAll(input, 'X')

	for _, x := range xLocations {
		directions := findSurrounding(input, x, 'M')
		for _, dir := range directions {
			m := [2]int{x[0] + dir[0], x[1] + dir[1]}
			a := findDirected(input, m, dir, 'A')
			if a == nil {
				continue
			}
			s := findDirected(input, *a, dir, 'S')
			if s == nil {
				continue
			}
			result++
		}
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0
	aLocations := findAll(input, 'A')
	for _, el := range aLocations {
		corners := findCornerPairs(input, el)
		if corners != nil {
			if slices.Contains(corners[0], "M") && slices.Contains(corners[0], "S") &&
				slices.Contains(corners[1], "M") && slices.Contains(corners[1], "S") {
				result++
			}
		}
	}

	return result
}

func findAll(input []string, search byte) [][2]int {
	locations := [][2]int{}
	for y := range input {
		for x := range input[y] {
			if input[y][x] == search {
				locations = append(locations, [2]int{x, y})
			}
		}
	}
	return locations
}

func findSurrounding(input []string, coordinates [2]int, search byte) [][2]int {
	result := make([][2]int, 0)
	for yDir := -1; yDir <= 1; yDir++ {
		for xDir := -1; xDir <= 1; xDir++ {
			x := coordinates[0] + xDir
			y := coordinates[1] + yDir
			if x < 0 || y < 0 || y > len(input)-1 || x > len(input[y])-1 {
				continue
			}
			if input[y][x] == search {
				result = append(result, [2]int{xDir, yDir})
			}
		}
	}
	return result
}

func findDirected(input []string, coordinates, direction [2]int, search byte) *[2]int {
	y := coordinates[1] + direction[1]
	x := coordinates[0] + direction[0]
	if x < 0 || y < 0 || y > len(input)-1 || x > len(input[y])-1 {
		return nil
	}
	if input[y][x] == search {
		return &[2]int{x, y}
	}
	return nil
}

func findCornerPairs(input []string, coordinates [2]int) [][]string {
	x := coordinates[0]
	y := coordinates[1]
	if x == 0 || y == 0 || y == len(input)-1 || x == len(input[y])-1 {
		return nil
	}
	return [][]string{
		{
			string(input[y-1][x-1]),
			string(input[y+1][x+1])},
		{
			string(input[y-1][x+1]),
			string(input[y+1][x-1]),
		}}
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() []string {
	input := []string{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		input = append(input, line)
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
