pub fn star2() {
    let data: Vec<_> = include_str!("data.in")
        .bytes()
        .map(|u| (u - b'0') as usize)
        .collect();
    let mut disc: Vec<(usize, usize)> = data
        .windows(2)
        .step_by(2)
        .enumerate()
        .flat_map(|(i, v)| [(i, v[0]), (usize::MAX, v[1])])
        .collect();
    disc.push((data.len() / 2, data[data.len() - 1]));

    'outer: for index in (0..disc.len()).rev() {
        if disc[index].0 != usize::MAX {
            for free in 0..disc.len() {
                if index <= free {
                    continue;
                }
                let sb = disc[index].1;
                let sf = disc[free].1;
                if (disc[free].0 == usize::MAX) && (sf >= sb) {
                    if sf == sb {
                        disc.swap(index, free);
                    } else {
                        let sd = sf - sb;
                        let tmp = (usize::MAX, sd);
                        disc[free] = disc[index];
                        disc[index] = (usize::MAX, sb);
                        disc.insert(free + 1, tmp);
                    }
                    continue 'outer;
                }
            }
        }
    }
    let mut bit_disc: Vec<usize> = vec![];
    for (v, t) in disc {
        if v != usize::MAX {
            bit_disc.append(vec![v; t].as_mut());
        } else {
            bit_disc.append(vec![0; t].as_mut());
        }
    }
    let res = bit_disc
        .iter()
        .enumerate()
        .fold(0, |sum, (i, v)| sum + i * v);
    println!("{}", res);
}
