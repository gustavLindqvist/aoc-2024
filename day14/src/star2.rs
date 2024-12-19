use rayon::prelude::*;
const N: i64 = 101;
const M: i64 = 103;

pub fn star2() {
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
    let res = (0..(N * M))
        .par_bridge()
        .find_any(|t| inner(robots.clone(), *t))
        .unwrap();
    println!("{}", res);
}

fn inner(robots: Vec<((i64, i64), (i64, i64))>, t: i64) -> bool {
    let mut v = std::collections::HashSet::new();
    for ((x, y), (dx, dy)) in robots {
        let x = (x + t * dx.rem_euclid(N)) % N;
        let y = (y + t * dy.rem_euclid(M)) % M;
        if v.contains(&(y * N + x)) {
            return false;
        } else {
            v.insert(y * N + x);
        }
    }
    true
}
