pub fn star2() {
    let res = include_str!("data.in")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|p| p.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .flat_map(|v| [v.clone(), v.into_iter().rev().collect()])
        .filter(|v| {
            for i in 0..v.len() {
                let mut tmp = v.clone();
                tmp.remove(i);
                if tmp.windows(2).all(|v| (v[0] < v[1]) && (v[1] - v[0] < 4)){
                    return true;
                }
            }
            false
        })
        .count();
    println!("{}", res);
}
