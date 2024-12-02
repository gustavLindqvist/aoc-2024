use std::collections::HashMap;
pub fn star2() {
    let mut l1 = vec![];
    let mut counter = HashMap::new();
    for l in include_str!("data.in").lines(){
        let (n1, n2) = l.split_once("   ").unwrap();
        let n2 = n2.parse::<i64>().unwrap();
        l1.push(n1.parse::<i64>().unwrap());
        *counter.entry(n2).or_insert(0) += 1;
    }
    let sum = l1.iter().fold(0, |sum, n1| sum + n1 * counter.get(n1).unwrap_or(&0));
    println!("{}", sum);
}
