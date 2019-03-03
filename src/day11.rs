use std::collections::HashMap;
use std::fmt;

const GRID_SIZE: usize = 300;

#[derive(Debug)]
struct Grid(Vec<Vec<i32>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for column in self.0.iter().skip(1).take(100) {
            for row in column.iter().skip(1).take(100) {
                write!(f, "{:2} ", row)?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

impl Grid {
    fn new(serial: i32) -> Grid {
        let mut grid = vec![];
        grid.push(vec![]);
        for x in 1..=GRID_SIZE {
            let mut column = vec![0];
            for y in 1..=GRID_SIZE {
                column.push(power(x, y, serial));
            }
            grid.push(column)
        }
        Grid(grid)
    }
}

fn max_power(grid: &Grid) -> (usize, usize) {
    let mut powers = HashMap::new();
    for (x, columns) in grid.0.windows(3).enumerate() {
        for column in columns {
            for (y, rows) in column.windows(3).enumerate() {
                *powers.entry((x, y)).or_insert(0) += rows.iter().sum::<i32>();
            }
        }
    }
    powers.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

fn power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let power = rack_id * y as i32;
    let power = power + serial;
    let power = power * rack_id;
    hundreds_digit(power) - 5
}

fn hundreds_digit(n: i32) -> i32 {
    n / 100 % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hundreds_digit() {
        assert_eq!(hundreds_digit(0), 0);
        assert_eq!(hundreds_digit(123), 1);
        assert_eq!(hundreds_digit(12345), 3);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(3, 5, 8), 4);
        assert_eq!(power(122, 79, 57), -5);
        assert_eq!(power(217, 196, 39), 0);
        assert_eq!(power(101, 153, 71), 4);
    }

    #[test]
    fn test_max_power() {
        let grid = Grid::new(18);
        assert_eq!(max_power(&grid), (33, 45));

        let grid = Grid::new(8772);
        assert_eq!(max_power(&grid), (235, 31));
    }
}
