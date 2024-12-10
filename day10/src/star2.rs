pub fn star2() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                res += trailheads(&grid, (y, x));
            }
        }
    }
    println!("{}", res);
}

fn trailheads(grid: &[Vec<u8>], start: (usize, usize)) -> u32 {
    let mut q = vec![start];
    let mut res = 0;
    while let Some(node) = q.pop() {
        let h = grid[node.0][node.1];
        if h == 9 {
            res += 1;
            continue;
        }
        for next in [
            (node.0 + 1, node.1),
            (node.0, node.1 + 1),
            (node.0.wrapping_sub(1), node.1),
            (node.0, node.1.wrapping_sub(1)),
        ] {
            if (next.0 < grid.len()) && (next.1 < grid[0].len()) && (grid[next.0][next.1] == h + 1)
            {
                q.push(next);
            }
        }
    }
    res
}
