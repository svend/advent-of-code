// https://adventofcode.com/2019/day/10

use num::rational::Rational;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops;

/// Quadrant represent the four quadrants, starting with the top-right, and
/// going clock-wise
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Quadrant {
    /// top-right
    One,
    /// bottom-right
    Two,
    /// bottom-left
    Three,
    /// top-left
    Four,
}

/// Angle stores an angle as a quadrant, and a grade
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Angle {
    quadrant: Quadrant,
    grade: Rational,
}

/// A point represents X, Y coordinates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Point {
    fn quadrant(&self) -> Quadrant {
        match self {
            Point(x, y) if *x >= 0 && *y < 0 => Quadrant::One,
            Point(x, y) if *x > 0 && *y >= 0 => Quadrant::Two,
            Point(x, y) if *x <= 0 && *y > 0 => Quadrant::Three,
            Point(x, y) if *x < 0 && *y <= 0 => Quadrant::Four,
            _ => panic!("point has no quadrant"),
        }
    }

    fn angle(&self) -> Angle {
        let quadrant = self.quadrant();
        let (rise, run) = match quadrant {
            Quadrant::One | Quadrant::Three => (self.0, self.1),
            Quadrant::Two | Quadrant::Four => (self.1, self.0),
        };
        let grade = Rational::new(rise.abs(), run.abs());
        Angle { quadrant, grade }
    }

    fn angle_to(&self, other: &Point) -> Angle {
        let point = other.clone() - self.clone();
        point.angle()
    }

    fn sq_dist_to(&self, other: &Point) -> isize {
        let point = other.clone() - self.clone();
        point.0 * point.0 + point.1 * point.1
    }
}

/// A map is a list of points
#[derive(Debug)]
struct Map(Vec<Point>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_x = self.0.iter().map(|Point(x, _y)| *x).max().unwrap_or(0);
        let max_y = self.0.iter().map(|Point(_x, y)| *y).max().unwrap_or(0);
        let mut lines: Vec<String> = Vec::new();
        for y in 0..=max_y {
            let line: String = (0..=max_x)
                .map(|x| {
                    if self.0.contains(&Point(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            lines.push(line);
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl Map {
    fn parse(s: &str) -> Self {
        let points: Vec<Point> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(x, c)| {
                        if c == '#' {
                            Some(Point(x as isize, y as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Map(points)
    }

    fn angles(&self, other: &Point) -> HashSet<Angle> {
        self.0
            .iter()
            .filter(|point| *point != other)
            .map(|point| point.angle_to(other))
            .collect()
    }

    fn best_point(&self) -> Option<(Point, usize)> {
        self.0
            .iter()
            .map(|point| (point.clone(), self.angles(point).len()))
            .max_by_key(|point_num| point_num.1)
    }

    fn angle_distance(&self, other: &Point) -> Vec<(Point, Angle, isize)> {
        self.0
            .iter()
            .filter(|point| *point != other)
            .map(|point| {
                (
                    point.clone(),
                    other.angle_to(point),
                    point.sq_dist_to(other),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_1 = ".#..#
.....
#####
....#
...##";

        let input_2 = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let input_3 = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        let input_4 = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let input_5 = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let my_input = include_str!("day10.input").trim_end();

        let tests = &[
            (input_1, (Point(3, 4), 8)),
            (input_2, (Point(5, 8), 33)),
            (input_3, (Point(1, 2), 35)),
            (input_4, (Point(6, 3), 41)),
            (input_5, (Point(11, 13), 210)),
            (my_input, (Point(11, 13), 227)),
        ];
        for (input, expected) in tests {
            let map = Map::parse(input);
            assert_eq!(&format!("{}", map), input);
            assert_eq!(&map.best_point().unwrap(), expected);
        }
    }

    #[test]
    fn test_quadrant() {
        let mut input = vec![
            Quadrant::Four,
            Quadrant::Three,
            Quadrant::Two,
            Quadrant::One,
        ];
        let expected = vec![
            Quadrant::One,
            Quadrant::Two,
            Quadrant::Three,
            Quadrant::Four,
        ];
        input.sort();
        assert_eq!(input, expected);

        let tests = &[
            (Point(0, -1), Quadrant::One),
            (Point(1, -2), Quadrant::One),
            (Point(1, -1), Quadrant::One),
            (Point(2, -1), Quadrant::One),
            (Point(1, 0), Quadrant::Two),
            (Point(2, 1), Quadrant::Two),
            (Point(1, 1), Quadrant::Two),
            (Point(1, 2), Quadrant::Two),
            (Point(0, 1), Quadrant::Three),
            (Point(-1, 2), Quadrant::Three),
            (Point(-1, 1), Quadrant::Three),
            (Point(-2, 1), Quadrant::Three),
            (Point(-1, 0), Quadrant::Four),
            (Point(-2, -1), Quadrant::Four),
            (Point(-1, -1), Quadrant::Four),
            (Point(-1, -2), Quadrant::Four),
        ];
        for (input, expected) in tests {
            dbg!(input, expected);
            assert_eq!(&input.quadrant(), expected);
        }
    }

    #[test]
    fn test_angle() {
        let mut input = vec![
            Point(0, -1),
            Point(1, -2),
            Point(1, -1),
            Point(2, -1),
            Point(1, 0),
            Point(2, 1),
            Point(1, 1),
            Point(1, 2),
            Point(0, 1),
            Point(-1, 2),
            Point(-1, 1),
            Point(-2, 1),
            Point(-1, 0),
            Point(-2, -1),
            Point(-1, -1),
            Point(-1, -2),
        ];
        let expected = input.clone();
        input.reverse();
        input.sort_by_key(|point| point.angle());
        assert_eq!(input, expected);
    }

    #[test]
    fn test_vaporize() {
        let input_1 = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
 ..#.#.....#....##";

        let input_2 = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let my_input = include_str!("day10.input").trim_end();

        let tests = &[
            ((input_1, Point(8, 3)), None),
            ((input_2, Point(11, 13)), Some(Point(8, 2))),
            ((my_input, Point(11, 13)), Some(Point(6, 4))),
        ];
        for (input, expected) in tests {
            let (input, base) = input;
            let map = Map::parse(input);

            let mut points = map.0;
            points.sort_by_key(|point| base.sq_dist_to(point));

            let mut angle_points: HashMap<Angle, Vec<Point>> = HashMap::new();
            for point in &points {
                if point == base {
                    continue;
                }
                let angle = base.angle_to(point);
                let entry = angle_points.entry(angle).or_insert(Vec::new());
                (*entry).push(point.clone());
            }

            let mut angle_points: Vec<_> = angle_points.into_iter().map(|(k, v)| (k, v)).collect();
            angle_points.sort_by_key(|(angle, _points)| angle.clone());

            let point_lists: Vec<_> = angle_points
                .into_iter()
                .map(|(_angle, points)| points)
                .collect();

            let longest = point_lists.iter().map(|ps| ps.len()).max().unwrap_or(0);
            let mut ordered_points = Vec::new();
            for i in 0..longest {
                for ps in &point_lists {
                    if let Some(point) = ps.get(i) {
                        ordered_points.push(point);
                    }
                }
            }

            println!("{:?}", ordered_points);
            assert_eq!(ordered_points.get(200 - 1).cloned(), expected.as_ref());
        }
    }
}
