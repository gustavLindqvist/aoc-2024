use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn star2() {
    let (towels, patterns) = include_str!("data.in").split_once("\n\n").unwrap();
    let towels: HashSet<_> = towels.split(", ").collect();
    let res = patterns
        .trim()
        .lines()
        .par_bridge()
        .map(|p| feasible(&towels, p, 0, &mut HashMap::new()))
        .sum::<usize>();
    println!("{}", res);
}

fn feasible(
    towels: &HashSet<&str>,
    pattern: &str,
    index: usize,
    map: &mut HashMap<String, usize>,
) -> usize {
    if map.contains_key(pattern) {
        return *map.get(pattern).unwrap();
    }
    let mut ways = 0;
    if pattern.is_empty() {
        return 1;
    }
    if index == pattern.len() {
        return ways;
    }
    if towels.contains(&&pattern[..=index]) {
        ways += feasible(towels, &pattern[(index + 1)..], 0, map);
    }
    ways += feasible(towels, pattern, index + 1, map);
    map.insert(pattern.to_string(), ways);
    ways
}
