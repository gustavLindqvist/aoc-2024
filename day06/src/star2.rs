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
    let mut possible = HashSet::new();
    possible.insert((0,0));
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !possible.contains(&(y,x)){
                continue;
            }
            let mut pos = start;
            let mut dir = (-1, 0);
            let mut visited = HashSet::new();

            loop {
                visited.insert((pos, dir));

                let next_pos = (
                    (pos.0 as i64 + dir.0) as usize,
                    (pos.1 as i64 + dir.1) as usize,
                );
                if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
                    break;
                }
                if grid[next_pos.0][next_pos.1] == '#' || next_pos == (y, x) {
                    dir = (dir.1, -dir.0);
                } else {
                    if visited.contains(&(next_pos, dir)){
                        res += 1;
                        break;
                    }
                    pos = next_pos;
                }
            }
            if (y, x) == (0,0){
                for (p, _) in visited{
                    possible.insert(p);
                }
            }
        }
    }
    println!("{}", res);
}
