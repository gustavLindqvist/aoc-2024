pub fn star1() {
    let data: Vec<Vec<_>> = include_str!("data.in").lines().map(|l|l.chars().collect()).collect();

    let mut sum = 0;
    for i in 0..data.len(){
        for j in 0..data[0].len(){
            sum += correct(&data, &[(i,j),(i+1,j),(i+2,j),(i+3,j)]);
            sum += correct(&data, &[(i,j),(i,j+1),(i,j+2),(i,j+3)]);
            sum += correct(&data, &[(i,j),(i+1,j+1),(i+2,j+2),(i+3,j+3)]);
            sum += correct(&data, &[(i,j),(i+1,j.wrapping_sub(1)),(i+2,j.wrapping_sub(2)),(i+3,j.wrapping_sub(3))]);
        }
    }
    println!("{}", sum);
}

fn correct(data: &[Vec<char>], index: &[(usize, usize)]) -> usize{
    let ans = ['X','M','A','S'];
    let mut f = true;
    let mut b = true;
    for i in 0..4{
        if (index[i].0 >= data.len()) || (index[i].1 >= data[0].len()){
            return 0;
        }
        f &= data[index[i].0][index[i].1] == ans[i];
        b &= data[index[i].0][index[i].1] == ans[3-i];
    }
    if f || b {1} else {0}
}