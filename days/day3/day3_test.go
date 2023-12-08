package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	testCases := map[string]struct {
		input    string
		expected int
	}{
		"left": {
			input:    `....55#...`,
			expected: 55,
		},
		"right": {
			input:    `...#55....`,
			expected: 55,
		},
		"up": {
			input: `.....55...
......*...`,
			expected: 55,
		},
		"down": {
			input: `......*...
......55..`,
			expected: 55,
		},
		"diagonal up left": {
			input: `......*...
.......55.`,
			expected: 55,
		},
		"diagonal up right": {
			input: `........*.
......55..`,
			expected: 55,
		},
		"diagonal down left": {
			input: `......55..
.....*....`,
			expected: 55,
		},
		"diagonal down right": {
			input: `.....55...
.......*..`,
			expected: 55,
		},
		"sample input": {
			input: `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`,
			expected: 4361,
		},
	}

	for name, tc := range testCases {
		t.Run(name, func(t *testing.T) {
			actual := partOne(strings.Split(tc.input, "\n"))
			if actual != tc.expected {
				t.Errorf("Expected %d, got %d", tc.expected, actual)
			}
		})
	}
}
