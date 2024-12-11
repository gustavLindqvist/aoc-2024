pub fn star1() {
    let mut nums: Vec<_> = include_str!("data.in").split_whitespace().map(|u|u.parse::<usize>().unwrap()).collect();
    for _ in 0..25{
        nums = nums.into_iter().flat_map(|n|{
            let digits = (n as f64).log10().floor() as u32 + 1;
            match n{
                0 => vec![1],
                _e if digits % 2 == 0 => {
                    let div = 10_usize.pow(digits/2);
                    vec![n / div, n % div]},
                _ => vec![n * 2024]
            }}).collect();
    }
    println!("{}",nums.len());
}
