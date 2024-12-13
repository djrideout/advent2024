use regex::Regex;

#[aoc_generator(day13)]
pub fn input_generator(in_lines: &str) -> Vec<f64> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.find_iter(in_lines)
        .map(|a| a.as_str().parse().unwrap())
        .collect()
}

fn compute_cost(input: &mut Vec<f64>, offset_prize: bool) -> i64 {
    let prize_offset: f64 = match offset_prize {
        false => 0.0,
        true => 10000000000000.0
    };
    let mut total = 0;
    while input.len() > 0 {
        let py = prize_offset + input.pop().unwrap();
        let px = prize_offset + input.pop().unwrap();
        let by = input.pop().unwrap();
        let bx = input.pop().unwrap();
        let ay = input.pop().unwrap();
        let ax = input.pop().unwrap();
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
pub fn solve_part1(input: &Vec<f64>) -> i64 {
    compute_cost(&mut input.clone(), false)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<f64>) -> i64 {
    compute_cost(&mut input.clone(), true)
}
