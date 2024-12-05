use std::collections::HashMap;
use std::collections::HashSet;
pub fn star1() {
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
        let mut valid = true;
        'outer: for i in 0..update.len() - 1 {
            if let Some(values) = ordering.get(&update[i]) {
                for u in update.iter().skip(i + 1) {
                    if !values.contains(u) {
                        valid = false;
                        break 'outer;
                    }
                }
            } 
        }
        if valid {
            res += update[update.len() / 2];
        }
    }

    println!("{}", res);
}
