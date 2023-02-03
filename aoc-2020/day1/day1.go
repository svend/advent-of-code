package day1

import "errors"

func day1part1(expenses []int) (int, error) {
	for i := range expenses {
		for j := i + 1; j < len(expenses); j++ {
			if i == j {
				continue
			}
			if expenses[i]+expenses[j] == 2020 {
				return expenses[i] * expenses[j], nil
			}
		}
	}
	return 0, errors.New("no answer")
}

func day1part2(expenses []int) (int, error) {
	for i := range expenses {
		for j := i + 1; j < len(expenses); j++ {
			for k := j + 1; k < len(expenses); k++ {
				if expenses[i]+expenses[j]+expenses[k] == 2020 {
					return expenses[i] * expenses[j] * expenses[k], nil
				}
			}
		}
	}
	return 0, errors.New("no answer")
}
