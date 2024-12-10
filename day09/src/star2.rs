use std::collections::BinaryHeap;
pub fn star2() {
    let disk: Vec<_> = include_str!("data.in")
        .bytes()
        .map(|b| (b - b'0') as usize)
        .collect();
    let mut b = 0;
    let mut free: Vec<_> = (0..10).map(|_| BinaryHeap::new()).collect();
    let triangle = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
    let mut res = 0;


    for (index, &size) in disk.iter().enumerate() {
        if index % 2 == 1 && size > 0 {
            free[size].push(-b);
        }
        b += size as i64;
    }

    for (index, &size) in disk.iter().enumerate().rev() {
        b -= size as i64;
        if index % 2 == 1 {
            continue;
        }

        let mut next_b = b as usize;
        let mut next_index = usize::MAX;

        for (i,b) in free.iter().enumerate().skip(size) {
            if let Some(&first) = b.peek() {
                if ((-first) as usize) < next_b {
                    next_b = (-first) as usize;
                    next_index = i;
                }
            }
        }

        if next_index != usize::MAX {
            free[next_index].pop();
            if size < next_index {
                free[next_index - size].push(-((next_b + size)as i64));
            }
        }
        res += (index / 2) * (next_b * size + triangle[size]);
    }

    println!("{}", res);
}
