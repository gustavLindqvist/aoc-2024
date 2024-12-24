use std::collections::HashSet;
pub fn star1() {
    let mut edges = [[false; 26 * 26]; 26 * 26];
    let mut nodes = HashSet::new();
    let data = include_bytes!("data.in");
    for start in (0..data.len()).step_by(6) {
        let n1 = hash(&data[start..=(start + 1)]);
        let n2 = hash(&data[(start + 3)..=(start + 4)]);
        edges[n1][n2] = true;
        edges[n2][n1] = true;
        nodes.insert(n1);
        nodes.insert(n2);
    }
    let nodes: Vec<_> = nodes.into_iter().collect();
    let mut res = 0;
    for i1 in 0..nodes.len() {
        for i2 in i1..nodes.len() {
            for i3 in i2..nodes.len() {
                let (n1, n2, n3) = (nodes[i1], nodes[i2], nodes[i3]);
                if edges[n1][n2]
                    && edges[n1][n3]
                    && edges[n2][n3]
                    && (start_t(n1) || start_t(n2) || start_t(n3))
                {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}

fn hash(s: &[u8]) -> usize {
    (s[0] - b'a') as usize * 26 + (s[1] - b'a') as usize
}

fn start_t(s: usize) -> bool {
    s / 26 == (b't' - b'a') as usize
}
