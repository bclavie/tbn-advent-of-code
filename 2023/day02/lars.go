package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var file string
var allowed = map[string]int{
	"red":   12,
	"green": 13,
	"blue":  14,
}

type Game struct {
	Nr    int
	Draws []string
}

func (g Game) IsPossible() bool {
	for _, draw := range g.Draws {
		for color, max := range allowed {
			amount := getColorNumber(draw, color)
			if amount > max {
				return false
			}
		}
	}
	return true
}

func (g Game) Power() int {
	maxAmounts := make(map[string]int)
	for _, draw := range g.Draws {
		for color := range allowed {
			amount := getColorNumber(draw, color)
			if max, ok := maxAmounts[color]; !ok || amount > max {
				maxAmounts[color] = amount
			}
		}
	}
	return maxAmounts["red"] * maxAmounts["green"] * maxAmounts["blue"]
}

func getColorNumber(draw, color string) int {
	re := regexp.MustCompile(fmt.Sprintf(`(\d+)\s%s`, color))
	matches := re.FindStringSubmatch(draw)
	if matches != nil {
		amount, err := strconv.Atoi(matches[1])
		if err != nil {
			log.Fatalf(err.Error())
		}
		return amount
	}
	return 0
}

func calculateResultA(input []string) int {
	result := 0
	games := parseGames(input)
	for _, game := range games {
		if game.IsPossible() {
			result += game.Nr
		}
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0
	games := parseGames(input)
	for _, game := range games {
		result += game.Power()
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

func parseGames(input []string) []Game {
	games := make([]Game, len(input))
	for i, line := range input {
		games[i] = Game{
			Nr:    i + 1,
			Draws: strings.Split(line, ";"),
		}
	}
	return games
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
