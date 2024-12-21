package main

import (
	"bufio"
	"fmt"
	"os"
)

var file string

type Point struct {
	X int
	Y int
}

func getAntinodes(group []Point, xMax, yMax int, all bool) []Point {
	antinodes := make([]Point, 0)
	for i := 0; i < len(group); i++ {
		a := group[i]
		for j := i + 1; j < len(group); j++ {
			b := group[j]
			inbounds := true
			if all {
				antinodes = append(antinodes, a, b)
			}
			for i := 1; inbounds; i++ {
				one := Point{a.X + i*(a.X-b.X), a.Y + i*(a.Y-b.Y)}
				inbounds = all
				if !outOfBounds(one, xMax, yMax) {
					antinodes = append(antinodes, one)
				} else {
					inbounds = false
				}
			}
			inbounds = true
			for i := 1; inbounds; i++ {
				two := Point{b.X + i*(b.X-a.X), b.Y + i*(b.Y-a.Y)}
				inbounds = all
				if !outOfBounds(two, xMax, yMax) {
					antinodes = append(antinodes, two)
				} else {
					inbounds = false
				}
			}
		}
	}
	return antinodes
}

func outOfBounds(point Point, xMax, yMax int) bool {
	return point.X < 0 || point.Y < 0 || point.X > xMax || point.Y > yMax
}

func calculateResultA(antennas map[string][]Point, xMax, yMax int) int {
	antinodes := make(map[Point]bool)
	for _, group := range antennas {
		groupAntinodes := getAntinodes(group, xMax, yMax, false)
		for _, el := range groupAntinodes {
			antinodes[el] = true
		}
	}
	return len(antinodes)
}

func calculateResultB(antennas map[string][]Point, xMax, yMax int) int {
	antinodes := make(map[Point]bool)
	for _, group := range antennas {
		groupAntinodes := getAntinodes(group, xMax, yMax, true)
		for _, el := range groupAntinodes {
			antinodes[el] = true
		}
	}
	return len(antinodes)
}

func getResult(part string) int {
	antennas, xMax, yMax := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(antennas, xMax, yMax)
	}

	return calculateResultB(antennas, xMax, yMax)
}

func getInput() (antennas map[string][]Point, xMax, yMax int) {
	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	xMax, yMax = 0, -1
	antennas = make(map[string][]Point)
	for scanner.Scan() {
		line := scanner.Text()
		xMax = len(line) - 1
		yMax++
		for x, el := range line {
			if el != '.' {
				antennas[string(el)] = append(antennas[string(el)], Point{
					X: x,
					Y: yMax,
				})
			}
		}
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
