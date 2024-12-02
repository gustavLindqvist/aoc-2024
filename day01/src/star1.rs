pub fn star1() {
    let mut l1 = vec![];
    let mut l2 = vec![];
    for l in include_str!("data.in").lines(){
        let (n1, n2) = l.split_once("   ").unwrap();
        l1.push(n1.parse::<i64>().unwrap());
        l2.push(n2.parse::<i64>().unwrap());
    }
    l1.sort();
    l2.sort();
    let sum = l1.iter().zip(l2.iter()).fold(0, |sum, (n1, n2)| sum + (n2- n1).abs());
    println!("{}", sum);
}