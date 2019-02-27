#[derive(Debug)]
struct Node {
    meta: Vec<usize>,
    children: Vec<Node>,
}

impl Node {
    fn new() -> Node {
        Node {
            meta: vec![],
            children: vec![],
        }
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for child in &self.children {
            len += child.len();
        }
        2 + len + self.meta.len()
    }

    fn checksum(&self) -> usize {
        let mut checksum = self.meta.iter().sum();
        for child in &self.children {
            checksum += child.checksum();
        }
        checksum
    }

    fn checksum_2(&self) -> usize {
        if self.children.is_empty() {
            self.meta.iter().sum()
        } else {
            let mut checksum = 0;
            for i in &self.meta {
                let idx = *i - 1;
                if let Some(child) = self.children.get(idx) {
                    checksum += child.checksum_2()
                }
            }
            checksum
        }
    }
}

fn process_tree(ns: &[usize]) -> Node {
    let mut node = Node::new();
    let num_children = ns[0];
    let num_meta = ns[1];

    let mut children_end = 2;
    for _ in 0..num_children {
        let child = process_tree(&ns[children_end..]);
        children_end += child.len();
        node.children.push(child);
    }
    let meta = &ns[children_end..children_end + num_meta];
    node.meta.extend(meta);

    node
}

fn parse_input(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = include_str!("8.example.input");
        let nums = parse_input(&input);
        assert_eq!(process_tree(&nums).checksum(), 138);

        let input = include_str!("8.input");
        let nums = parse_input(&input);
        assert_eq!(process_tree(&nums).checksum(), 46781);
    }

    #[test]
    fn test_checksum_2() {
        let input = include_str!("8.example.input");
        let nums = parse_input(&input);
        assert_eq!(process_tree(&nums).checksum_2(), 66);

        let input = include_str!("8.input");
        let nums = parse_input(&input);
        assert_eq!(process_tree(&nums).checksum_2(), 21405);
    }
}
