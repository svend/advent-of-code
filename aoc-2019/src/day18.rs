// https://adventofcode.com/2019/day/18

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Point {
    Start,
    Key(char),
    Door(char),
}

type Distance = usize;

type Edge = (Point, Point, Distance);

#[derive(Debug)]
struct Graph(Vec<Edge>);

impl Graph {
    /// Find all adjacent points to point
    fn find(&self, point: &Point) -> Vec<(&Point, Distance)> {
        self.0
            .iter()
            .flat_map(|(p1, p2, d)| {
                if p1 == point {
                    Some((p2, *d))
                } else if p2 == point {
                    Some((p1, *d))
                } else {
                    None
                }
            })
            .collect()
    }

    fn points(&self) -> HashSet<&Point> {
        self.0
            .iter()
            .flat_map(|(p1, p2, _d)| vec![p1, p2])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Point::*;
    use super::*;

    #[test]
    fn test_foo() {
        let graph = vec![
            (Start, Key('a'), 2),
            (Start, Door('a'), 2),
            (Door('a'), Key('b'), 2),
        ];
        let graph = Graph(graph);
        println!("{:?}", graph);
        println!("points {:?}", graph.points());
        println!("start {:?}", graph.find(&Start));
        println!("a {:?}", graph.find(&Door('a')));
        unimplemented!();
    }
}
