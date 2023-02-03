package day4

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
	assert.Equal(t, Part1(readFile("example.txt")), 2)
	assert.Equal(t, Part1(readFile("input.txt")), 242)
}

func TestPart2(t *testing.T) {
	assert.Equal(t, Part2(readFile("example-invalid.txt")), 0)
	assert.Equal(t, Part2(readFile("example-valid.txt")), 4)
	assert.Equal(t, Part2(readFile("input.txt")), 186)
}
