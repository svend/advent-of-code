#[derive(Debug)]
struct Point(i32, i32);

type Grid = Vec<Vec<Option<()>>>;

fn new_grid(width: usize, height: usize) -> Grid {
    vec![vec![None; height]; width]
}

fn print_grid(grid: &Grid) {
    for row in grid {
        let row: String = row
            .iter()
            .map(|c| match c {
                Some(_) => 'x',
                None => '.',
            })
            .collect();
        println!("{}", row)
    }
}

fn grid_on(grid: &mut Grid, point: &Point) {
    let Point(x, y) = point;
    grid[*x as usize][*y as usize] = Some(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("6.example.input");

        let lines: Vec<_> = input.lines().collect();
        let points = parse_lines(&lines);

        let max_x = points.iter().map(|Point(x, _)| x).max().unwrap();
        let max_y = points.iter().map(|Point(_, y)| y).max().unwrap();
        let mut grid = new_grid(*max_x as usize + 1, *max_y as usize + 1);

        for point in &points {
            grid_on(&mut grid, &point);
        }
        print_grid(&grid);

        panic!();
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point(0, 0);
        let p2 = Point(6, 6);
        assert_eq!(manhattan_distance(&p1, &p2), 12);
        assert_eq!(manhattan_distance(&p2, &p1), 12);
    }
}
