package day5

import (
	"bufio"
	"io"
)

type Region int

const (
	numRows = 128
	numCols = 8
)

const (
	Front = iota
	Back
	Left
	Right
)

func (r Region) String() string {
	switch r {
	case Front:
		return "F"
	case Back:
		return "B"
	case Left:
		return "L"
	case Right:
		return "R"
	default:
		panic("invalid region")
	}
}

func NewRegion(c rune) Region {
	switch c {
	case 'F':
		return Front
	case 'B':
		return Back
	case 'L':
		return Left
	case 'R':
		return Right
	default:
		panic("invalid char")
	}
}

func ParseRegions(rs string) []Region {
	var regions []Region
	for _, r := range rs {
		regions = append(regions, NewRegion(r))
	}
	return regions
}

func SeatID(row, col int) int {
	return row*8 + col
}

func Range(first, last int, lowerHalf bool) (int, int) {
	num := last - first + 1
	mid := first + num/2
	if lowerHalf {
		return first, (mid - 1)
	}
	return mid, last
}

func GetRowOrCol(count int, rs []Region) int {
	first := 0
	last := count - 1
	for _, r := range rs {
		lowerHalf := r == Front || r == Left
		first, last = Range(first, last, lowerHalf)
	}
	return first
}

func GetSeatID(rs []Region) int {
	row := GetRowOrCol(numRows, rs[:7])
	col := GetRowOrCol(numCols, rs[7:10])
	return SeatID(row, col)
}

func GetMaxSeatID(rss [][]Region) int {
	var max int
	for _, rs := range rss {
		if id := GetSeatID(rs); id > max {
			max = id
		}
	}
	return max
}

// Part1 returns the highest seat number
func Part1(r io.Reader) int {
	var rss [][]Region

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		rs := ParseRegions(scanner.Text())
		rss = append(rss, rs)
	}

	return GetMaxSeatID(rss)
}

// Part2 returns the missing seat ID
func Part2(r io.Reader) int {
	maxID := SeatID(numRows-1, numCols-1)
	seats := make(map[int]bool)

	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		rs := ParseRegions(scanner.Text())
		seats[GetSeatID(rs)] = true
	}

	for i := 0; i <= maxID; i++ {
		// Seat is missing but seat-1 and seat+1 are not
		if !seats[i] && seats[i-1] && seats[i+1] {
			return i
		}
	}

	return -1
}
