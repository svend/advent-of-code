// https://adventofcode.com/2019/day/3

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

/// A direction on the graph
#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// A point on the graph.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn shift(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
            Direction::Up => Point(self.0, self.1 + 1),
            Direction::Down => Point(self.0, self.1 - 1),
        }
    }
}

/// Calculate the Manhattan distance between two points.
fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

/// A change in position.
#[derive(Debug, PartialEq)]
struct Shift {
    direction: Direction,
    length: usize,
}

impl FromStr for Shift {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, length) = s.split_at(1);
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            c => panic!("bad direction: {}", c),
        };
        let length = length.parse::<usize>()?;
        Ok(Shift { direction, length })
    }
}

fn points_along_shift(start: &Point, m: &Shift) -> Vec<Point> {
    let mut points = vec![];
    let mut position = start.clone();
    for _ in 0..m.length {
        position = position.shift(&m.direction);
        points.push(position.clone())
    }
    points
}

fn points_along_path(start: &Point, path: &[Shift]) -> Vec<Point> {
    let mut all_points: Vec<Point> = vec![];
    let mut position = start.clone();
    for shift in path {
        let mut points = points_along_shift(&position, &shift);
        if let Some(point) = points.last() {
            position = point.clone()
        }
        all_points.append(&mut points)
    }
    all_points
}

fn common_points(paths: Vec<Vec<Shift>>) -> HashSet<Point> {
    let mut point_sets = paths
        .iter()
        .map(|path| points_along_path(&Point(0, 0), &path).into_iter().collect());
    let mut common: HashSet<Point> = HashSet::new();
    if let Some(points) = point_sets.next() {
        common = points;
        for points in point_sets {
            common = common.intersection(&points).cloned().collect();
        }
    }
    common
}

fn min_distance(paths: Vec<Vec<Shift>>) -> Option<i32> {
    let common = common_points(paths);
    common
        .into_iter()
        .map(|point| manhattan_distance(&Point(0, 0), &point))
        .min()
}

fn common_points_with_steps(paths: Vec<Vec<Shift>>) -> HashMap<Point, usize> {
    let mut steps: HashMap<Point, usize> = HashMap::new();
    for path in &paths {
        let points = points_along_path(&Point(0, 0), &path);
        for (step, point) in points.iter().enumerate() {
            let s = steps.entry(point.clone()).or_insert(0);
            // the points are zero-indexed, so add one
            *s += step + 1;
        }
    }
    let common = common_points(paths);
    steps.retain(|point, _steps| common.contains(&point));
    steps
}

fn min_steps(paths: Vec<Vec<Shift>>) -> Option<usize> {
    let common = common_points_with_steps(paths);
    common.into_iter().map(|(_point, step)| step).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input(input: &str) -> Vec<Vec<Shift>> {
        input
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<Shift>().unwrap())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_manhattan_distance() {
        let tests = &[
            ((Point(0, 0), Point(0, 0)), 0),
            ((Point(1, 1), Point(0, 0)), 2),
            ((Point(10, 10), Point(1, 1)), 18),
        ];
        for (input, expected) in tests {
            assert_eq!(&manhattan_distance(&input.0, &input.1), expected);
        }
    }

    #[test]
    fn test_min_distance() {
        let tests = &[
            ("R8,U5,L5,D3\nU7,R6,D4,L4", Some(6)),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                Some(159),
            ),
            (include_str!("day3.input"), Some(1264)),
        ];
        for (input, expected) in tests {
            let input = parse_input(input);
            assert_eq!(&min_distance(input), expected);
        }
    }

    #[test]
    fn test_min_steps() {
        let tests = &[
            ("R8,U5,L5,D3\nU7,R6,D4,L4", Some(30)),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                Some(610),
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                Some(410),
            ),
            (include_str!("day3.input"), Some(37390)),
        ];
        for (input, expected) in tests {
            let input = parse_input(input);
            assert_eq!(&min_steps(input), expected);
        }
    }
}
