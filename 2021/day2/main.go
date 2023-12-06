package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	FORWARD = iota
	DOWN
	UP
)

func main() {
	file, err := os.Open("input.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}
	part1(file)

	file, err = os.Open("input.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}
	part2(file)

}

func part2(file *os.File) {
	reader := bufio.NewReader(file)

	var position, depth, aim int

	line, err := reader.ReadSlice('\n')
	for err == nil {
		action, val := parseAction(line)

		switch action {
		case FORWARD:
			position += val
			depth += aim * val
		case DOWN:
			aim += val
		case UP:
			aim -= val
		}

		line, err = reader.ReadSlice('\n')
	}
	fmt.Println("Position: ", position)
	fmt.Println("Depth: ", depth)
	fmt.Println("Part 2 Answer: ", position*depth)
}

func parseAction(line []byte) (int, int) {
	command := strings.Split(string(line[:len(line)-1]), " ")

	var action int
	switch command[0] {
	case "forward":
		action = FORWARD
	case "down":
		action = DOWN
	case "up":
		action = UP
	}

	val, err := strconv.Atoi(command[1])
	if err != nil {
		panic(err)
	}

	return action, val
}

func part1(file *os.File) {
	reader := bufio.NewReader(file)

	var position, depth int

	line, err := reader.ReadSlice('\n')
	for err == nil {
		action, val := parseAction(line)

		switch action {
		case FORWARD:
			position += val
		case DOWN:
			depth += val
		case UP:
			depth -= val
		}

		line, err = reader.ReadSlice('\n')
	}
	fmt.Println("Position: ", position)
	fmt.Println("Depth: ", depth)
	fmt.Println("Part 1 Answer: ", position*depth)
}
