package day6

import (
	"bytes"
	"io"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func readFile(filename string) io.Reader {
	data, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	return bytes.NewReader(data)
}

func TestPart1(t *testing.T) {
	assert.Equal(t, 11, Part1(readFile("example.txt")))
	assert.Equal(t, 6297, Part1(readFile("input.txt")))
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 6, Part2(readFile("example.txt")))
	assert.Equal(t, 3158, Part2(readFile("input.txt")))
}
