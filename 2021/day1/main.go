package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {

	file, err := os.Open("input.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}
	part1(file)

	file, err = os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	part2(file)
}

func parseInt(line []byte) (int, error) {
	return strconv.Atoi(string(line[:len(line)-1]))
}

func part2(file *os.File) {
	reader := bufio.NewReader(file)

	win := make([]int, 4)
	for i := 0; i < 4; i++ {
		line, err := reader.ReadSlice('\n')
		if err != nil {
			panic(err)
		}
		win[i], err = parseInt(line)
	}

	prevTotal := win[0] + win[1] + win[2]
	currTotal := win[1] + win[2] + win[3]

	var increases int
	if prevTotal < currTotal {
		increases = 1
	}

	line, err := reader.ReadSlice('\n')
	for err == nil {
		var depth int
		depth, err = parseInt(line)
		if err != nil {
			panic(err)
		}

		// prev = [0, 1, 2]
		prevTotal = prevTotal - win[0] + win[3]

		// curr = [1, 2, 3]
		currTotal = currTotal - win[1] + depth

		if prevTotal < currTotal {
			increases++
		}

		win = append(win[1:], depth)

		line, err = reader.ReadSlice('\n')
	}

	fmt.Println("Part 2 Answer: ", increases)

}

func part1(file *os.File) {
	reader := bufio.NewReader(file)

	var prev, curr, increases int
	line, err := reader.ReadSlice('\n')
	for err == nil {
		curr, err = parseInt(line)
		if err != nil {
			panic(err)
		}

		if prev != 0 && prev < curr {
			increases++
		}

		prev = curr
		line, err = reader.ReadSlice('\n')
	}

	fmt.Println("Part 1 Answer: ", increases)
}
