use std::collections::HashSet;

pub fn star1() {
    let (towels, patterns) = include_str!("data.in").split_once("\n\n").unwrap();
    let towels: HashSet<_> = towels.split(", ").collect();
    let mut res = 0;
    for pattern in patterns.trim().lines() {
        if feasible(&towels, pattern, 0) {
            res += 1;
        }
    }
    println!("{}", res);
}

fn feasible(towels: &HashSet<&str>, pattern: &str, index: usize) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if index == pattern.len() {
        return false;
    }
    if towels.contains(&&pattern[..=index]) && feasible(towels, &pattern[(index + 1)..], 0) {
        return true;
    }
    feasible(towels, pattern, index + 1)
}
