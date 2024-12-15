pub fn star1() {
    let (grid, moves) = include_str!("data.in").split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<_>> = grid.lines().map(|l| l.chars().collect()).collect();
    let mut pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '@').map(|x| (y, x)))
        .unwrap();
    for m in moves.chars() {
        let dir = match m {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, usize::MAX),
            '^' => (usize::MAX, 0),
            _ => continue,
        };
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
    }
    let res = grid
        .iter()
        .enumerate()
        .map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'O')
                .map(move |(x, _)| y * 100 + x)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", res);
}
