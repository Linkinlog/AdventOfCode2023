package main

import (
	"bufio"
	"os"
)

func main() {
	lines, err := readFileLines("input")
	if err != nil {
		panic(err)
	}

	println(partOne(lines))
	println(partTwo(lines))

}

func readFileLines(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return lines, nil
}
