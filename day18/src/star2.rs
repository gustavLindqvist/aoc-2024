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
    let mut t = 1024.min(stones.len());
    loop {
        let mut q = VecDeque::new();
        q.push_back(((0, 0), vec![]));
        let mut visited = [[false; N + 1]; N + 1];
        let mut res = vec![];
        while let Some((pos, time)) = q.pop_front() {
            if pos == (N, N) {
                res = time;
                break;
            }
            if visited[pos.0][pos.1] {
                continue;
            }
            visited[pos.0][pos.1] = true;
            for next in [
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1.wrapping_sub(1)),
                (pos.0.wrapping_sub(1), pos.1),
            ] {
                if (next.0 <= N) && (next.1 <= N) && !stones[..=t].contains(&(next)) {
                    let mut tmp = time.clone();
                    tmp.push(next);
                    q.push_back((next, tmp));
                }
            }
        }
        if res.is_empty() {
            break;
        }

        for (dt, test) in stones[t..].iter().enumerate().skip(1){
            if res.contains(test){
                t += dt;
                break;
            }
        }
    }
    println!("{},{}", stones[t].0, stones[t].1);
}
