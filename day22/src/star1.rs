pub fn star1() {
    println!(
        "{}",
        (0..2000)
            .fold(
                include_str!("data.in")
                    .lines()
                    .map(|l| l.parse().unwrap())
                    .collect::<Vec<usize>>(),
                |acc, _| acc.into_iter().map(morph).collect(),
            )
            .iter()
            .sum::<usize>()
    );
}
fn morph(mut secret: usize) -> usize {
    secret ^= secret << 6;
    secret &= (1 << 24) - 1;
    secret ^= secret >> 5;
    secret &= (1 << 24) - 1;
    secret ^= secret << 11;
    secret &= (1 << 24) - 1;
    secret
}
