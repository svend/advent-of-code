use std::fmt;

#[derive(Clone, Debug, PartialEq)]
struct Point(i32, i32);

struct Grid(Vec<Vec<Option<()>>>);

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid(vec![vec![None; height]; width])
    }

    fn from_points(points: &[Point]) -> Self {
        let max_x = points.iter().map(|Point(x, _)| x).max().unwrap();
        let max_y = points.iter().map(|Point(_, y)| y).max().unwrap();
        let mut grid = Self::new(*max_x as usize + 1, *max_y as usize + 1);
        for point in points {
            Self::on(&mut grid, &point);
        }
        grid
    }

    fn on(&mut self, point: &Point) {
        let Point(x, y) = point;
        self.0[*x as usize][*y as usize] = Some(())
    }

    fn height(&self) -> usize {
        self.0.first().map(|column| column.len()).unwrap_or(0)
    }

    fn width(&self) -> usize {
        self.0.len()
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = vec![];
        for (x, row) in self.0.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                points.push(Point(x as i32, y as i32))
            }
        }
        points
    }

    fn border_points(&self) -> Vec<Point> {
        let points = self.all_points();
        points
            .into_iter()
            .filter(|Point(x, y)| {
                *x == 0
                    || *x == (self.width() as i32 - 1)
                    || *y == 0
                    || *y == (self.height() as i32 - 1)
            })
            .collect()
    }

    fn occupied_points(&self) -> Vec<Point> {
        let mut points = vec![];
        for (x, row) in self.0.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                if column.is_some() {
                    points.push(Point(x as i32, y as i32))
                }
            }
        }
        points
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for column in 0..self.height() {
            let row: String = self
                .0
                .iter()
                .map(|c| c[column])
                .map(|c| match c {
                    Some(_) => 'X',
                    None => '.',
                })
                .collect();
            s.push_str("\n");
            s.push_str(&row);
        }
        write!(f, "{}", s)
    }
}

fn closest_point(p: &Point, ps: &[Point]) -> Option<Point> {
    let shortest = ps.iter().map(|p2| manhattan_distance(p, p2)).min().unwrap();
    let closest: Vec<_> = ps
        .into_iter()
        .filter(|p2| manhattan_distance(p, p2) == shortest)
        .collect();
    match closest.as_slice() {
        [p] => Some(p.clone().clone()),
        _ => None,
    }
}

fn parse_line(s: &str) -> Point {
    let s: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let xy: Vec<_> = s.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    match xy.as_slice() {
        [x, y] => Point(*x, *y),
        _ => panic!("bad input"),
    }
}

fn parse_lines(lines: &[&str]) -> Vec<Point> {
    lines.iter().map(|s| parse_line(s)).collect()
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    let Point(x1, y1) = p1;
    let Point(x2, y2) = p2;
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn get_max_area(points: &[Point]) -> usize {
    let grid = Grid::from_points(&points);

    let all_points = grid.all_points();
    let border_points = grid.border_points();
    let occupied = grid.occupied_points();

    let border_closest: Vec<_> = border_points
        .iter()
        .map(|p| closest_point(p, &occupied))
        .filter_map(|x| x)
        .collect();

    let closest: Vec<_> = all_points
        .iter()
        .map(|p| closest_point(p, &occupied))
        .filter_map(|x| x)
        .filter(|p| !border_closest.contains(p))
        .collect();

    closest
        .iter()
        .map(|p1| closest.iter().filter(|&p2| p1 == p2).count())
        .max()
        .unwrap()
}

fn get_max_area2(points: &[Point], less: i32) -> usize {
    let grid = Grid::from_points(&points);

    let all_points = grid.all_points();
    let occupied = grid.occupied_points();

    all_points
        .iter()
        .filter(|p1| {
            occupied
                .iter()
                .map(|p2| manhattan_distance(p1, p2))
                .sum::<i32>()
                < less
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point(0, 0);
        let p2 = Point(6, 6);
        assert_eq!(manhattan_distance(&p1, &p2), 12);
        assert_eq!(manhattan_distance(&p2, &p1), 12);
    }

    #[test]
    fn test_max_area() {
        let input = include_str!("6.example.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(get_max_area(&points), 17);

        let input = include_str!("6.example2.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(get_max_area(&points), 9);

        let input = include_str!("6.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(get_max_area(&points), 4475);
    }

    #[test]
    fn test_max_area2() {
        let input = include_str!("6.example.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(get_max_area2(&points, 32), 16);

        let input = include_str!("6.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(get_max_area2(&points, 10000), 35237);
    }
}
