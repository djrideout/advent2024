pub struct Input {
    a: u64,
    program: Vec<u64>
}

#[aoc_generator(day17)]
pub fn input_generator(in_lines: &str) -> Input {
    let lines: Vec<&str> = in_lines.lines().collect();
    Input {
        a: lines[0].split(": ").collect::<Vec<&str>>()[1].parse().unwrap(),
        program: lines[4].split(": ").collect::<Vec<&str>>()[1]
            .split(",").map(|d| d.parse().unwrap()).collect()
    }
}

fn run_program(a: u64, program: &Vec<u64>, test_output: bool) -> Vec<u64> {
    let mut a = a;
    let mut b = 0;
    let mut c = 0;
    let mut pc = 0_u64;
    let mut out: Vec<u64> = vec![];
    while (pc as usize) < program.len() {
        let opcode = program[pc as usize];
        let lit_operand = program[pc as usize + 1];
        let cmb_operand = match lit_operand {
            4 => a,
            5 => b,
            6 => c,
            _ => lit_operand
        };
        pc += 2;
        match opcode {
            0 => {
                // adv
                a = a / 2_u64.pow(cmb_operand as u32);
                if test_output {
                    return vec![a, b % 8];
                }
            },
            1 => {
                // bxl
                b = b ^ lit_operand;
            },
            2 => {
                // bst
                b = cmb_operand % 8;
            },
            3 => {
                // jnz
                if a != 0 {
                    pc = lit_operand;
                }
            },
            4 => {
                // bxc
                b = b ^ c;
            },
            5 => {
                // out
                out.push(cmb_operand % 8);
            },
            6 => {
                // bdv
                b = a / 2_u64.pow(cmb_operand as u32);
            },
            7 => {
                // cdv
                c = a / 2_u64.pow(cmb_operand as u32);
            },
            _ => {}
        }
    }
    out
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Input) -> String {
    run_program(input.a, &input.program, false)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let mut prev_a: Vec<u64> = vec![0];
    for digit in input.program.iter().rev() {
        let mut results: Vec<u64> = vec![];
        let mut i = prev_a.first().unwrap() << 3;
        let max = (prev_a.last().unwrap() + 1) << 3;
        while i < max {
            let out = run_program(i, &input.program, true);
            if prev_a.contains(&out[0]) && out[1] == *digit {
                results.push(i);
            }
            i += 1;
        }
        prev_a = results;
    }
    *prev_a.first().unwrap()
}
