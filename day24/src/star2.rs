use std::collections::{HashSet, VecDeque};

pub fn star2() {
    let (_, gates) = include_str!("data.in").split_once("\n\n").unwrap();
    let q = gates
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|l| (l[0], l[1], l[2], l[4]))
        .collect::<VecDeque<_>>();

    // Graph with node for all gates
    let edges: HashSet<_> = q
        .clone()
        .into_iter()
        .flat_map(|(i1, op, i2, _)| [(i1, op), (i2, op)])
        .collect();

    let mut faulty = vec![];

    for (i1, op, i2, out) in q.clone() {
        match (i1, i2, op, out) {
            (i1, i2, "AND", out) if i1 != "x00" && i2 != "x00" && !edges.contains(&(out, "OR")) => {
                faulty.push(out);
            }
            (_, _, "OR", out) if out.starts_with('z') && out != "z45" => {
                faulty.push(out);
            }
            (_, _, "OR", out) if edges.contains(&(out, "OR")) => {
                faulty.push(out);
            }
            (i1, i2, "XOR", out)
                if (i1.starts_with('x') || i2.starts_with('x'))
                    && i1 != "x00"
                    && i2 != "x00"
                    && !edges.contains(&(out, "XOR")) =>
            {
                faulty.push(out);
            }
            (_, _, "XOR", out)
                if !i1.starts_with('x') && !i2.starts_with('x') && !out.starts_with('z') =>
            {
                faulty.push(out);
            }
            _ => {}
        }
    }

    let mut res: Vec<_> = faulty.into_iter().collect();
    res.sort();
    println!("{}", res.join(","));
}
