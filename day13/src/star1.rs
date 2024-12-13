use regex::Regex;
pub fn star1() {
    let re = Regex::new(r"[^0-9]+").unwrap();
    let mut res = 0;
    for g in include_str!("data.in").split("\n\n") {
        let v: Vec<_> = re
            .split(g)
            .skip(1)
            .map(|w| w.parse::<i64>().unwrap())
            .collect();
        let div = v[0] * v[3] - v[1] * v[2];
        let x1 = v[0] * v[5] - v[1] * v[4];
        let x2 = v[3] * v[4] - v[2] * v[5];

        if div == 0 || x1 % div != 0 || x2 % div != 0 {
            continue;
        }

        let x1 = x1 / div;
        let x2 = x2 / div;

        if x1 < 0 || x2 < 0 {
            continue;
        }
        res += x1 + 3 * x2;
    }
    println!("{}", res);
}

// Ax = b
// x = A'b

//  1/   | a' b' | | b1 | = | x1 |
//  /div | c' d' | | b2 | = | x2 | 

// div = ad - bc
// a' = d / div
// b' = -b / div
// c' = -c / div
// d' = a / div

// x1 = (a' * b1 + b' * b2) / div
// x2 = (c' * b1 + d' * b2) / div