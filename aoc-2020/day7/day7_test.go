package day7

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
	assert.Equal(t, Part1(readFile("example.txt")), 4)
}
