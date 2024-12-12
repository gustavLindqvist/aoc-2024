pub fn star1() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut res = 0;
    for y in 0..m {
        for x in 0..n {
            if visited[y][x] {
                continue;
            }
            let mut q = vec![(y, x)];
            let mut area = 0;
            let mut perimiter = 0;
            while let Some(pos) = q.pop() {
                if visited[pos.0][pos.1] {
                    continue;
                }
                area += 1;
                visited[pos.0][pos.1] = true;
                for next in [
                    (pos.0.wrapping_sub(1), pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0 + 1, pos.1),
                    (pos.0, pos.1.wrapping_sub(1)),
                ] {
                    if (next.0 >= m) || (next.1 >= n) || grid[next.0][next.1] != grid[y][x] {
                        perimiter += 1;
                    } else {
                        q.push(next);
                    }
                }
            }
            res += area * perimiter;
        }
    }
    println!("{}", res);
}
