use std::collections::{HashMap, HashSet};
pub fn star1() {
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (n1, n2) in include_str!("data.in")
        .lines()
        .map(|l| l.split_once("-").unwrap())
    {
        edges.entry(n1).or_default().insert(n2);
        edges.entry(n2).or_default().insert(n1);
    }
    let nodes: Vec<_> = edges.keys().collect();
    let mut res = 0;
    for n1 in 0..nodes.len() {
        for n2 in n1..nodes.len() {
            for n3 in n2..nodes.len() {
                let e1 = edges.get(nodes[n1]).unwrap();
                if e1.contains(nodes[n2])
                    && e1.contains(nodes[n3])
                    && edges.get(nodes[n2]).unwrap().contains(nodes[n3])
                && (nodes[n1].starts_with('t')
                    || nodes[n2].starts_with('t')
                    || nodes[n3].starts_with('t'))
                {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}