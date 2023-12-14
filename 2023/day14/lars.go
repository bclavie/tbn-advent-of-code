package main

import (
	"bufio"
	"fmt"
	"os"
)

var file string

type Platform struct {
	// [y][x] [row][column]
	rows     [][]string
	original [][]string
	height   int
	width    int
}

func (p Platform) Print() {
	for i := range p.original {
		fmt.Printf("%v  |  %v\n", p.original[i], p.rows[i])
	}
	fmt.Println("Weight", p.Weight())
}

func (p Platform) Weight() (weight int) {
	for y, row := range p.rows {
		for _, el := range row {
			if el == "O" {
				weight += p.height - y
			}
		}
	}
	return
}

func (p *Platform) cycle() {
	p.tilt("north")
	p.tilt("west")
	p.tilt("south")
	p.tilt("east")
}

func (p *Platform) tilt(direction string) {
	switch direction {
	case "north":
		p.tiltVertical(true)
	case "south":
		p.tiltVertical(false)
	case "east":
		p.tiltHorizontal(false)
	case "west":
		p.tiltHorizontal(true)
	}
}

func (p *Platform) tiltVertical(north bool) {
	for x := 0; x < p.width; x++ {
		blocks := p.FindColumnBlocks(x)
		for i := 0; i < len(blocks)-1; i++ {
			start, end := blocks[i], blocks[i+1]
			rocks := p.CountRocksInColumnInterval(x, start, end)
			if !north {
				start, end = end, start
			}
			direction := 1
			if end < start {
				direction = -1
			}
			for y := start + direction; y != end; y += direction {
				if rocks > 0 {
					p.rows[y][x] = "O"
					rocks--
				} else {
					p.rows[y][x] = "."
				}
			}
		}
	}
}

func (p *Platform) tiltHorizontal(west bool) {
	for y := 0; y < p.width; y++ {
		blocks := p.FindRowBlocks(y)
		for i := 0; i < len(blocks)-1; i++ {
			start, end := blocks[i], blocks[i+1]
			rocks := p.CountRocksInRowInterval(y, start, end)
			if !west {
				start, end = end, start
			}
			direction := 1
			if end < start {
				direction = -1
			}
			for x := start + direction; x != end; x += direction {
				if rocks > 0 {
					p.rows[y][x] = "O"
					rocks--
				} else {
					p.rows[y][x] = "."
				}
			}
		}
	}
}

func (p Platform) FindColumnBlocks(column int) (blocks []int) {
	blocks = []int{-1}
	for y := 0; y < p.height; y++ {
		if p.rows[y][column] == "#" {
			blocks = append(blocks, y)
		}
	}
	blocks = append(blocks, p.height)
	return
}

func (p Platform) FindRowBlocks(row int) (blocks []int) {
	blocks = []int{-1}
	for x := 0; x < p.width; x++ {
		if p.rows[row][x] == "#" {
			blocks = append(blocks, x)
		}
	}
	blocks = append(blocks, p.width)
	return
}

// Count all the rocks in the column interval excluding the extremas
func (p Platform) CountRocksInColumnInterval(column, min, max int) (value int) {
	for y := min + 1; y < max; y++ {
		if p.rows[y][column] == "O" {
			value++
		}
	}
	return
}

// Count all the rocks in the row interval excluding the extremas
func (p Platform) CountRocksInRowInterval(row, min, max int) (value int) {
	for x := min + 1; x < max; x++ {
		if p.rows[row][x] == "O" {
			value++
		}
	}
	return
}

func calculateResultA(input []string) int {
	platform := ParsePlatform(input)
	platform.tilt("north")

	return platform.Weight()
}

func calculateResultB(input []string) int {
	platform := ParsePlatform(input)
	sequence := []int{platform.Weight()}
	// brents cycle detection algortihm
	f := func(x int) int {
		for x > len(sequence)-1 {
			platform.cycle()
			sequence = append(sequence, platform.Weight())
		}
		return sequence[x]
	}
	controlLam := func(lam int) bool {
		if lam < 2 {
			return false
		}
		tortoise, hare := 0, lam
		mu := 0
		for f(tortoise) != f(hare) {
			tortoise++
			hare++
			mu++
		}
		for i := 0; i < lam; i++ {
			if f(tortoise+i) != f(hare+i) {
				return false
			}
		}
		return true
	}
	tortoise := 0
	hare := 1
	power, lam := 1, 1
	done := false
	for !done {
		if power == lam {
			tortoise = hare
			power *= 2
			lam = 0
		}
		hare++
		lam++
		if f(tortoise) == f(hare) {
			done = controlLam(lam)
		}
	}
	cycles := 1000000000
	cycleIndex := (cycles - tortoise) % lam
	result := f(tortoise + cycleIndex)

	return result

}

func ParsePlatform(input []string) Platform {
	rows := make([][]string, len(input))
	moved := make([][]string, len(input))
	for y, line := range input {
		rows[y] = make([]string, len(line))
		moved[y] = make([]string, len(line))
		for x, el := range line {
			rows[y][x] = string(el)
			moved[y][x] = string(el)
		}
	}
	height := len(rows)
	width := len(rows[0])
	return Platform{rows, moved, height, width}
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
