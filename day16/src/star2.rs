use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

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
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (y, x)))
        .unwrap();
    let (dist, parents) = dijkstra(&grid, start, 0);
    let (end_dir, _) = (0..4)
        .map(|i| dist.get(&(end, i)).unwrap_or(&usize::MAX))
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let mut paths = HashSet::new();
    let mut q = vec![(end, end_dir as u8)];

    while let Some(current) = q.pop() {
        paths.insert(current.0);
        for next in parents.get(&current).unwrap_or(&vec![]) {
            q.push(*next);
        }
    }
    println!("{}", paths.len());
}

fn dijkstra(
    grid: &[Vec<char>],
    start: (usize, usize),
    start_dir: u8,
) -> (
    HashMap<((usize, usize), u8), usize>,
    HashMap<((usize, usize), u8), Vec<((usize, usize), u8)>>,
) {
    let mut q = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut parents: HashMap<((usize, usize), u8), Vec<((usize, usize), u8)>> = HashMap::new();

    q.push(Reverse((0, start, start_dir)));
    dist.insert((start, start_dir), 0);

    while let Some(Reverse((path_cost, vertex, dir))) = q.pop() {
        // println!("NODE: {:?}, {}, {}", vertex, dir, path_cost);
        let mut edges = vec![
            (vertex, (dir + 1) % 4, 1000),
            (vertex, dir.wrapping_sub(1) % 4, 1000),
        ];
        let next_v = match dir {
            0 => (vertex.0, vertex.1 + 1),
            1 => (vertex.0 + 1, vertex.1),
            2 => (vertex.0, vertex.1.wrapping_sub(1)),
            3 => (vertex.0.wrapping_sub(1), vertex.1),
            _ => unreachable!(),
        };
        if grid[next_v.0][next_v.1] != '#' {
            edges.insert(0, (next_v, dir, 1));
        }
        for (v, d, cost) in edges {
            let cost_togo = path_cost + cost;
            match dist.get(&(v, d)) {
                // Skip start or lower cost
                Some(old_cost) if cost_togo > *old_cost => {}
                Some(old_cost) if cost_togo == *old_cost => {
                    parents.entry((v, d)).or_default().push((vertex, dir));
                }
                // Not start, Not found or lower cost
                _ => {
                    // println!("INSERT: {:?}, {}, {}", v, d, cost_togo);
                    parents.entry((v, d)).or_default().push((vertex, dir));
                    dist.insert((v, d), cost_togo);
                    q.push(Reverse((cost_togo, v, d)));
                }
            }
        }
    }

    (dist, parents)
}
