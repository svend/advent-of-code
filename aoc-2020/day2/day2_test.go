package day2

import (
	"io"
	"os"
	"strings"
	"testing"
)

func TestParseEntry(t *testing.T) {
	input := "1-3 a: abcde"
	expected := PasswordEntry{PasswordPolicy{1, 3, 'a'}, "abcde"}
	actual, err := parseEntry(input)
	if err != nil {
		t.Fatal(err)
	}
	if actual != expected {
		t.Errorf("expected %v, actual %v", expected, actual)
	}
}

func TestPasswordEntryValid(t *testing.T) {
	tts := []struct {
		input    PasswordEntry
		expected bool
	}{
		{
			PasswordEntry{PasswordPolicy{1, 3, 'a'}, "abcde"},
			true,
		},
		{
			PasswordEntry{PasswordPolicy{1, 3, 'b'}, "cdefg"},
			false,
		},
	}

	for _, tt := range tts {
		actual := tt.input.valid()
		if actual != tt.expected {
			t.Errorf("expected %v, actual %v", tt.expected, actual)
		}
	}
}

func TestDay2Part1(t *testing.T) {
	f, err := os.Open("input.txt")
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close()

	tts := []struct {
		input    io.Reader
		expected int
	}{
		{
			strings.NewReader("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"),
			2,
		},
		{
			f,
			460,
		},
	}

	for _, tt := range tts {
		entries, err := parseEntries(tt.input)
		if err != nil {
			t.Fatal(err)
		}
		actual := day2part1(entries)
		if actual != tt.expected {
			t.Errorf("actual %d, expected %d", actual, tt.expected)
		}
	}
}

func TestDay2Part2(t *testing.T) {
	f, err := os.Open("input.txt")
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close()

	tts := []struct {
		input    io.Reader
		expected int
	}{
		{
			strings.NewReader("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"),
			1,
		},
		{
			f,
			251,
		},
	}

	for _, tt := range tts {
		entries, err := parseEntries(tt.input)
		if err != nil {
			t.Fatal(err)
		}
		actual := day2part2(entries)
		if actual != tt.expected {
			t.Errorf("actual %d, expected %d", actual, tt.expected)
		}
	}
}
