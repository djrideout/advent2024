use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i64,
    y :i64
}

#[derive(Clone, Debug)]
pub struct Machine {
    a: Pair,
    b: Pair,
    prize: Pair
}

#[aoc_generator(day13)]
pub fn input_generator(in_lines: &str) -> Vec<Machine> {
    let mut machines = vec![];
    let mut machine = Machine {
        a: Pair {x: 0, y: 0},
        b: Pair {x: 0, y: 0},
        prize: Pair {x: 0, y: 0}
    };
    let mut count = 0;
    let re = Regex::new(r"X(:?\+|=)(\d+),\s*Y(\+|=)(\d+)").unwrap();
    in_lines
        .lines()
        .for_each(|l| {
            if let Some(captures) = re.captures(l) {
                let x = captures.get(2).map_or("", |m| m.as_str());
                let y = captures.get(4).map_or("", |m| m.as_str());
                match count % 4 {
                    0 => {
                        machine.a.x = x.parse().unwrap();
                        machine.a.y = y.parse().unwrap();
                    }
                    1 => {
                        machine.b.x = x.parse().unwrap();
                        machine.b.y = y.parse().unwrap();
                    }
                    2 => {
                        machine.prize.x = x.parse().unwrap();
                        machine.prize.y = y.parse().unwrap();
                        machines.push(machine.clone());
                    }
                    _ => {}
                }
            }
            count += 1;
        });
    machines
}

fn compute_cost(input: &Vec<Machine>, offset_prize: bool) -> i64 {
    let prize_offset: f64 = match offset_prize {
        false => 0.0,
        true => 10000000000000.0
    };
    let mut total = 0;
    for i in 0 .. input.len() {
        let machine = &input[i];
        let px = prize_offset + machine.prize.x as f64;
        let ax = machine.a.x as f64;
        let bx = machine.b.x as f64;
        let py = prize_offset + machine.prize.y as f64;
        let ay = machine.a.y as f64;
        let by = machine.b.y as f64;
        let b_presses = (px / ax - py / ay) / (bx / ax - by / ay);
        let a_presses = (px - bx * b_presses) / ax;
        let tolerance = 0.001_f64;
        if (a_presses - a_presses.round()).abs() <= tolerance && (b_presses - b_presses.round()).abs() <= tolerance {
            total += 3 * (a_presses.round() as i64) + (b_presses.round() as i64);
        }
    }
    total
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<Machine>) -> i64 {
    compute_cost(input, false)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<Machine>) -> i64 {
    compute_cost(input, true)
}
