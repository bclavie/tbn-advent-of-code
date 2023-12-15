package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var file string

type BoxItem struct {
	Label string
	value int
}

func HashFunction(s string) int {
	result := 0
	for _, el := range s {
		result += int(el)
		result *= 17
		result = result % 256
	}
	return result
}

func AddItem(box []BoxItem, label string, value int) []BoxItem {
	newBox := []BoxItem{}
	added := false
	for _, item := range box {
		if item.Label == label {
			newBox = append(newBox, BoxItem{label, value})
			added = true
		} else {
			newBox = append(newBox, item)
		}
	}
	if !added {
		newBox = append(newBox, BoxItem{label, value})
	}
	return newBox
}

func RemoveItem(box []BoxItem, label string) []BoxItem {
	newBox := []BoxItem{}
	for _, item := range box {
		if item.Label != label {
			newBox = append(newBox, item)
		}
	}
	return newBox
}

func FocusingPower(boxes map[int][]BoxItem) int {
	result := 0
	for i, box := range boxes {
		for j, item := range box {
			result += (i + 1) * (j + 1) * item.value
		}
	}
	return result
}

func calculateResultA(input []string) int {
	result := 0
	for _, el := range input {
		result += HashFunction(el)
	}

	return result
}

func calculateResultB(input []string) int {
	boxes := make(map[int][]BoxItem)
	for _, el := range input {
		split := strings.Split(el, "=")
		var label string
		if len(split) == 2 {
			label = split[0]
			value, _ := strconv.Atoi(split[1])
			boxIndex := HashFunction(label)
			boxes[boxIndex] = AddItem(boxes[boxIndex], label, value)
		} else {
			label = strings.Split(el, "-")[0]
			boxIndex := HashFunction(label)
			boxes[boxIndex] = RemoveItem(boxes[boxIndex], label)
		}
		fmt.Println(boxes)
	}
	return FocusingPower(boxes)
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

	return strings.Split(input[0], ",")
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
