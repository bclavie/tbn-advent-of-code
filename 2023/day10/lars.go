package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

var file string

type Coord struct {
	x int
	y int
}

func (c Coord) GetSymbol(input []string) string {
	return string(input[c.y][c.x])
}

func (c Coord) PrintSymbol(input []string) string {
	symbol := c.GetSymbol(input)
	switch symbol {
	case "7":
		return "┓"
	case "F":
		return "┏"
	case "J":
		return "┛"
	case "L":
		return "┗"
	case "-":
		return "━"
	case "|":
		return "┃"
	default:
		return symbol
	}
}

type Pipe struct {
	co       Coord
	Symbol   string
	prev     Coord
	Distance int
}

func (p *Pipe) GetSymbol(input []string) string {
	if p.Symbol == "" {
		p.Symbol = p.co.GetSymbol(input)
	}
	return p.Symbol
}

func (p Pipe) String() string {
	return fmt.Sprintf("%v -> %s (%d) -> %v", p.prev, p.Symbol, p.Distance, p.co)
}

func (p Pipe) isTop(x, y int) bool {
	return x == p.co.x && y == p.co.y-1
}

func (p Pipe) isBottom(x, y int) bool {
	return x == p.co.x && y == p.co.y+1
}

func (p Pipe) isLeft(x, y int) bool {
	return x == p.co.x-1 && y == p.co.y
}

func (p Pipe) isRight(x, y int) bool {
	return x == p.co.x+1 && y == p.co.y
}

func (p Pipe) Connects(x, y int) bool {
	switch p.Symbol {
	case "|":
		return p.isBottom(x, y) || p.isTop(x, y)
	case "-":
		return p.isLeft(x, y) || p.isRight(x, y)
	case "L":
		return p.isTop(x, y) || p.isRight(x, y)
	case "J":
		return p.isTop(x, y) || p.isLeft(x, y)
	case "7":
		return p.isBottom(x, y) || p.isLeft(x, y)
	case "F":
		return p.isBottom(x, y) || p.isRight(x, y)
	}
	return false
}

func (p Pipe) Traverse(input []string) Pipe {
	next := Pipe{prev: p.co, Distance: p.Distance + 1}
	top := Coord{p.co.x, p.co.y - 1}
	bottom := Coord{p.co.x, p.co.y + 1}
	left := Coord{p.co.x - 1, p.co.y}
	right := Coord{p.co.x + 1, p.co.y}
	switch p.Symbol {
	case "|":
		if p.isTop(p.prev.x, p.prev.y) {
			next.co = bottom
		} else {
			next.co = top
		}
	case "-":
		if p.isLeft(p.prev.x, p.prev.y) {
			next.co = right
		} else {
			next.co = left
		}
	case "L":
		if p.isTop(p.prev.x, p.prev.y) {
			next.co = right
		} else {
			next.co = top
		}
	case "J":
		if p.isTop(p.prev.x, p.prev.y) {
			next.co = left
		} else {
			next.co = top
		}
	case "7":
		if p.isBottom(p.prev.x, p.prev.y) {
			next.co = left
		} else {
			next.co = bottom
		}
	case "F":
		if p.isBottom(p.prev.x, p.prev.y) {
			next.co = right
		} else {
			next.co = bottom
		}
	}
	next.GetSymbol(input)
	return next
}

func calculateResultA(input []string) int {
	_, connecting := GetStart(input)
	for {
		for i := range connecting {
			connecting[i] = connecting[i].Traverse(input)
		}
		if connecting[0].Connects(connecting[1].co.x, connecting[1].co.y) || connecting[0].co == connecting[1].co {
			return connecting[0].Distance
		}
	}
}

func calculateResultB(input []string) int {
	// attempt to count for each point in the loop if it within or without by counting "|" and "-" pieces
	start, connecting := GetStart(input)
	loop := []Coord{start.co, connecting[0].co, connecting[1].co}
	for {
		for i := range connecting {
			connecting[i] = connecting[i].Traverse(input)
			loop = append(loop, connecting[i].co)
		}
		if connecting[0].Connects(connecting[1].co.x, connecting[1].co.y) || connecting[0].co == connecting[1].co {
			break
		}
	}
	result := 0
	for y, line := range input {
		inLoop := false
		onPipe := false
		hitTop, hitBottom := false, false
		for x := range line {
			el := Coord{x, y}
			// If we hit a pipe piece we only invert in/out when a continous piece of pipe connects to top and bottom
			if slices.Contains(loop, el) {
				// If we continuously are on connecting pipe
				if onPipe && (Pipe{co: el, Symbol: el.GetSymbol(input)}).Connects(x-1, y) {
					// we check if this piece hits top or bottom add that to the memory
					hitTop = hitTop || slices.Contains([]string{"|", "J", "L"}, el.GetSymbol(input))
					hitBottom = hitBottom || slices.Contains([]string{"|", "7", "F"}, el.GetSymbol(input))
					// If we were not on pipe or it was not connecting
				} else {
					onPipe = true
					// we reset the top and bottom stat starting with this piece of pipe
					hitTop = slices.Contains([]string{"|", "J", "L"}, el.GetSymbol(input))
					hitBottom = slices.Contains([]string{"|", "7", "F"}, el.GetSymbol(input))
				}
				// if we hit both top and bottom we invert the in/out state
				if hitTop && hitBottom {
					inLoop = !inLoop
				}
				// if we dont hit a pipe piece
			} else {
				onPipe = false
				// and are currently in loop we increment
				if inLoop {
					result++
				}
			}
		}
	}

	return result

}

func GetStart(input []string) (Pipe, []Pipe) {
	var start Pipe
	// find coordinates
Loop:
	for y, line := range input {
		for x := range line {
			coord := Pipe{co: Coord{x, y}}
			if coord.GetSymbol(input) == "S" {
				start = coord
				break Loop
			}
		}
	}
	// figure out 2 connecting pieces
	connecting := []Pipe{}
	for y := start.co.y - 1; y <= start.co.y+1; y++ {
		for x := start.co.x - 1; x <= start.co.x+1; x++ {
			if y >= 0 && y < len(input) && x >= 0 && x < len(input[0]) {
				test := Pipe{co: Coord{x, y}, prev: start.co, Distance: 1}
				test.GetSymbol(input)
				if test.Connects(start.co.x, start.co.y) {
					connecting = append(connecting, test)
				}
			}
		}
	}
	return start, connecting
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
