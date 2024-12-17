pub fn star2() {
    let re = regex::Regex::new(r"[^0-9]+").unwrap();
    let input = re.split(include_str!("data.in")).skip(4);
    let program = input
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // Collapsed the loop but only works for my program so cheating
    // let mut a = 0;
    // let mut goal = vec![];
    // 'outer: for b in program.iter().rev() {
    //     goal.insert(0, *b);
    //     for test in 0..16 {
    //         let mut tmp = a + test;
    //         let mut out = vec![];
    //         while tmp != 0{
    //             out.push(((((tmp % 8) ^ 1) ^ 4) ^ (tmp >> ((tmp % 8) ^ 1))) % 8);
    //             tmp >>= 3;
    //         }
    //         if out == goal{
    //             a += test;
    //             a <<= 3;
    //             continue 'outer;
    //         }
    //     }
    // }
    // a >>= 3;

    let mut a = 0;
    let mut goal = vec![];
    'outer: for b in program.iter().rev() {
        goal.insert(0, *b);
        for test in 0..16 {
            let mut registers = [0, 1, 2, 3, a + test, 0, 0];
            let mut out = vec![];
            let mut inst = 0;
            while inst < program.len() {
                match program[inst] {
                    0 => registers[4] >>= registers[program[inst + 1]],
                    1 => registers[5] ^= program[inst + 1],
                    2 => registers[5] = registers[program[inst + 1]] % 8,
                    3 => {
                        if registers[4] != 0 {
                            inst = program[inst + 1];
                            continue;
                        }
                    }
                    4 => registers[5] ^= registers[6],
                    5 => out.push(registers[program[inst + 1]] % 8),
                    6 => registers[5] = registers[4] >> registers[program[inst + 1]],
                    7 => registers[6] = registers[4] >> registers[program[inst + 1]],
                    _ => break,
                }
                inst += 2;
            }
            if out == goal {
                println!("{}", test);
                a += test;
                a <<= 3;
                continue 'outer;
            }
        }
        break;
    }
    a >>= 3;

    println!("{}", a);
}
