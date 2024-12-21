package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var file string

func numLength(num int) int {
	count := 0
	for num > 0 {
		num = num / 10
		count++
	}
	return count
}

var BlinkCache = make(map[[2]int]int)

// Returns the amount of stones this turned into after blinking given times
func Blink(stone, times int) int {
	// if no blinks left, return that there is one stone
	if times < 1 {
		return 1
	}
	// Check for cache hit
	if value, ok := BlinkCache[[2]int{stone, times}]; ok {
		return value
	}
	// Check which rule to apply then go again
	if stone == 0 {
		value := Blink(1, times-1)
		BlinkCache[[2]int{1, times - 1}] = value
		return value
	}
	if numLength(stone)%2 == 0 {
		left, right := Split(stone)
		leftValue := Blink(left, times-1)
		rightValue := Blink(right, times-1)
		BlinkCache[[2]int{left, times - 1}] = leftValue
		BlinkCache[[2]int{right, times - 1}] = rightValue
		return leftValue + rightValue
	}
	value := Blink(stone*2024, times-1)
	BlinkCache[[2]int{stone * 2024, times - 1}] = value
	return value
}

func Split(stone int) (int, int) {
	valueString := strconv.Itoa(stone)
	left, _ := strconv.Atoi(valueString[:len(valueString)/2])
	right, _ := strconv.Atoi(valueString[len(valueString)/2:])
	return left, right
}

func calculateResultA(input []int) int {
	result := 0
	for _, el := range input {
		result += Blink(el, 25)
	}

	return result
}

func calculateResultB(input []int) int {
	result := 0
	for _, el := range input {
		result += Blink(el, 75)
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

func getInput() []int {
	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanWords)

	var input []int
	for scanner.Scan() {
		word := scanner.Text()
		num, _ := strconv.Atoi(word)
		input = append(input, num)
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
