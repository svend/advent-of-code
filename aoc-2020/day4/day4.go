package day4

import (
	"bufio"
	"io"
	"regexp"
	"strconv"
	"strings"
)

const HairColorRE = "^#[a-f0-9][a-f0-9][a-f0-9][a-f0-9][a-f0-9][a-f0-9]$"

type Passport struct {
	BirthYear      string
	IssueYear      string
	ExpirationYear string
	Height         string
	HairColor      string
	EyeColor       string
	PassportID     string
	CountryID      string
}

func (p *Passport) SetField(k, v string) {
	switch k {
	case "byr":
		p.BirthYear = v
	case "iyr":
		p.IssueYear = v
	case "eyr":
		p.ExpirationYear = v
	case "hgt":
		p.Height = v
	case "hcl":
		p.HairColor = v
	case "ecl":
		p.EyeColor = v
	case "pid":
		p.PassportID = v
	case "cid":
		p.CountryID = v
	default:
		panic("invalid field key")
	}
}

func (p Passport) Valid() bool {
	return p.BirthYear != "" && p.IssueYear != "" && p.ExpirationYear != "" && p.Height != "" && p.HairColor != "" && p.EyeColor != "" && p.PassportID != ""
}

func (p Passport) ValidStrict() bool {
	if len(p.BirthYear) != 4 {
		return false
	}
	birthYear, _ := strconv.Atoi(p.BirthYear)
	if birthYear < 1920 || birthYear > 2002 {
		return false
	}

	if len(p.IssueYear) != 4 {
		return false
	}
	issueYear, _ := strconv.Atoi(p.IssueYear)
	if issueYear < 2010 || issueYear > 2020 {
		return false
	}

	if len(p.ExpirationYear) != 4 {
		return false
	}
	expirationYear, _ := strconv.Atoi(p.ExpirationYear)
	if expirationYear < 2020 || expirationYear > 2030 {
		return false
	}

	var heightUnit string
	if strings.HasSuffix(p.Height, "cm") {
		heightUnit = "cm"
	} else if strings.HasSuffix(p.Height, "in") {
		heightUnit = "in"
	} else {
		return false
	}
	height, _ := strconv.Atoi(strings.TrimSuffix(p.Height, heightUnit))
	if heightUnit == "cm" && !(150 <= height && height <= 193) {
		return false
	}
	if heightUnit == "in" && !(59 <= height && height <= 76) {
		return false
	}

	if !regexp.MustCompile(HairColorRE).MatchString(p.HairColor) {
		return false
	}

	if p.EyeColor != "amb" &&
		p.EyeColor != "blu" &&
		p.EyeColor != "brn" &&
		p.EyeColor != "gry" &&
		p.EyeColor != "grn" &&
		p.EyeColor != "hzl" &&
		p.EyeColor != "oth" {
		return false
	}

	if len(p.PassportID) != 9 {
		return false
	}
	if _, err := strconv.Atoi(p.PassportID); err != nil {
		return false
	}

	return p.Height != ""
}

func ParsePassports(r io.Reader) []Passport {
	scanner := bufio.NewScanner(r)
	var passports []Passport
	var passport Passport
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			passports = append(passports, passport)
			passport = Passport{}
			continue
		}
		fields := strings.Split(line, " ")
		for _, field := range fields {
			fieldKV := strings.SplitN(field, ":", 2)
			if len(fieldKV) != 2 {
				panic("invalid number of field parts")
			}
			passport.SetField(fieldKV[0], fieldKV[1])
		}
	}
	passports = append(passports, passport)
	return passports
}

// Part1 returns the number of valid passports in the input
func Part1(r io.Reader) int {
	passports := ParsePassports(r)

	var numValid int
	for _, passport := range passports {
		if passport.Valid() {
			numValid++
		}
	}

	return numValid
}

// Part1 returns the number of valid passports in the input using stricter
// checks
func Part2(r io.Reader) int {
	passports := ParsePassports(r)

	var numValid int
	for _, passport := range passports {
		if passport.ValidStrict() {
			numValid++
		}
	}

	return numValid
}
