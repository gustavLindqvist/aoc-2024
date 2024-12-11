use std::collections::HashMap;

pub fn star1() {
    let mut nums: HashMap<_, _> = include_str!("data.in")
        .split_whitespace()
        .map(|u| (u.parse::<usize>().unwrap(), 1))
        .collect();
    for _ in 0..25 {
        let mut new: HashMap<usize, usize> = HashMap::new();
        for (n, count) in nums {
            let digits = (n as f64).log10().floor() as u32 + 1;
            match n {
                0 => *new.entry(1).or_default() += count,
                _e if digits % 2 == 0 => {
                    let div = 10_usize.pow(digits / 2);
                    *new.entry(n % div).or_default() += count;
                    *new.entry(n / div).or_default() += count;
                }
                _ => *new.entry(n * 2024).or_default() += count,
            }
        }
        nums = new;
    }
    println!("{}", nums.values().sum::<usize>());
}
