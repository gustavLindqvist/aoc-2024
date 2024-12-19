use std::collections::VecDeque;
const N: usize = 70;

pub fn star2() {
    let stones: Vec<_> = include_str!("data.in")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect();
    let t = binary_search(&stones);
    println!("{},{}", stones[t].0, stones[t].1);
}

fn binary_search(stones: &[(usize, usize)]) -> usize {
    let mut lind = 1024;
    let mut hind = stones.len();
    let mut half = (lind + hind) / 2;

    while lind <= hind {
        if path(stones, half) {
            lind = half + 1;
        } else {
            hind = half - 1;
        }
        half = (hind + lind) / 2;
    }
    half + 1
}

fn path(stones: &[(usize, usize)], t: usize) -> bool {
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut visited = [[false; N + 1]; N + 1];
    while let Some(pos) = q.pop_front() {
        if pos == (N, N) {
            return true;
        }
        visited[pos.0][pos.1] = true;
        for next in [
            (pos.0, pos.1 + 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0.wrapping_sub(1), pos.1),
        ] {
            if (next.0 <= N) && (next.1 <= N) && !stones[..=t].contains(&(next)) && !visited[pos.0][pos.1]{
                q.push_back(next);
            }
        }
    }
    false
}
