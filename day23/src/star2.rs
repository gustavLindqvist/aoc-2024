use std::collections::{HashMap, HashSet};
pub fn star2() {
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (n1, n2) in include_str!("data.in")
        .lines()
        .map(|l| l.split_once("-").unwrap())
    {
        edges.entry(n1).or_default().insert(n2);
        edges.entry(n2).or_default().insert(n1);
    }
    let mut clique = HashSet::new();
    bron_kerbosch(
        &mut HashSet::new(),
        &edges.keys().copied().collect(),
        &mut HashSet::new(),
        &edges,
        &mut clique,
    );
    let mut res: Vec<_> = clique.into_iter().collect();
    res.sort();
    println!("{}", res.join(","));
}

fn bron_kerbosch<'a>(
    r: &mut HashSet<&'a str>,
    p: &HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    edges: &HashMap<&str, HashSet<&'a str>>,
    clique: &mut HashSet<&'a str>,
) {
    if p.is_empty() && x.is_empty() && r.len() > clique.len() {
        *clique = r.clone();
    }

    let mut p_working = p.clone();
    while let Some(&v) = p_working.iter().next() {
        p_working.remove(v);
        let neighbors = edges.get(v).unwrap();
        r.insert(v);
        let p_intersect: HashSet<_> = p_working.intersection(neighbors).copied().collect();
        let x_intersect: HashSet<_> = x.intersection(neighbors).copied().collect();
        bron_kerbosch(r, &p_intersect, &mut x_intersect.clone(), edges, clique);

        r.remove(v);
        x.insert(v);
    }
}

// algorithm BronKerbosch1(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}
