package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var file string

func calculateResultA(input []int) int {
	result := 0

	// last file that still has blocks to move
	last := len(input) - 1
	index := 0
	for i, el := range input {
		// when even just add to the checksum
		if i%2 == 0 {
			for j := 0; j < el; j++ {
				result += index * (i / 2)
				index++
			}
		} else {
			// when odd grab the elements from the back
			for j := 0; j < el; j++ {
				if input[last] < 1 {
					last -= 2
				}
				input[last]--
				result += index * (last / 2)
				index++
			}
		}
		if i >= last {
			return result
		}
	}

	return result
}

func calculateResultB(input []int) int {
	result := 0

	workingCopy := make([]int, len(input))
	copy(workingCopy, input)

	// empty -> filling with
	fillMap := make(map[int][]int)
	// moved elements
	movedMap := make(map[int]bool)
	// move blocks from back to front and reduce checksum accordingly
	for i := len(workingCopy) - 1; i > 1; i -= 2 {
		// check empty blocks from the front
		for j := 1; j < i; j += 2 {
			if j%2 != 0 && j < i {
				// if the file fits into empty block
				if i == 4 {
					fmt.Printf("j: %v - size of i: %v - size of j: %v\n %v \n\n", j, workingCopy[i], workingCopy[j], workingCopy)
				}
				if input[i] <= workingCopy[j] {
					// reduce space in empty block
					workingCopy[j] -= input[i]
					// add numbers to filled spot before to keep index in sync
					workingCopy[j-1] += input[i]
					// remove file that was moved
					workingCopy[i] -= input[i]
					fillMap[j] = append(fillMap[j], i)
					movedMap[i] = true
					break
				}
			}
		}
	}

	// Now calculate checksum using the movemap when hitting an empty block
	index := 0
	for i, el := range input {
		// when even just add to the checksum
		if i%2 == 0 {
			for j := 0; j < el; j++ {
				if !movedMap[i] {
					result += index * (i / 2)
				}
				index++
			}
		} else {
			// when odd check the moveMap and use the files otherwise only increase index
			back := fillMap[i]
			fillers := []int{}
			for _, el := range back {
				amount := input[el]
				for j := 0; j < amount; j++ {
					fillers = append(fillers, el/2)
				}
			}
			for j := 0; j < el; j++ {
				if j < len(fillers) {
					result += index * (fillers[j])
				}
				index++
			}
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

func getInput() []int {
	input := []int{}

	if file == "" {
		file = "input.txt"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		for _, el := range line {
			size, _ := strconv.Atoi(string(el))
			input = append(input, size)
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
