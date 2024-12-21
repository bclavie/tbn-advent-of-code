package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

var file string

func OutOfBounds(grid [][]string, x, y int) bool {
	return x < 0 || y < 0 || y > len(grid)-1 || x > len(grid[y])-1
}

func GetPoint(grid [][]string, x, y int) string {
	if OutOfBounds(grid, x, y) {
		return ""
	}
	return grid[y][x]
}

type Region struct {
	Plant string
	Plots map[[2]int]int // plot -> perimeter contribution
	ToDo  [][2]int
}

func (r Region) GetPerimeter() int {
	sum := 0
	for _, el := range r.Plots {
		sum += el
	}
	return sum
}

func (r Region) GetSides(grid [][]string) int {
	sides := 0
	// To find all top sides use all plots that have a perimeter
	// - find all plots that have no plot in the region above them
	// - put slices of x coordinates into map by height
	// - sort x elements then group
	// Then repeat for left, right and bottom
	// TOP
	// y -> []x
	topBorders := map[int][]int{}
	bottomBorders := map[int][]int{}
	// x -> []y
	leftBorders := map[int][]int{}
	rightBorders := map[int][]int{}
	for el, num := range r.Plots {
		if num > 0 {
			top := GetPoint(grid, el[0], el[1]-1)
			if top != r.Plant {
				topBorders[el[1]] = append(topBorders[el[1]], el[0])
			}
			bottom := GetPoint(grid, el[0], el[1]+1)
			if bottom != r.Plant {
				bottomBorders[el[1]] = append(bottomBorders[el[1]], el[0])
			}
			left := GetPoint(grid, el[0]-1, el[1])
			if left != r.Plant {
				leftBorders[el[0]] = append(leftBorders[el[0]], el[1])
			}
			right := GetPoint(grid, el[0]+1, el[1])
			if right != r.Plant {
				rightBorders[el[0]] = append(rightBorders[el[0]], el[1])
			}
		}
	}
	for _, row := range topBorders {
		slices.Sort(row)
		sides++
		for i := 0; i < len(row)-1; i++ {
			if row[i]+1 != row[i+1] {
				sides++
			}
		}
	}
	for _, row := range bottomBorders {
		slices.Sort(row)
		sides++
		for i := 0; i < len(row)-1; i++ {
			if row[i]+1 != row[i+1] {
				sides++
			}
		}
	}
	for _, column := range leftBorders {
		slices.Sort(column)
		sides++
		for i := 0; i < len(column)-1; i++ {
			if column[i]+1 != column[i+1] {
				sides++
			}
		}
	}
	for _, column := range rightBorders {
		slices.Sort(column)
		sides++
		for i := 0; i < len(column)-1; i++ {
			if column[i]+1 != column[i+1] {
				sides++
			}
		}
	}

	return sides
}

func (r *Region) Explore(grid [][]string, handled map[[2]int]bool) {
	for len(r.ToDo) > 0 {
		current := r.ToDo[0]
		r.ToDo = r.ToDo[1:]
		r.Plots[current] = 0
		handled[current] = true
		neighbours := r.GetNeighbours(current)
		for _, el := range neighbours {
			plant := GetPoint(grid, el[0], el[1])
			if plant != r.Plant {
				r.Plots[current]++
			} else {
				r.ToDo = append(r.ToDo, el)
			}
		}
	}
}

func (r Region) GetNeighbours(current [2]int) [][2]int {
	x, y := current[0], current[1]
	possible := [][2]int{{x - 1, y}, {x + 1, y}, {x, y - 1}, {x, y + 1}}
	actual := make([][2]int, 0)
	for _, el := range possible {
		_, ok := r.Plots[el]
		if !ok && !slices.Contains(r.ToDo, el) {
			actual = append(actual, el)
		}
	}
	return actual
}

func calculateResultA(input [][]string) int {
	result := 0
	handled := make(map[[2]int]bool)

	for y, row := range input {
		for x, el := range row {
			if !handled[[2]int{x, y}] {
				region := Region{
					Plant: el,
					Plots: make(map[[2]int]int),
					ToDo:  [][2]int{{x, y}},
				}
				region.Explore(input, handled)
				result += len(region.Plots) * region.GetPerimeter()
			}
		}
	}

	return result
}

func calculateResultB(input [][]string) int {
	result := 0
	handled := make(map[[2]int]bool)

	for y, row := range input {
		for x, el := range row {
			if !handled[[2]int{x, y}] {
				region := Region{
					Plant: el,
					Plots: make(map[[2]int]int),
					ToDo:  [][2]int{{x, y}},
				}
				region.Explore(input, handled)
				result += len(region.Plots) * region.GetSides(input)
			}
		}
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

func getInput() [][]string {
	input := [][]string{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		row := make([]string, len(line))
		for i, el := range line {
			row[i] = string(el)
		}
		input = append(input, row)
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
