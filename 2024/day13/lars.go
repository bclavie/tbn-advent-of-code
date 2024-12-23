package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

var file string

var ButtonARegex = regexp.MustCompile(`Button A: X\+(\d+), Y\+(\d+)`)
var ButtonBRegex = regexp.MustCompile(`Button B: X\+(\d+), Y\+(\d+)`)
var PrizeRegex = regexp.MustCompile(`Prize: X=(\d+), Y=(\d+)`)

type Game struct {
	A     [2]int
	B     [2]int
	Prize [2]int
}

// Simple solution before I had to play with linear equations
func (g Game) TryCombinations(max int) (int, int) {
	a, b := 0, max
	test := g.Test(a, b)
	for test != [2]int{0, 0} && a <= 100 && b >= 0 {
		if test[0] > 0 || test[1] > 0 {
			b--
		}
		if test[0] < 0 || test[1] < 0 {
			a++
		}
		test = g.Test(a, b)
	}
	if test == [2]int{0, 0} {
		return a, b
	}

	return 0, 0
}

func (g Game) Solve() (int, int) {
	x := float64(g.Prize[0])
	y := float64(g.Prize[1])
	AX := float64(g.A[0])
	BX := float64(g.B[0])
	AY := float64(g.A[1])
	BY := float64(g.B[1])
	a := (x*BY/BX - y) / (BY*AX/BX - AY)
	b := (x*AY/AX - y) / (BX*AY/AX - BY)
	return int(math.Round(a)), int(math.Round(b))
}

func (g Game) Test(a, b int) [2]int {
	xDiff := a*g.A[0] + b*g.B[0] - g.Prize[0]
	yDiff := a*g.A[1] + b*g.B[1] - g.Prize[1]
	return [2]int{xDiff, yDiff}
}

func calculateResultA(input []Game) int {
	result := 0

	for _, el := range input {
		a, b := el.Solve()
		if a > 0 && a < 100 && b > 0 && b < 100 && el.Test(a, b) == [2]int{0, 0} {
			result += 3*a + b
		}
	}

	return result
}

func calculateResultB(input []Game) int {
	result := 0

	for _, el := range input {
		el.Prize[0] += 10000000000000
		el.Prize[1] += 10000000000000
		a, b := el.Solve()
		if a > 100 && b > 100 && el.Test(a, b) == [2]int{0, 0} {
			fmt.Println(el, a, b)
			result += 3*a + b
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

func getInput() []Game {
	input := []Game{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	var game Game
	for scanner.Scan() {
		line := scanner.Text()
		if matches := ButtonARegex.FindStringSubmatch(line); len(matches) > 0 {
			aX, _ := strconv.Atoi(matches[1])
			aY, _ := strconv.Atoi(matches[2])
			game = Game{
				A: [2]int{aX, aY},
			}
		}
		if matches := ButtonBRegex.FindStringSubmatch(line); len(matches) > 0 {
			bX, _ := strconv.Atoi(matches[1])
			bY, _ := strconv.Atoi(matches[2])
			game.B = [2]int{bX, bY}
		}
		if matches := PrizeRegex.FindStringSubmatch(line); len(matches) > 0 {
			pX, _ := strconv.Atoi(matches[1])
			pY, _ := strconv.Atoi(matches[2])
			game.Prize = [2]int{pX, pY}
			input = append(input, game)
		}
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
