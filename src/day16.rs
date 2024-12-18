use regex::Regex;
use crate::grid::{Input, generate_input, dijkstra};

#[aoc_generator(day16)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let start_pos = Regex::new(r"S").unwrap().find(&input.input).unwrap().start() as i32;
    let end_pos = Regex::new(r"E").unwrap().find(&input.input).unwrap().start() as i32;
    let result = dijkstra(&input.input, input.line_len, start_pos, end_pos, 2000, 1000);
    *result.min_distances.get(&end_pos).unwrap()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let start_pos = Regex::new(r"S").unwrap().find(&input.input).unwrap().start() as i32;
    let end_pos = Regex::new(r"E").unwrap().find(&input.input).unwrap().start() as i32;
    let result = dijkstra(&input.input, input.line_len, start_pos, end_pos, 2000, 1000);
    result.positions.len()
}
