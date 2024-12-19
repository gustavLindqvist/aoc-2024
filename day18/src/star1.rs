use std::collections::VecDeque;
pub fn star1() {
    const N: usize = 70;
    let stones: Vec<_> = include_str!("data.in")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect();
    let t = 1024.min(stones.len());

    let mut q = VecDeque::new();
    q.push_back(((0, 0), 0));
    let mut visited = [[false; N + 1]; N + 1];
    let mut res = usize::MAX;
    while let Some((pos, time)) = q.pop_front() {
        if pos == (N, N) {
            res = time;
            break;
        }
        visited[pos.0][pos.1] = true;
        for next in [
            (pos.0, pos.1 + 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0.wrapping_sub(1), pos.1),
        ] {
            if (next.0 <= N) && (next.1 <= N) && !stones[..t].contains(&(next)) && !visited[pos.0][pos.1]{
                q.push_back((next, time + 1));
            }
        }
    }
    println!("{}", res);
}
