pub fn star2() {
    let n = 101;
    let m = 103;
    let re = regex::Regex::new(r"[^\-0-9]+").unwrap();
    let robots: Vec<_> = include_str!("data.in")
        .lines()
        .map(|l| {
            re.split(l)
                .skip(1)
                .map(|w| w.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .collect();
    // Hella ugly men jag är för bakis
    'outer: for t in 0..n * m {
        let mut v = std::collections::HashSet::new();
        for ((x, y), (dx, dy)) in robots.clone() {
            let x = (x + t * dx.rem_euclid(n)) % n;
            let y = (y + t * dy.rem_euclid(m)) % m;
            if v.contains(&(y * n + x)) {
                continue 'outer;
            } else {
                v.insert(y * n + x);
            }
        }
        println!("{}", t);
        return;
        // for y in 0..m {
        //     for x in 0..n {
        //         if v.contains(&(y, x)) {
        //             print!("#");
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }
    }
}
