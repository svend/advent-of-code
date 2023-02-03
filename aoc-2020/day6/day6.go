package day6

import (
	"bufio"
	"io"
)

func Part1(r io.Reader) int {
	sum := 0

	scanner := bufio.NewScanner(r)
	answers := make(map[rune]bool)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			sum += len(answers)
			answers = make(map[rune]bool)
			continue
		}

		for _, c := range line {
			answers[c] = true
		}
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	// Add the last group
	sum += len(answers)

	return sum
}

func Part2(r io.Reader) int {
	sum := 0
	person := 0

	scanner := bufio.NewScanner(r)
	answers := make(map[rune]int)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			for _, count := range answers {
				if count == person {
					sum++
				}
			}
			person = 0
			answers = make(map[rune]int)
			continue
		}

		for _, c := range line {
			answers[c]++
		}

		person++
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	// Add the last group
	for _, count := range answers {
		if count == person {
			sum++
		}
	}

	return sum
}
