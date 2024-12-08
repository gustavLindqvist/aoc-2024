use std::collections::{HashMap, HashSet};

pub fn star1() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut ants: HashMap<char, Vec<_>> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                ants.entry(grid[y][x])
                    .or_default()
                    .push((y as i64, x as i64));
            }
        }
    }
    let mut found = HashSet::new();
    for v in ants.values() {
        for n1 in 0..v.len() {
            for n2 in (n1 + 1)..v.len() {
                let dy = v[n1].0 - v[n2].0;
                let dx = v[n1].1 - v[n2].1;
                found.insert((v[n1].0 + dy, v[n1].1 + dx));
                found.insert((v[n2].0 - dy, v[n2].1 - dx));
            }
        }
    }

    let res = found
        .into_iter()
        .filter(|(y, x)| ((*y as usize) < grid.len()) && ((*x as usize) < grid[0].len()))
        .count();
    println!("{}", res);
}
