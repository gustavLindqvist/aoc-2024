pub fn star2() {
    let re = regex::Regex::new(r"[^0-9]+").unwrap();
    let input = re.split(include_str!("data.in")).skip(4);
    let program = input
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = program.len();
    let mut q: Vec<(usize, usize)> = vec![(0, 0)];
    let mut sol = vec![];
    while let Some((a, b)) = q.pop() {
        if b >= program.len() {
            sol.push(a >> 3);
            continue;
        }
        let goal = &program[(n - b - 1)..n];
        for test in 0..(1 << 3) {
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
                    _ => unreachable!(),
                }
                inst += 2;
            }
            if out == goal {
                q.push(((a | test) << 3, b + 1));
            }
        }
    }
    println!("{}",sol.iter().min().unwrap());
}
