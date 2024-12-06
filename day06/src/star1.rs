pub fn star1() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut pos = (0, 0);
    'outer: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                pos = (y, x);
                break 'outer;
            }
        }
    }
    let mut dir = 0;
    let mut visited: Vec<Vec<u8>> = vec![vec![0; grid[0].len()]; grid.len()];
    loop {
        visited[pos.0][pos.1] = 1;

        let next_pos = match dir {
            0 => (pos.0.wrapping_sub(1), pos.1),
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0 + 1, pos.1),
            3 => (pos.0, pos.1.wrapping_sub(1)),
            _ => unreachable!(),
        };

        if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
            break;
        }
        if grid[next_pos.0][next_pos.1] == '#' {
            dir = (dir + 1) % 4;
        } else {
            pos = next_pos;
        }
    }
    println!("{}", visited.into_iter().map(|l| l.into_iter().map(|u| u as u32).sum::<u32>()).sum::<u32>());
}
