package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"sort"
	"strconv"
	"strings"
)

var file string

type Hand struct {
	Cards    string
	Bid      int
	Strength int
}

func (h Hand) CardStrength(i int, order []string) int {
	card := h.Cards[i : i+1]
	return slices.Index(order, card)
}

func (h *Hand) Evaluate(partB bool) int {
	if h.Strength == 0 {
		labelMap := map[rune]int{}
		jokers := 0
		for _, el := range h.Cards {
			if partB && el == 'J' {
				jokers++
			} else {
				labelMap[el]++
			}
		}
		most := 0
		secondMost := 0
		for _, nr := range labelMap {
			if nr > most {
				secondMost = most
				most = nr
			} else if nr > secondMost {
				secondMost = nr
			}
		}
		if most+jokers == 5 {
			h.Strength = 7
		} else if most+jokers == 4 {
			h.Strength = 6
		} else if most == 3 && secondMost == 2 || most+secondMost+jokers == 5 {
			h.Strength = 5
		} else if most+jokers == 3 {
			h.Strength = 4
		} else if most == 2 && secondMost == 2 || most+secondMost+jokers == 4 {
			h.Strength = 3
		} else if most+jokers == 2 {
			h.Strength = 2
		} else if most == 1 {
			h.Strength = 1
		}
	}
	return h.Strength
}

func calculateResult(input []string, partB bool) int {
	var order []string
	if partB {
		order = []string{"J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"}
	} else {
		order = []string{"2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"}
	}
	result := 0
	hands := ParseHands(input)
	sort.SliceStable(hands, func(i, j int) bool {
		strengthI := hands[i].Evaluate(partB)
		strengthJ := hands[j].Evaluate(partB)
		if strengthI != strengthJ {
			return strengthI < strengthJ
		}
		for k := 0; k < 5; k++ {
			if hands[i].CardStrength(k, order) != hands[j].CardStrength(k, order) {
				return hands[i].CardStrength(k, order) < hands[j].CardStrength(k, order)
			}
		}
		return false
	})

	for rank, hand := range hands {
		result += hand.Bid * (rank + 1)
	}

	return result
}

func ParseHands(input []string) []Hand {
	hands := []Hand{}
	for _, line := range input {
		split := strings.Split(line, " ")
		bid, _ := strconv.Atoi(split[1])
		hands = append(hands, Hand{split[0], bid, 0})
	}
	return hands
}

func getResult(part string) int {
	input := getInput()
	secondPart := part != "A"

	return calculateResult(input, secondPart)
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
