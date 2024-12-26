use std::collections::{HashMap, VecDeque};

pub fn star1() {
    let (start, gates) = include_str!("data.in").split_once("\n\n").unwrap();
    let mut q = gates
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|l| (l[0], l[1], l[2], l[4]))
        .collect::<VecDeque<_>>();
    let mut done = HashMap::new();
    for (var, val) in start.lines().map(|l| l.split_once(" ").unwrap()) {
        done.insert(&var[0..=2], val == "1");
    }
    while let Some((in1, gate, in2, out)) = q.pop_front() {
        if done.contains_key(in1) && done.contains_key(in2) {
            let (a, b) = (done.get(in1).unwrap(), done.get(in2).unwrap());
            let val = match gate {
                "XOR" => a ^ b,
                "AND" => a & b,
                "OR" => a | b,
                _ => unreachable!(),
            };
            done.insert(out, val);
        } else {
            q.push_back((in1, gate, in2, out));
        }
    }
    let mut res: Vec<_> = done
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, v)| (k, if v { 1 } else { 0 }))
        .collect();
    res.sort_by(|a, b| b.cmp(a));
    let res: usize = res.into_iter().fold(0, |acc, (_, b)| acc * 2 + b);
    println!("{}", res);
}
