package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"slices"
	"strconv"
)

var file string

type ShiftRange struct {
	Source int
	Length int
	Shift  int
}

func (s ShiftRange) Max() int {
	return s.Source + s.Length
}

func (s ShiftRange) Contains(num int) bool {
	if s.Source <= num && num <= s.Max() {
		return true
	}
	return false
}

type Mapping struct {
	Name   string
	Ranges []ShiftRange
}

func (m Mapping) Transform(seed int) int {
	for _, el := range m.Ranges {
		if el.Contains(seed) {
			return seed + el.Shift
		}
	}
	return seed
}

func (m Mapping) Reverse(destination int) int {
	for _, el := range m.Ranges {
		if el.Source+el.Shift <= destination && destination <= el.Max()+el.Shift {
			return destination - el.Shift
		}
	}
	return destination
}

func calculateResultA(input []string) int {
	result := math.MaxInt

	seedRanges, mappings := parseInput(input, false)
	for _, seedRange := range seedRanges {
		for seed := seedRange.Source; seed <= seedRange.Max(); seed++ {
			current := seed
			for _, mapping := range mappings {
				current = mapping.Transform(current)
			}
			if current < result {
				result = current
			}
		}
	}

	return result
}

func calculateResultB(input []string) int {
	seedRanges, mappings := parseInput(input, true)
	slices.Reverse(mappings)
	for i := 0; ; i++ {
		current := i
		for _, mapping := range mappings {
			current = mapping.Reverse(current)
		}
		for _, seedRange := range seedRanges {
			if seedRange.Contains(current) {
				return i
			}
		}
	}
}

func parseInput(input []string, seedRanges bool) (seeds []ShiftRange, mappings []Mapping) {
	var seedRe = regexp.MustCompile(`(\d+)+`)
	var newMapRe = regexp.MustCompile(`(.+) map:`)
	var mapRowRe = regexp.MustCompile(`^(\d+) (\d+) (\d+)$`)
	for lineNR, line := range input {
		if lineNR == 0 {
			seedMatches := seedRe.FindAllStringSubmatch(line, len(line))
			for i, el := range seedMatches {
				if num, err := strconv.Atoi(el[0]); err == nil {
					if i%2 == 1 && seedRanges {
						seeds[len(seeds)-1].Length = num
						continue
					}
					seeds = append(seeds, ShiftRange{Source: num, Length: 1, Shift: 0})
				}
			}
		}
		newMapMatch := newMapRe.FindStringSubmatch(line)
		if newMapMatch != nil {
			mappings = append(mappings, Mapping{Name: newMapMatch[1]})
		}
		mapRowMatch := mapRowRe.FindStringSubmatch(line)
		if mapRowMatch != nil {
			destination, _ := strconv.Atoi(mapRowMatch[1])
			source, _ := strconv.Atoi(mapRowMatch[2])
			length, _ := strconv.Atoi(mapRowMatch[3])
			shiftRange := ShiftRange{
				Source: source,
				Shift:  destination - source,
				Length: length,
			}
			mappings[len(mappings)-1].Ranges = append(mappings[len(mappings)-1].Ranges, shiftRange)
		}
	}

	return
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
