package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
)

var file string

type Map struct {
	rows      [][]string
	guard     [2]int
	direction [2]int
	dirSymbol string
	obstacles [][2]int
	startPos  [2]int
	startDir  [2]int
}

func (m Map) String() string {
	out := fmt.Sprintf("G: %v ==> %v\n", m.guard, m.direction)
	for _, row := range m.rows {
		out += fmt.Sprintf("%v\n", row)
	}
	return out
}

func (m Map) Copy() Map {
	n := Map{
		rows:      make([][]string, len(m.rows)),
		guard:     [2]int{m.startPos[0], m.startPos[1]},
		direction: [2]int{m.startDir[0], m.startDir[1]},
		dirSymbol: m.dirSymbol,
	}
	for i, row := range m.rows {
		n.rows[i] = make([]string, len(row))
		for j := range row {
			n.rows[i][j] = m.rows[i][j]
		}
	}
	return n
}

func (m *Map) Step(obstacles bool) {
	if obstacles {
		m.ObstacleCheck()
	}
	// set old as visited using current direction or keeping +
	old := m.rows[m.guard[1]][m.guard[0]]
	if old == "|" && m.dirSymbol == "-" || old == "-" && m.dirSymbol == "|" || old == "+" {
		m.rows[m.guard[1]][m.guard[0]] = "+"
	} else {
		m.rows[m.guard[1]][m.guard[0]] = m.dirSymbol
	}
	// set new position and check if we run out of bounds
	x, y := m.guard[0]+m.direction[0], m.guard[1]+m.direction[1]
	outOfBounds := m.OutOfBounds(x, y)
	if !outOfBounds {
		// check if we need to turn away from obstacle
		for m.rows[y][x] == "#" || m.rows[y][x] == "O" {
			m.direction, m.dirSymbol = RotateGuard(m.direction)
			x, y = m.guard[0]+m.direction[0], m.guard[1]+m.direction[1]
			// also set to + if we rotated
			m.rows[m.guard[1]][m.guard[0]] = "+"
		}
	}
	// move guard
	m.guard = [2]int{x, y}
}

func (m Map) OutOfBounds(x, y int) bool {
	return x < 0 || y < 0 || y > len(m.rows)-1 || x > len(m.rows[y])-1
}

// Place obstacle on current guard position and try again to see if it loops
func (m *Map) ObstacleCheck() {
	x, y := m.guard[0], m.guard[1]
	// dont do it if this is the starting pos
	if x == m.startPos[0] && y == m.startPos[1] {
		return
	}
	// create map copy with guard at starting pos
	n := m.Copy()
	// add new obstacle position
	outOfBounds := n.OutOfBounds(x, y)
	if outOfBounds {
		return
	}
	n.rows[y][x] = "O"
	visited := [][2][2]int{{{n.guard[0], n.guard[1]}, n.direction}}
	// run it with a loop check
	for !n.Finished() {
		n.Step(false)
		curr := [2][2]int{{n.guard[0], n.guard[1]}, n.direction}
		if slices.Contains(visited, curr) {
			if !slices.Contains(m.obstacles, [2]int{x, y}) {
				m.obstacles = append(m.obstacles, [2]int{x, y})
			}
			return
		} else {
			visited = append(visited, curr)
		}
	}
}

func (m Map) Finished() bool {
	x := m.guard[0]
	y := m.guard[1]
	if x < 0 || y < 0 || y > len(m.rows)-1 || x > len(m.rows[y])-1 {
		return true
	}
	return false
}

func (m Map) Score() int {
	result := 0
	for _, row := range m.rows {
		for _, cell := range row {
			if slices.Contains([]string{"X", "|", "-", "+"}, cell) {
				result++
			}
		}
	}
	return result
}

func RotateGuard(direction [2]int) ([2]int, string) {
	switch direction {
	case [2]int{0, -1}: // ^
		return [2]int{1, 0}, "-"
	case [2]int{1, 0}: // >
		return [2]int{0, 1}, "|"
	case [2]int{0, 1}: // v
		return [2]int{-1, 0}, "-"
	case [2]int{-1, 0}: // <
		return [2]int{0, -1}, "|"
	}
	return [2]int{0, 0}, ""
}

func calculateResultA(labMap Map) int {
	for !labMap.Finished() {
		labMap.Step(false)
	}

	return labMap.Score()
}

func calculateResultB(labMap Map) int {
	for !labMap.Finished() {
		labMap.Step(true)
	}

	return len(labMap.obstacles)
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() Map {

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	labMap := Map{
		rows:      make([][]string, 0),
		obstacles: [][2]int{},
	}
	for scanner.Scan() {
		line := scanner.Text()
		labMap.rows = append(labMap.rows, strings.Split(line, ""))
	}
	for y, row := range labMap.rows {
		for x, cell := range row {
			switch cell {
			case "^":
				labMap.guard = [2]int{x, y}
				labMap.direction = [2]int{0, -1}
				labMap.dirSymbol = "|"
			case ">":
				labMap.guard = [2]int{x, y}
				labMap.direction = [2]int{1, 0}
				labMap.dirSymbol = "-"
			case "v":
				labMap.guard = [2]int{x, y}
				labMap.direction = [2]int{0, 1}
				labMap.dirSymbol = "|"
			case "<":
				labMap.guard = [2]int{x, y}
				labMap.direction = [2]int{-1, 0}
				labMap.dirSymbol = "-"
			}
		}
	}
	labMap.startPos = [2]int{labMap.guard[0], labMap.guard[1]}
	labMap.startDir = [2]int{labMap.direction[0], labMap.direction[1]}

	return labMap
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
