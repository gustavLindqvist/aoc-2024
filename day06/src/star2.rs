use std::collections::HashSet;
pub fn star2() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut start = (0, 0);
    'outer: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                start = (y, x);
                break 'outer;
            }
        }
    }
    let mut res = 0;
    let mut possible = vec![vec![1]];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if possible[y][x] == 0 {
                continue;
            }
            let mut pos = start;
            let mut dir = 0;
            let mut visited: Vec<Vec<u8>> = vec![vec![0; grid[0].len()]; grid.len()];
            loop {
                visited[pos.0][pos.1] += 1 << dir;

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
                if grid[next_pos.0][next_pos.1] == '#' || next_pos == (y, x) {
                    dir = (dir + 1) % 4;
                } else {
                    if visited[next_pos.0][next_pos.1] & 1 << dir != 0{
                        res += 1;
                        break;
                    }
                    pos = next_pos;                }
            }
            if (y, x) == (0, 0) {
                possible = visited;
            }
        }
    }
    println!("{}", res);
}
