use regex::Regex;

pub struct Calibration {
    total: i64,
    numbers: Vec<i64>
}

#[aoc_generator(day7)]
pub fn input_generator(in_lines: &str) -> Vec<Calibration> {
    Regex::new(r":").unwrap().replace_all(in_lines, "")
        .lines()
        .map(|l| {
            let mut nums: Vec<i64> = l.trim().split(" ").map(|d| d.parse().unwrap()).collect();
            Calibration {
                total: nums[0],
                numbers: nums.split_off(1)
            }
        }).collect()
}

fn get_total(input: &Vec<Calibration>, op_bit_width: u32) -> i64 {
    let max_op_count = 2_u32.pow(op_bit_width);
    input.into_iter().fold(0, |acc, c| {
        for bits in 0 .. max_op_count.pow(c.numbers.len() as u32 - 1) {
            let mut total = c.numbers[0];
            for num in 1 .. c.numbers.len() {
                total = match bits >> op_bit_width * (num as u32 - 1) & (max_op_count - 1) {
                    0 => total + c.numbers[num],
                    1 => total * c.numbers[num],
                    2 => format!("{}{}", total, c.numbers[num]).parse().unwrap(),
                    _ => 0
                };
                if total == 0 {
                    break;
                }
            }
            if total == c.total {
                return acc + total;
            }
        }
        acc
    })
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Calibration>) -> i64 {
    get_total(input, 1)
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Calibration>) -> i64 {
    get_total(input, 2)
}
