package day5

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

func TestParseRegions(t *testing.T) {
	assert.Equal(t,
		ParseRegions("FBFBBFFRLR"),
		[]Region{Front, Back, Front, Back, Back, Front, Front, Right, Left, Right})
}

func TestSeatID(t *testing.T) {
	cases := []struct {
		row, col int
		want     int
	}{
		{44, 5, 357},
		{70, 7, 567},
		{14, 7, 119},
		{102, 4, 820},
	}

	for _, c := range cases {
		got := SeatID(c.row, c.col)
		if got != c.want {
			t.Errorf("want %v, got %v", c.want, got)
		}
	}
}

func TestRange(t *testing.T) {
	cases := []struct {
		first, last         int
		lowerHalf           bool
		wantFirst, wantLast int
	}{
		{0, 127, true, 0, 63},
		{0, 63, false, 32, 63},
		{32, 63, true, 32, 47},
		{32, 47, false, 40, 47},
		{40, 47, false, 44, 47},
		{44, 47, true, 44, 45},
		{44, 45, true, 44, 44},
	}

	for _, c := range cases {
		gotFirst, gotLast := Range(c.first, c.last, c.lowerHalf)
		if gotFirst != c.wantFirst {
			t.Errorf("want %v, got %v", c.wantFirst, gotFirst)
		}
		if gotLast != c.wantLast {
			t.Errorf("want %v, got %v", c.wantLast, gotLast)
		}
	}
}

func TestGetRowOrCol(t *testing.T) {
	cases := []struct {
		max  int
		rs   []Region
		want int
	}{
		{128, ParseRegions("FBFBBFF"), 44},
		{8, ParseRegions("RLR"), 5},
	}

	for _, c := range cases {
		got := GetRowOrCol(c.max, c.rs)
		if got != c.want {
			t.Errorf("want %v, got %v", c.want, got)
		}
	}
}

func TestGetSeatID(t *testing.T) {
	cases := []struct {
		rs   []Region
		want int
	}{
		{ParseRegions("FBFBBFFRLR"), 357},
		{ParseRegions("BFFFBBFRRR"), 567},
		{ParseRegions("BBFFBBFRLL"), 820},
	}

	for _, c := range cases {
		got := GetSeatID(c.rs)
		if got != c.want {
			t.Errorf("want %v, got %v", c.want, got)
		}
	}
}

func TestGetMaxSeatID(t *testing.T) {
	cases := []struct {
		rss  [][]Region
		want int
	}{
		{
			[][]Region{ParseRegions("FBFBBFFRLR"), ParseRegions("BFFFBBFRRR"), ParseRegions("BBFFBBFRLL")},
			820,
		},
	}

	for _, c := range cases {
		got := GetMaxSeatID(c.rss)
		if got != c.want {
			t.Errorf("want %v, got %v", c.want, got)
		}
	}
}

func TestPart1(t *testing.T) {
	assert.Equal(t, Part1(readFile("input.txt")), 965)
}

func TestPart2(t *testing.T) {
	assert.Equal(t, Part2(readFile("input.txt")), 524)
}
