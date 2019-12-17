use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Graph {
    nodes: HashSet<char>,
    edges: HashSet<(char, char)>,
}

impl Graph {
    fn new(edges: &[(char, char)]) -> Graph {
        let nodes: HashSet<_> = edges
            .iter()
            .cloned()
            .flat_map(|(a, b)| vec![a, b])
            .collect();
        let edges: HashSet<_> = edges.iter().cloned().collect();
        Graph { nodes, edges }
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn children(&self) -> HashSet<&char> {
        self.edges.iter().map(|(_, b)| b).collect()
    }

    // Returns the nodes which have no parents
    fn no_parents(&self) -> HashSet<&char> {
        self.nodes
            .iter()
            .filter(|n| !self.children().contains(n))
            .collect()
    }

    fn next_nodes(&self) -> Vec<char> {
        let mut nodes: Vec<_> = self.no_parents().into_iter().cloned().collect();
        nodes.sort();
        nodes.reverse();
        nodes
    }

    fn next_node(&self) -> Option<char> {
        let mut nodes = self.next_nodes();
        nodes.pop()
    }

    fn remove_node(&mut self, c: char) {
        self.nodes.remove(&c);
        let edges: HashSet<(char, char)> = self
            .edges
            .iter()
            .cloned()
            .filter(|(a, b)| *a != c && *b != c)
            .collect();
        self.edges = edges;
    }
}

fn do_steps(graph: Graph, done: &[char]) -> Vec<char> {
    let mut graph = graph;
    let mut done: Vec<_> = done.iter().cloned().collect();

    if let Some(c) = &graph.next_node().clone() {
        graph.remove_node(*c);
        done.push(*c);
        do_steps(graph, &done)
    } else {
        done
    }
}

fn ordered_steps(pairs: &[(char, char)]) -> String {
    let graph = Graph::new(pairs);

    let steps = do_steps(graph, &vec![]);
    steps.iter().collect()
}

type Workers = Vec<VecDeque<char>>;

fn step_to_vec(c: char, time: i32) -> Vec<char> {
    let count = (c as i32) - 64 + time;
    std::iter::repeat(c).take(count as usize).collect()
}

fn timed_steps(
    mut graph: Graph,
    base_time: i32,
    mut workers: Workers,
    done: &[char],
    time: u32,
) -> (Vec<char>, u32) {
    let mut done: Vec<_> = done.iter().cloned().collect();

    // Process completed steps for all workers
    for worker in &mut workers {
        if let Some(c) = &worker.pop_front() {
            if worker.is_empty() {
                graph.remove_node(*c);
                done.push(*c);
            }
        }
    }

    let mut next_steps: Vec<_> = graph
        .next_nodes()
        .into_iter()
        .filter(|c| !workers.iter().any(|w| w.contains(c)))
        .collect();
    for worker in &mut workers {
        if worker.is_empty() {
            if let Some(c) = &next_steps.pop() {
                let cs = step_to_vec(*c, base_time);
                worker.extend(cs)
            }
        }
    }

    print!("{:3} ", time);
    for worker in &workers {
        let c = match worker.get(0) {
            Some(c) => c,
            None => &'.',
        };
        print!("{} ", c);
    }
    println!("{}", done.iter().collect::<String>());

    if !graph.is_empty() {
        timed_steps(graph, base_time, workers, &done, time + 1)
    } else {
        (done, time)
    }
}

fn step_time(pairs: &[(char, char)], nworkers: usize, base_time: i32) -> (String, u32) {
    let graph = Graph::new(pairs);
    let workers = std::iter::repeat(VecDeque::new()).take(nworkers).collect();

    let (steps, time) = timed_steps(graph, base_time, workers, &vec![], 0);
    (steps.into_iter().collect(), time)
}

fn parse_line(s: &str) -> (char, char) {
    let fields: Vec<_> = s.split_whitespace().collect();
    (
        fields[1].chars().next().unwrap(),
        fields[7].chars().next().unwrap(),
    )
}

fn parse_lines(lines: &[&str]) -> Vec<(char, char)> {
    lines.iter().map(|l| parse_line(l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_steps() {
        let input = include_str!("7.example.input");
        let lines: Vec<_> = input.lines().collect();
        let pairs = parse_lines(&lines);
        let steps = ordered_steps(&pairs);
        assert_eq!(steps, "CABDFE");

        let input = include_str!("7.input");
        let lines: Vec<_> = input.lines().collect();
        let pairs = parse_lines(&lines);
        let steps = ordered_steps(&pairs);
        assert_eq!(steps, "ABGKCMVWYDEHFOPQUILSTNZRJX");
    }

    #[test]
    fn test_step_time() {
        let input = include_str!("7.example.input");
        let lines: Vec<_> = input.lines().collect();
        let pairs = parse_lines(&lines);
        let (steps, time) = step_time(&pairs, 2, 0);
        assert_eq!(steps, "CABFDE");
        assert_eq!(time, 15);

        let input = include_str!("7.input");
        let lines: Vec<_> = input.lines().collect();
        let pairs = parse_lines(&lines);
        let (_, time) = step_time(&pairs, 5, 60);
        assert_eq!(time, 898);
    }
}
