pub fn star2() {
    let (grid, moves) = include_str!("data.in").split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<_>> = grid
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let mut pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '@').map(|x| (y, x)))
        .unwrap();
    'outer: for m in moves.chars() {
        let dir = match m {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, usize::MAX),
            '^' => (usize::MAX, 0),
            _ => continue,
        };
        if ['<', '>'].contains(&m) {
            let mut to = (pos.0, pos.1);
            loop {
                to = (to.0 + dir.0, to.1 + dir.1);
                if grid[to.0][to.1] == '.' {
                    while to != pos {
                        let from = (to.0 - dir.0, to.1 - dir.1);
                        grid[to.0][to.1] = grid[from.0][from.1];
                        to = from;
                    }
                    grid[pos.0][pos.1] = '.';
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                    break;
                }
                if grid[to.0][to.1] == '#' {
                    break;
                }
            }
        } else {
            let mut moves = vec![];
            let mut to = vec![(pos.0 + dir.0, pos.1 + dir.1)];
            while let Some(t) = to.pop() {
                if grid[t.0][t.1] == '.' {
                    let from = (t.0 - dir.0, t.1 - dir.1);
                    moves.push((from, t));
                }
                if grid[t.0][t.1] == '#' {
                    continue 'outer;
                }
                if grid[t.0][t.1] == '[' {
                    let from = (t.0 - dir.0, t.1 - dir.1);
                    moves.push((from, t));
                    to.push((t.0 + dir.0, t.1 + dir.1));
                    to.push((t.0 + dir.0, t.1 + dir.1 + 1));
                }
                if grid[t.0][t.1] == ']' {
                    let from = (t.0 - dir.0, t.1 - dir.1);
                    moves.push((from, t));
                    to.push((t.0 + dir.0, t.1 + dir.1));
                    to.push((t.0 + dir.0, t.1 + dir.1 - 1));
                }
            }
            let mut tmp = grid.clone();
            if dir.0 != 1{
                moves.sort_by(|a,b|a.0.cmp(&b.0));
            } else {
                moves.sort_by(|a,b|b.0.cmp(&a.0));   
            }
            for (from, to) in moves {
                tmp[to.0][to.1] = grid[from.0][from.1];
                tmp[from.0][from.1] = '.';
            }
            grid = tmp;
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
    }
    let res = grid
        .iter()
        .enumerate()
        .map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == '[')
                .map(move |(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", res);
}
