use std::collections::{HashMap, HashSet};

pub fn star2() {
    let (r, p) = include_str!("data.in").split_once("\n\n").unwrap();
    let mut ordering: HashMap<usize, HashSet<usize>> = HashMap::new();
    let updates: Vec<Vec<usize>> = p
        .lines()
        .map(|line| line.split(",").map(|i| i.parse().unwrap()).collect())
        .collect();
    for (k, v) in r.lines().map(|line| line.split_once("|").unwrap()) {
        ordering
            .entry(k.parse().unwrap())
            .or_default()
            .insert(v.parse().unwrap());
    }
    let mut res = 0;

    for update in updates {
        let mut tmp = update.clone();
        tmp.sort_by(|a, b| ordering.get(b).unwrap().contains(a).cmp(&true));
        if tmp != update {
            res += tmp[tmp.len() / 2];
        }
    }
    println!("{}", res);
}
