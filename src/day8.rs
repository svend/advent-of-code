fn process_tree(ns: &[usize], meta2: &[usize]) -> (usize, Vec<usize>) {
    let mut meta2: Vec<_> = meta2.into_iter().cloned().collect();
    dbg!(&meta2);

    let num_children = ns[0];
    let num_meta = ns[1];

    let mut len_children = 0;
    for _ in 0..num_children {
        let (x, meta) = process_tree(&ns[(2 + len_children)..], &vec![]);
        meta2.extend(meta);
        len_children += x;
    }
    let meta = &ns[2 + len_children..2 + len_children + num_meta];
    meta2.extend(meta);

    dbg!(num_children);
    dbg!(num_meta);
    dbg!(meta);
    (2 + len_children + num_meta, meta2)
}

fn process_data(ns: &[usize]) -> usize {
    let (_, meta) = process_tree(&ns, &vec![]);
    dbg!(&meta);
    meta.into_iter().sum()
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
    fn test() {
        let input = include_str!("8.example.input");
        let nums = parse_input(&input);
        assert_eq!(process_data(&nums), 138);

        // let input = include_str!("8.input");
        // let nums = parse_input(&input);
        // assert_eq!(process_data(&nums), 0);
    }
}
