pub fn star1() {
    let data: Vec<_> = include_str!("data.in")
        .bytes()
        .map(|u| (u - b'0') as usize)
        .collect();
    let mut disc: Vec<usize> = vec![];
    let mut expteced = 0;
    for index in (0..data.len() - 1).step_by(2) {
        expteced += data[index];
        disc.append(vec![index / 2; data[index]].as_mut());
        disc.append(vec![usize::MAX; data[index + 1]].as_mut());
    }
    disc.append(vec![(data.len() - 1) / 2; data[data.len() - 1]].as_mut());
    expteced += data[data.len() - 1];

    let mut res = 0;
    let mut back = disc.len() - 1;
    for front in 0..disc.len() {
        if front >= expteced {
            break;
        }
        if disc[front] != usize::MAX {
            res += disc[front] * front;
        } else {
            while disc[back] == usize::MAX {
                back -= 1;
            }
            res += disc[back] * front;
            back -= 1;
        }
    }
    println!("{}", res);
}
