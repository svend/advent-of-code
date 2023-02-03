package day7

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

type Rule map[string]int
type RuleSet map[string]Rule

func singular(s string) string {
	if strings.HasSuffix(s, "s") {
		return s[:len(s)-1]
	}
	return s
}

func parseRule(line string) (string, Rule) {
	parts := strings.SplitN(line, " contain ", 2)
	bag := singular(parts[0])
	rule := make(Rule)

	if parts[1] == "no other bags." {
		return bag, nil
	}

	contents := strings.Split(parts[1], ", ")
	for i := range contents {
		c := contents[i]
		// Remove trailing period
		if strings.HasSuffix(c, ".") {
			c = c[:len(c)-1]
		}
		c = singular(c)
		contentParts := strings.SplitN(c, " ", 2)
		num, err := strconv.Atoi(contentParts[0])
		if err != nil {
			panic(err)
		}
		rule[contentParts[1]] = num
	}

	return bag, rule
}

func parseRules(r io.Reader) RuleSet {
	rules := make(RuleSet)

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		line := scanner.Text()
		bag, contents := parseRule(line)
		rules[bag] = contents
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return rules
}

func Part1(r io.Reader) int {

	rules := parseRules(r)
	bagType := "shiny gold bag"

	for bag, rule := range rules {
		fmt.Println(bag, rule)
	}

	count := countBags(rules, bagType, "", 0)
	fmt.Println(count)

	panic("unimplemented")
}

func countBags(rs RuleSet, targetBag, bag string, count int) int {
	fmt.Println("looking for", targetBag, bag, count)
	for b, rule := range rs {
		delete(rs, b)
		if b == bag {
			fmt.Println("found bag", bag)
			continue
		}
		for b2 := range rule {
			if b2 == targetBag {
				count++
				continue
			}
			count += countBags(rs, targetBag, b2, count)
		}
	}
	return count
}
