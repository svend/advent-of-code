package day1

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"testing"
)

func day1part1input() []int {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var expenses []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		i, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		expenses = append(expenses, i)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return expenses
}

func TestDay1Part1(t *testing.T) {
	tts := []struct {
		input    []int
		expected int
	}{
		{
			[]int{1721, 979, 366, 299, 675, 1456},
			514579,
		},
		{
			day1part1input(),
			1006875,
		},
	}

	for _, tt := range tts {
		actual, err := day1part1(tt.input)
		if err != nil {
			t.Fatal(err)
		}
		if actual != tt.expected {
			t.Errorf("expected %d, actual %d", tt.expected, actual)
		}
	}
}

func TestDay1Part2(t *testing.T) {
	tts := []struct {
		input    []int
		expected int
	}{
		{
			[]int{1721, 979, 366, 299, 675, 1456},
			241861950,
		},
		{
			day1part1input(),
			165026160,
		},
	}

	for _, tt := range tts {
		actual, err := day1part2(tt.input)
		if err != nil {
			t.Fatal(err)
		}
		if actual != tt.expected {
			t.Errorf("expected %d, actual %d", tt.expected, actual)
		}
	}
}
