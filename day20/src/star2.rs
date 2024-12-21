use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn star2() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (y, x)))
        .unwrap();

    let dist = path(&grid, start);
    let res = dist
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((d1, &(y1, x1)), (d2, &(y2, x2)))| {
            let d = y1.abs_diff(y2) + x1.abs_diff(x2);
            d <= 20 && (d2 - d1 - d >= 100)
        })
        .count();
    println!("{}", res);
}

fn path(grid: &[Vec<char>], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![start, start];
    let mut curr = 1;
    while curr < path.len() {
        let vertex = path[curr];
        for e in [
            (vertex.0, vertex.1 + 1),
            (vertex.0 + 1, vertex.1),
            (vertex.0, vertex.1.wrapping_sub(1)),
            (vertex.0.wrapping_sub(1), vertex.1),
        ]
        .into_iter()
        .filter(|e| grid[e.0][e.1] != '#')
        {
            if e != path[curr - 1] {
                path.push(e);
            }
        }
        curr += 1;
    }
    path.remove(0);
    path
}

fn dijkstra(grid: &[Vec<char>], start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut q = BinaryHeap::new();
    let mut dist = HashMap::new();

    q.push(Reverse((0, start)));
    dist.insert(start, 0);

    while let Some(Reverse((path_cost, vertex))) = q.pop() {
        let edges = [
            (vertex.0, vertex.1 + 1),
            (vertex.0 + 1, vertex.1),
            (vertex.0, vertex.1.wrapping_sub(1)),
            (vertex.0.wrapping_sub(1), vertex.1),
        ];
        let edges = edges.iter().filter(|e| grid[e.0][e.1] != '#');

        for v in edges {
            let cost_togo = path_cost + 1;
            match dist.get(v) {
                Some(old_cost) if cost_togo >= *old_cost => {}
                _ => {
                    dist.insert(*v, cost_togo);
                    q.push(Reverse((cost_togo, *v)));
                }
            }
        }
    }
    dist
}
