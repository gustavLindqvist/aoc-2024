use std::collections::HashSet;
pub fn star1() {
    let grid: Vec<Vec<_>> = include_str!("data.in").lines().map(|l|l.chars().collect()).collect();
    let mut pos = (0,0);
    'outer: for y in 0..grid.len(){
        for x in 0..grid[0].len(){
            if grid[y][x] == '^'{
                pos = (y,x);
                break 'outer;
            }
        }
    }
    let mut dir = (-1,0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        visited.insert(pos);
        let next_pos = ((pos.0 as i64 + dir.0) as usize, (pos.1 as i64+ dir.1) as usize);
        if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len(){
            break;
        }
        if grid[next_pos.0][next_pos.1] == '#'{
            dir = (dir.1, -dir.0);
        } else {
            pos = next_pos;
        }
    }
    println!("{}", visited.len());
}
