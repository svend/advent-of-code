use std::fmt;

#[derive(Debug)]
struct Graph(Vec<Point>);

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let points: Vec<_> = self.0.iter().map(|p| p.position).collect();

        let min_x = points.iter().map(|p| p.0).min().unwrap();
        let max_x = points.iter().map(|p| p.0).max().unwrap();
        let min_y = points.iter().map(|p| p.1).min().unwrap();
        let max_y = points.iter().map(|p| p.1).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if points.contains(&(x, y)) {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Graph {
    fn advance(&mut self) {
        for point in &mut self.0 {
            let x = point.position.0 + point.velocity.0;
            let y = point.position.1 + point.velocity.1;
            point.position = (x, y);
        }
    }

    fn back(&mut self) {
        for point in &mut self.0 {
            let x = point.position.0 - point.velocity.0;
            let y = point.position.1 - point.velocity.1;
            point.position = (x, y);
        }
    }

    fn width(&self) -> i32 {
        let points: Vec<_> = self.0.iter().map(|p| p.position).collect();
        let min = points.iter().map(|p| p.0).min().unwrap();
        let max = points.iter().map(|p| p.0).max().unwrap();
        max - min
    }

    fn height(&self) -> i32 {
        let points: Vec<_> = self.0.iter().map(|p| p.position).collect();
        let min = points.iter().map(|p| p.1).min().unwrap();
        let max = points.iter().map(|p| p.1).max().unwrap();
        max - min
    }
}

#[derive(Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn print_graph(graph: &Graph) {
    let points: Vec<_> = graph.0.iter().map(|p| p.position).collect();

    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn delim(c: char) -> bool {
    c == '<' || c == ',' || c == '>'
}

fn parse_line(s: &str) -> Point {
    let fields: Vec<_> = s.split(delim).map(|s| s.trim()).collect();
    Point {
        position: (fields[1].parse().unwrap(), fields[2].parse().unwrap()),
        velocity: (fields[4].parse().unwrap(), fields[5].parse().unwrap()),
    }
}

fn parse_input(s: &str) -> Graph {
    Graph(s.lines().map(|l| parse_line(l)).collect())
}

fn message(graph: &mut Graph) -> (String, u32) {
    let mut width = graph.width();
    let mut height = graph.height();
    let mut time = 0;
    loop {
        graph.advance();
        let width_new = graph.width();
        let height_new = graph.height();
        if width_new > width || height_new > height {
            graph.back();
            break;
        }
        time += 1;
        width = width_new;
        height = height_new;
    }
    (graph.to_string(), time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let input = include_str!("10.example.input");
        let solution = include_str!("10.example.solution");
        let mut graph = parse_input(&input);
        let (msg, time) = message(&mut graph);
        assert_eq!(msg, solution);
        assert_eq!(time, 3);

        let input = include_str!("10.input");
        let solution = include_str!("10.solution");
        let mut graph = parse_input(&input);
        let (msg, time) = message(&mut graph);
        assert_eq!(msg, solution);
        assert_eq!(time, 10054);
    }
}
