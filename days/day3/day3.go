package main

import (
	"regexp"
	"strconv"
)

type part struct {
	location
}

type location struct {
	x int
	y int
}

type numberInfo struct {
	location location
	number   int
}

func partOne(input []string) int {
	numbers, parts := parseInput(input)
	total := 0
	for _, number := range numbers {
		if numberHasPart(parts, number.location, len(strconv.Itoa(number.number))) {
			total += number.number
		}
	}
	return total
}

func partTwo(input []string) int {
	numberInfos, parts := parseInput(input)
	partNumbers := make(map[part][]int)
	for _, numInfo := range numberInfos {
		p := partForNumber(parts, numInfo.location, len(strconv.Itoa(numInfo.number)))
		if p.x != 0 && p.y != 0 {
			partNumbers[p] = append(partNumbers[p], numInfo.number)
		}
	}

	total := 0
	for _, nums := range partNumbers {
		if len(nums) >= 2 {
			total += nums[0] * nums[1]
		}
	}
	return total
}

func parseInput(input []string) ([]numberInfo, []part) {
	numbers := make([]numberInfo, 0)
	parts := make([]part, 0)

	re := regexp.MustCompile(`\d+`)
	for y, line := range input {
		for x := 0; x < len(line); x++ {
			if isNumeric(string(line[x])) {
				matches := re.FindStringSubmatch(line[x:])
				if len(matches) > 0 {
					number, _ := strconv.Atoi(matches[0])
					numbers = append(numbers, numberInfo{location: location{x: x, y: y}, number: number})
					x += len(matches[0]) - 1
				}
			} else if line[x] != '.' {
				parts = append(parts, part{location: location{x: x, y: y}})
			}
		}
	}
	return numbers, parts
}

func numberHasPart(parts []part, loc location, numberLength int) bool {
	if contains(parts, location{loc.x - 1, loc.y}) || contains(parts, location{loc.x + numberLength, loc.y}) {
		return true
	}

	for i := -1; i < numberLength+1; i++ {
		if contains(parts, location{loc.x + i, loc.y - 1}) || contains(parts, location{loc.x + i, loc.y + 1}) {
			return true
		}
	}
	return false
}

func partForNumber(parts []part, loc location, numberLength int) part {
	if contains(parts, location{loc.x - 1, loc.y}) {
		return part{location{loc.x - 1, loc.y}}
	}
	if contains(parts, location{loc.x + numberLength, loc.y}) {
		return part{location{loc.x + numberLength, loc.y}}
	}
	for i := -1; i <= numberLength; i++ {
		if contains(parts, location{loc.x + i, loc.y - 1}) {
			return part{location{loc.x + i, loc.y - 1}}
		}
		if contains(parts, location{loc.x + i, loc.y + 1}) {
			return part{location{loc.x + i, loc.y + 1}}
		}
	}
	return part{}
}

func isNumeric(s string) bool {
	_, err := strconv.Atoi(s)
	return err == nil
}

func contains(parts []part, loc location) bool {
	for _, v := range parts {
		if loc.x == v.x && loc.y == v.y {
			return true
		}
	}
	return false
}
