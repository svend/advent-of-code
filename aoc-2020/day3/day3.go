package day3

import (
	"bufio"
	"fmt"
	"io"
)

type GridPoint int

const (
	Tree = iota
	NoTree
)

func (p GridPoint) String() string {
	switch p {
	case Tree:
		return "#"
	case NoTree:
		return "."
	default:
		panic(fmt.Sprint("invalid gridpoint:", p.String()))
	}
}

func NewGridPoint(c rune) GridPoint {
	switch c {
	case '#':
		return Tree
	case '.':
		return NoTree
	default:
		panic(fmt.Sprint("invalid rune:", c))
	}
}

type Grid [][]GridPoint

func ParseGrid(r io.Reader) Grid {
	scanner := bufio.NewScanner(r)
	var grid Grid
	for scanner.Scan() {
		var row []GridPoint
		for _, c := range scanner.Text() {
			p := NewGridPoint(rune(c))
			row = append(row, p)
		}
		grid = append(grid, row)
	}
	return grid
}

func (g Grid) String() string {
	var s string
	for i := range g {
		for j := range g {
			s = s + fmt.Sprint(g[i][j])
		}
		if i < len(g)-1 {
			s = s + "\n"
		}
	}
	return s
}

func (g Grid) Get(x, y int) GridPoint {
	return g[y][x%len(g[0])]
}

func Trees(g Grid, dX, dY int) int {
	trees := 0
	for x, y := 0, 0; y < len(g); {
		point := g.Get(x, y)
		if point == Tree {
			trees++
		}
		x += dX
		y += dY
	}
	return trees
}

func Part1(r io.Reader) int {
	grid := ParseGrid(r)
	return Trees(grid, 3, 1)
}

func Part2(r io.Reader) int {
	grid := ParseGrid(r)
	slopes := [][]int{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}
	trees := 1
	for _, slope := range slopes {
		trees *= Trees(grid, slope[0], slope[1])
	}
	return trees
}
