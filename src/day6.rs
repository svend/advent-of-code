#[derive(Clone, Debug, PartialEq)]
struct Point(i32, i32);

type Grid = Vec<Vec<Option<()>>>;

fn new_grid(width: usize, height: usize) -> Grid {
    vec![vec![None; height]; width]
}

fn grid_from_points(points: &[Point]) -> Grid {
    let max_x = points.iter().map(|Point(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|Point(_, y)| y).max().unwrap();
    let mut grid = new_grid(*max_x as usize + 1, *max_y as usize + 1);
    for point in points {
        grid_on(&mut grid, &point);
    }
    grid
}

fn print_grid(grid: &Grid) {
    let width = grid.first().map(|column| column.len()).unwrap_or(0);
    for column in 0..width {
        let row: String = grid
            .iter()
            .map(|c| c[column])
            .map(|c| match c {
                Some(_) => 'x',
                None => '.',
            })
            .collect();
        println!("{}", row)
    }
}

fn grid_points(grid: &Grid) -> Vec<Point> {
    let mut points = vec![];
    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            points.push(Point(x as i32, y as i32))
        }
    }
    points
}

fn occupied_points(grid: &Grid) -> Vec<Point> {
    let mut points = vec![];
    for (x, row) in grid.iter().enumerate() {
        for (y, column) in row.iter().enumerate() {
            if column.is_some() {
                points.push(Point(x as i32, y as i32))
            }
        }
    }
    points
}

fn grid_on(grid: &mut Grid, point: &Point) {
    let Point(x, y) = point;
    grid[*x as usize][*y as usize] = Some(())
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

fn outermost_points(points: &[Point]) -> Vec<&Point> {
    let mut outer = vec![];

    let mut xs: Vec<_> = points.iter().map(|Point(x, _)| x).collect();
    xs.sort();
    let xs = vec![xs.first().unwrap(), xs.last().unwrap()];

    let mut ys: Vec<_> = points.iter().map(|Point(_, y)| y).collect();
    ys.sort();
    let ys = vec![ys.first().unwrap(), ys.last().unwrap()];

    for point in points {
        let Point(x, y) = point;
        if xs.contains(&&x) || ys.contains(&&y) {
            outer.push(point)
        }
    }
    outer
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
    let grid = grid_from_points(&points);

    let all_points = grid_points(&grid);
    let occupied = occupied_points(&grid);
    let outermost = outermost_points(&occupied);

    let closest: Vec<_> = all_points
        .iter()
        .map(|p| closest_point(p, &occupied))
        .filter_map(|x| x)
        .filter(|p| !outermost.contains(&p))
        .collect();

    closest
        .iter()
        .map(|p1| closest.iter().filter(|&p2| p1 == p2).count())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(get_max_area(&points), 6278);
    }

    #[test]
    fn test_outermost_points() {
        let input = include_str!("6.example.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(
            outermost_points(&points),
            vec![&Point(1, 1), &Point(1, 6), &Point(8, 3), &Point(8, 9)]
        );

        let input = include_str!("6.input");
        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);
        assert_eq!(
            outermost_points(&points),
            vec![
                &Point(359, 177),
                &Point(101, 47),
                &Point(147, 351),
                &Point(54, 122)
            ]
        );
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point(0, 0);
        let p2 = Point(6, 6);
        assert_eq!(manhattan_distance(&p1, &p2), 12);
        assert_eq!(manhattan_distance(&p2, &p1), 12);
    }
}
