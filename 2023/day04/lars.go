package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

var file string

type Card struct {
	Amount  int
	Winning []int
	Have    []int
}

func (c Card) GetPoints() int {
	matching := c.NumMatching()
	return int(1 * math.Pow(2, float64(matching-1)))
}

func (c Card) NumMatching() int {
	hash := map[int]bool{}
	overlap := []int{}
	for _, el := range c.Winning {
		hash[el] = true
	}
	for _, el := range c.Have {
		if hash[el] {
			overlap = append(overlap, el)
			hash[el] = false
		}
	}
	return len(overlap)
}

func calculateResultA(input []string) int {
	result := 0
	cards := parseCards(input)
	for _, card := range cards {
		result += card.GetPoints()
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0
	cards := parseCards(input)
	for i, card := range cards {
		matching := card.NumMatching()
		for j := i + 1; j <= i+matching; j++ {
			cards[j].Amount += card.Amount
		}
		result += card.Amount
	}

	return result
}

func parseCards(input []string) []Card {
	cards := make([]Card, len(input))
	for i, line := range input {
		numbers := strings.Split(strings.Split(line, ":")[1], "|")
		winningStrings := strings.Split(numbers[0], " ")
		haveStrings := strings.Split(numbers[1], " ")
		cards[i] = Card{
			Amount:  1,
			Winning: []int{},
			Have:    []int{},
		}
		for _, el := range winningStrings {
			if num, err := strconv.Atoi(el); err == nil {
				cards[i].Winning = append(cards[i].Winning, num)
			}
		}
		for _, el := range haveStrings {
			if num, err := strconv.Atoi(el); err == nil {
				cards[i].Have = append(cards[i].Have, num)
			}
		}
	}
	return cards
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
