pub fn star1() {println!("{}",regex::Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap().captures_iter(include_str!("data.in")).map(|c| c.extract()).map(|(_, [f1, f2])| f1.parse::<usize>().unwrap() * f2.parse::<usize>().unwrap()).sum::<usize>());}