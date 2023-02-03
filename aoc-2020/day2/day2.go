package day2

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type PasswordPolicy struct {
	min, max int
	c        rune
}

type PasswordEntry struct {
	policy   PasswordPolicy
	password string
}

func (p PasswordEntry) valid() bool {
	var count int
	for _, c := range p.password {
		if rune(c) == p.policy.c {
			count++
		}
	}
	return count >= p.policy.min && count <= p.policy.max
}

func (p PasswordEntry) valid2() bool {
	if len(p.password) < p.policy.min || len(p.password) < p.policy.max {
		return false
	}

	var count int
	if rune(p.password[p.policy.min-1]) == p.policy.c {
		count++
	}
	if rune(p.password[p.policy.max-1]) == p.policy.c {
		count++
	}

	return count == 1
}

func parseEntry(s string) (PasswordEntry, error) {
	parts := strings.SplitN(s, " ", 3)
	minMax := strings.SplitN(parts[0], "-", 2)
	min, err := strconv.Atoi(minMax[0])
	if err != nil {
		return PasswordEntry{}, fmt.Errorf("min: %w", err)
	}
	max, err := strconv.Atoi(minMax[1])
	if err != nil {
		return PasswordEntry{}, fmt.Errorf("min: %w", err)
	}
	cPart := parts[1]
	if len(cPart) != 2 || cPart[1] != ':' {
		return PasswordEntry{}, errors.New("invalid policy character")
	}
	policy := PasswordPolicy{
		min: min,
		max: max,
		c:   rune(cPart[0]),
	}
	return PasswordEntry{
		policy:   policy,
		password: parts[2],
	}, nil
}

func parseEntries(r io.Reader) ([]PasswordEntry, error) {
	scanner := bufio.NewScanner(r)
	var entries []PasswordEntry
	for scanner.Scan() {
		entry, err := parseEntry(scanner.Text())
		if err != nil {
			return nil, err
		}
		entries = append(entries, entry)
	}
	return entries, nil
}

func day2part1(entries []PasswordEntry) int {
	validCount := 0
	for _, entry := range entries {
		if entry.valid() {
			validCount++
		}
	}
	return validCount
}

func day2part2(entries []PasswordEntry) int {
	validCount := 0
	for _, entry := range entries {
		if entry.valid2() {
			validCount++
		}
	}
	return validCount
}
