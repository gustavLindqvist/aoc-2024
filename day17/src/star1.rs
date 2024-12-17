pub fn star1() {
    let re = regex::Regex::new(r"[^0-9]+").unwrap();
    let mut input = re.split(include_str!("data.in")).skip(1);
    let mut registers = [0, 1, 2, 3, 0, 0, 0];
    registers[4] = input.next().unwrap().parse::<usize>().unwrap();
    registers[5] = input.next().unwrap().parse::<usize>().unwrap();
    registers[6] = input.next().unwrap().parse::<usize>().unwrap();
    let program = input
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

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
    println!(
        "{}",
        out.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
