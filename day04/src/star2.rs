pub fn star2() {
    let data: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut sum = 0;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if correct(
                &data,
                &[
                    (i.wrapping_sub(1), j.wrapping_sub(1)),
                    (i, j),
                    (i + 1, j + 1),
                ],
            ) && correct(
                &data,
                &[
                    (i.wrapping_sub(1), j + 1),
                    (i, j),
                    (i + 1, j.wrapping_sub(1)),
                ],
            ) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
fn correct(data: &[Vec<char>], index: &[(usize, usize)]) -> bool {
    let ans = ['M', 'A', 'S'];
    let mut f = true;
    let mut b = true;
    for i in 0..3 {
        if (index[i].0 >= data.len()) || (index[i].1 >= data[0].len()) {
            return false;
        }
        f &= data[index[i].0][index[i].1] == ans[i];
        b &= data[index[i].0][index[i].1] == ans[2 - i];
    }
    f || b
}
