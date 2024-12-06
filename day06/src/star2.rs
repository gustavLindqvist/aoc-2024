use rayon::prelude::*;
pub fn star2() {
    let mut grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.bytes().collect())
        .collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|&x| x == b'^')
                .map(|col_idx| (row_idx, col_idx))
        })
        .unwrap();

    let mut tested: Vec<Vec<_>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut pos = start;
    let mut dir = 0;
    let mut work_items = vec![];
    loop {
        let next_pos = next(pos, dir);
        if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
            break;
        }
        if grid[next_pos.0][next_pos.1] == b'#' {
            dir = (dir + 1) % 4;
        } else {
            if !tested[next_pos.0][next_pos.1] {
                tested[next_pos.0][next_pos.1] = true;
                work_items.push((&grid, pos, (dir + 1) % 4, next_pos));
                // res += loops(&grid, pos, (dir + 1) % 4, next_pos);
            }
            pos = next_pos;
        }
    }
    let res: u32 = work_items.par_iter().map(|(g,p,d,b)|loops(g,*p,*d,*b)).sum();
    println!("{}", res);
}

fn loops(grid: &[Vec<u8>], mut pos: (usize, usize), mut dir: u8, block: (usize, usize)) -> u32 {
    let mut visited: Vec<Vec<u8>> = vec![vec![0; grid[0].len()]; grid.len()];
    visited[pos.0][pos.1] += 1 << ((dir + 3) % 4);
    loop {
        visited[pos.0][pos.1] += 1 << dir;

        let next_pos = next(pos, dir);

        if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
            return 0;
        }
        if grid[next_pos.0][next_pos.1] == b'#' || next_pos == block {
            dir = (dir + 1) % 4;
        } else {
            if visited[next_pos.0][next_pos.1] & 1 << dir != 0 {
                return 1;
            }
            pos = next_pos;
        }
    }
}

fn next(pos: (usize, usize), dir: u8) -> (usize, usize) {
    match dir {
        0 => (pos.0.wrapping_sub(1), pos.1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 + 1, pos.1),
        3 => (pos.0, pos.1.wrapping_sub(1)),
        _ => unreachable!(),
    }
}
