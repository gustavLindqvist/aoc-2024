pub fn star2() {
    let lines = include_str!("data.in")
        .lines()
        .map(|l| l.split_once(":").unwrap());
    let mut acc = 0;
    for (res, nums) in lines {
        let res = res.parse::<i64>().unwrap();
        let par: Vec<_> = nums
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        if rec(res, &par) {
            acc += res;
        }
    }
    println!("{}", acc);
}

fn rec(target: i64, nums: &[i64]) -> bool {
    if target < 0 {
        return false;
    }
    if nums.is_empty() {
        return target == 0;
    }
    let x = nums[nums.len() - 1];
    let y = 10_i64.pow(x.ilog10() + 1);
    (target % y == x && rec(target / y, &nums[..nums.len() - 1]))
        || (target % x == 0 && rec(target / x, &nums[..nums.len() - 1]))
        || rec(target - x, &nums[..nums.len() - 1])
}
