use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn star1() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (y, x)))
        .unwrap();
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (y, x)))
        .unwrap();
    let dist = dijkstra(&grid, start);
    let goal = dist.get(&end).unwrap();
    let mut res = 0;
    for vertex in dist.keys() {
        let edges = [
            (vertex.0, vertex.1 + 2),
            (vertex.0 + 1, vertex.1 + 1),
            (vertex.0 + 2, vertex.1),
            (vertex.0 + 1, vertex.1.wrapping_sub(1)),
            (vertex.0, vertex.1.wrapping_sub(2)),
            (vertex.0.wrapping_sub(1), vertex.1.wrapping_sub(1)),
            (vertex.0.wrapping_sub(2), vertex.1),
            (vertex.0.wrapping_sub(1), vertex.1 + 1),
        ];
        let edges = edges
            .iter()
            .filter(|e| (e.0 < grid.len()) && (e.1 < grid[0].len()) && (grid[e.0][e.1] != '#'));
        for skip in edges {
            let start = goal - dist.get(vertex).unwrap();
            let end = goal - dist.get(skip).unwrap();
            if start > end && (start - end - 2 >= 100) {
                res += 1;
            }
        }
    }
    println!("{}", res);
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
