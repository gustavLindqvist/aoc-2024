pub fn star2() {
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
            let mut fence = vec![vec![0; n]; m];

            while let Some(pos) = q.pop() {
                if visited[pos.0][pos.1] {
                    continue;
                }
                area += 1;
                visited[pos.0][pos.1] = true;
                for (i, next) in [
                    (pos.0.wrapping_sub(1), pos.1),
                    (pos.0 + 1, pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1.wrapping_sub(1)),
                ]
                .into_iter()
                .enumerate()
                {
                    if (next.0 >= m) || (next.1 >= n) || grid[next.0][next.1] != grid[y][x] {
                        fence[pos.0][pos.1] += 1 << i;
                    } else {
                        q.push(next);
                    }
                }
            }
            let mut perimiter = 0;
            for y in 0..fence.len() {
                for x in 0..fence[0].len() {
                    // X Moves
                    for i in 0..2{
                        if (fence[y][x] & (1 << i) != 0) && ((x + 1) >= fence[0].len() || (fence[y][x+1] & (1 << i) == 0)) {
                            perimiter += 1;
                        }
                    }
                    // Y moves
                    for i in 2..4 {
                        if (fence[y][x] & (1 << i) != 0) && ((y + 1) >= fence.len() || (fence[y + 1][x] & (1 << i) == 0)) {
                            perimiter += 1;
                        }
                    }
                }
            }
            res += area * perimiter;
        }
    }
    println!("{}", res);
}
