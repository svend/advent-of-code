package day3

import (
	"bytes"
	"io"
	"io/ioutil"
	"testing"

	"github.com/stretchr/testify/assert"
)

func readFile(filename string) io.Reader {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	return bytes.NewReader(data)
}

func TestPart1(t *testing.T) {
	assert.Equal(t, Part1(readFile("example.txt")), 7)
	assert.Equal(t, Part1(readFile("input.txt")), 299)
}

func TestPart2(t *testing.T) {
	assert.Equal(t, Part2(readFile("example.txt")), 336)
	assert.Equal(t, Part2(readFile("input.txt")), 3621285278)
}
