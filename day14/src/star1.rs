pub fn star1() {
    let n = 101;
    let m = 103;
    let t = 100;
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
    let mut res = [0; 4];

    for ((x,y), (dx,dy)) in robots {
        let x = (x + t * dx.rem_euclid(n)) % n;
        let y = (y + t * dy.rem_euclid(m)) % m;
        if x < n/2 {
            if y < m/2 {
                res[0] += 1;
            }
            if y > m/2 {
                res[1] += 1;
            }
        }
        if x > n/2 {
            if y < m/2 {
                res[2] += 1;
            }
            if y > m/2 {
                res[3] += 1;
            }
        }
    }
    println!("{}", res.iter().product::<usize>());
}