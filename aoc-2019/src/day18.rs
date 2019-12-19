// https://adventofcode.com/2019/day/18

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum State {
    Empty,
    Start,
    Key(char),
    Door(char),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

#[derive(Debug)]
struct Graph(Vec<(Point, State)>);

impl Graph {
    fn new(input: &str) -> Self {
        let points: Vec<(Point, State)> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_x, c)| *c != '#')
                    .map(move |(x, _c)| (Point(x, y), State::Empty))
                    .collect::<Vec<_>>()
            })
            .collect();
        Self(points)
    }

    fn contains(&self, point: &Point) -> bool {
        self.points().contains(point)
    }

    fn points(&self) -> HashSet<&Point> {
        self.0.iter().map(|(point, _state)| point).collect()
    }

    fn max(&self) -> Point {
        let max_x = self.points().iter().map(|Point(x, _y)| *x).max().unwrap();
        let max_y = self.points().iter().map(|Point(_x, y)| *y).max().unwrap();
        Point(max_x, max_y)
    }

    fn draw(&self) {
        let Point(max_x, max_y) = self.max();
        for y in 0..=(max_y + 1) {
            for x in 0..=(max_x + 1) {
                if self.contains(&Point(x, y)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let input = include_str!("day18.input");
        let graph = Graph::new(&input);
        graph.draw();
        todo!();
    }
}
