use std::collections::{HashMap, HashSet, VecDeque};

pub fn star2() {
    let secrets: Vec<_> = include_str!("data.in")
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect();
    let mut big_map: HashMap<isize, isize> = HashMap::new();
    for mut secret in secrets {
        let mut price = secret % 10;
        let mut sequence = VecDeque::new();
        let mut seen = HashSet::new();

        for _ in 0..2_000 {
            let next_secret = morph(secret);
            let next_price = next_secret % 10;
            sequence.push_back(next_price - price + 9);
            secret = next_secret;
            price = next_price;

            if sequence.len() > 4 {
                sequence.pop_front();
                let hash = hash(&sequence);
                if !seen.contains(&hash) {
                    *big_map.entry(hash).or_default() += price;
                    seen.insert(hash);
                }
            }
        }
    }
    println!("{}", big_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1);
}

fn hash(seq: &VecDeque<isize>) -> isize {
    (seq[0] << 12) + (seq[1] << 8) + (seq[2] << 4) + (seq[3])
}

fn morph(mut secret: isize) -> isize {
    secret ^= secret << 6;
    secret &= (1 << 24) - 1;
    secret ^= secret >> 5;
    secret &= (1 << 24) - 1;
    secret ^= secret << 11;
    secret &= (1 << 24) - 1;
    secret
}
