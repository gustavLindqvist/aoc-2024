pub fn star1() {
    let lines = include_str!("data.in")
        .lines()
        .map(|l| l.split_once(":").unwrap());
    let mut acc = 0;
    for (res, nums) in lines {
        let res = res.parse::<usize>().unwrap();
        let mut nums = nums.split_whitespace().map(|n| n.parse::<usize>().unwrap());
        let found = nums.fold(vec![0], |v, n| {
            v.iter().flat_map(|e| [e * n, e + n]).collect()
        });
        if found.contains(&res) {
            acc += res;
        }
    }
    println!("{}", acc);
}
