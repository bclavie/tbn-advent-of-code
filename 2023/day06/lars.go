package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var file string

var numRe = regexp.MustCompile(`\d+`)

type Race struct {
	Time     int
	Distance int
}

func (r Race) Solve() int {
	numSolutions := 0
	for x := 1; x < r.Time; x++ {
		distance := (r.Time - x) * x
		if distance > r.Distance {
			numSolutions++
		}
	}
	return numSolutions
}

func calculateResultA(input []string) int {
	result := 1
	races := ParseRaces(input)
	for _, race := range races {
		result *= race.Solve()
	}

	return result
}

func calculateResultB(input []string) int {
	times := numRe.FindAllStringSubmatch(input[0], len(input[0]))
	distances := numRe.FindAllStringSubmatch(input[1], len(input[1]))
	timeString, distString := "", ""
	for i := range times {
		timeString += times[i][0]
		distString += distances[i][0]
	}
	time, _ := strconv.Atoi(timeString)
	dist, _ := strconv.Atoi(distString)
	race := Race{time, dist}

	return race.Solve()
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

func ParseRaces(input []string) []Race {
	times := numRe.FindAllStringSubmatch(input[0], len(input[0]))
	distances := numRe.FindAllStringSubmatch(input[1], len(input[1]))
	races := make([]Race, len(times))
	for i := range times {
		time, _ := strconv.Atoi(times[i][0])
		distance, _ := strconv.Atoi(distances[i][0])
		races[i] = Race{
			Time:     time,
			Distance: distance,
		}
	}
	return races
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
