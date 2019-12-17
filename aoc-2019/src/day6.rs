use std::collections::{HashMap, HashSet};

struct Graph(HashMap<String, HashSet<String>>);

impl Graph {
    fn new(edges: &[(String, String)]) -> Self {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        for (n1, n2) in edges {
            let entry = graph.entry(n1.to_string()).or_insert_with(HashSet::new);
            (*entry).insert(n2.to_string());
        }
        Graph(graph)
    }

    fn nodes(&self) -> HashSet<&str> {
        self.0.keys().map(|s| s.as_str()).collect()
    }

    fn children(&self) -> HashSet<&str> {
        self.0.values().flatten().map(|s| s.as_str()).collect()
    }

    fn parent(&self, node: &str) -> Option<&str> {
        self.0
            .iter()
            .filter(|(_k, v)| v.contains(node))
            .map(|(k, _v)| k.as_str())
            .next()
    }

    fn path(&self, node: &str, path: &mut Vec<String>) {
        path.push(node.to_string());
        if let Some(parent) = self.parent(node) {
            self.path(parent, path);
        }
    }

    fn start(&self) -> Option<&str> {
        let children = self.children();
        let nodes = self.nodes();
        match nodes.difference(&children).collect::<Vec<_>>()[..] {
            [node] => Some(*node),
            [] => None,
            _ => panic!("multiple starting nodes found"),
        }
    }

    fn count_orbits_from(&self, start: &str, counts: &mut Vec<usize>) -> usize {
        if let Some(children) = self.0.get(start) {
            let count = children.len()
                + children
                    .iter()
                    .map(|c| self.count_orbits_from(&c, counts))
                    .sum::<usize>();
            counts.push(count);
            count
        } else {
            0
        }
    }

    fn count_orbits(&self) -> usize {
        let mut counts: Vec<usize> = Vec::new();
        if let Some(start) = self.start() {
            self.count_orbits_from(&start, &mut counts);
        }
        counts.iter().sum::<usize>()
    }

    fn print_from(&self, start: &str) {
        print!("{}", start);
        if let Some(children) = self.0.get(start) {
            print!(" ");
            for child in children {
                self.print_from(&child);
            }
        } else {
            print!(" | ")
        }
    }

    fn print(&self) {
        if let Some(start) = self.start() {
            self.print_from(&start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input(s: &str) -> Vec<(String, String)> {
        s.lines()
            .map(|line| {
                let parts: Vec<_> = line.split(')').collect();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect()
    }

    #[test]
    fn test_part_1() {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
        let edges = parse_input(input);
        let graph = Graph::new(&edges);
        assert_eq!(graph.count_orbits(), 42);

        let input = include_str!("day6.input");
        let edges = parse_input(input);
        let graph = Graph::new(&edges);
        assert_eq!(graph.count_orbits(), 194721);
    }

    #[test]
    fn test_part_2() {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;
        let edges = parse_input(input);
        let graph = Graph::new(&edges);
        assert_eq!(graph.count_orbits(), 54);
        assert_eq!(graph.parent("SAN"), Some("I"));
        assert_eq!(graph.parent("COM"), None);
        let mut path1 = Vec::new();
        graph.path("SAN", &mut path1);
        dbg!(&path1);
        let mut path2 = Vec::new();
        graph.path("YOU", &mut path2);
        dbg!(&path2);
        let path1: HashSet<_> = path1.iter().collect();
        let path2: HashSet<_> = path2.iter().collect();
        assert_eq!(path1.symmetric_difference(&path2).count() - 2, 4);

        let input = include_str!("day6.input");
        let edges = parse_input(input);
        let graph = Graph::new(&edges);
        let mut path1 = Vec::new();
        graph.path("SAN", &mut path1);
        let mut path2 = Vec::new();
        graph.path("YOU", &mut path2);
        let path1: HashSet<_> = path1.iter().collect();
        let path2: HashSet<_> = path2.iter().collect();
        dbg!(path1.symmetric_difference(&path2).count() - 2, 316);
    }
}
