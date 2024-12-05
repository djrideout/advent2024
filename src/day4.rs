use regex::Regex;
use std::sync::Arc;

fn _get_char(pos: i32, curr_pos: i32, line_offset: i32, input: Arc<String>, line_len: i32) -> char {
    if pos <= 0 || pos >= input.len() as i32 || curr_pos <= 0 || curr_pos >= input.len() as i32 {
        return '.';
    }
    let curr_row = curr_pos / line_len;
    let other_row_start = (curr_row + line_offset) * line_len;
    let other_row_end = other_row_start + line_len - 1;
    if pos >= other_row_start && pos <= other_row_end {
        return input.chars().nth(pos as usize).unwrap();
    }
    '.'
}

fn get_closures(input: Arc<String>, line_len: i32) -> Vec<Box<dyn Fn(i32, i32) -> char>> {
    let create_closure = |char_offset: i32, line_offset: i32| {
        let input = input.clone();
        Box::new(move |pos, distance| _get_char(pos + char_offset * distance, pos + char_offset * (distance - 1), line_offset, input.clone(), line_len))
    };
    let get_char: Vec<Box<dyn Fn(i32, i32) -> char>> = vec![
        create_closure(-line_len - 1, -1), // Up-left
        create_closure(-line_len, -1),     // Up
        create_closure(-line_len + 1, -1), // Up-right
        create_closure(-1, 0),             // Left
        create_closure(1, 0),              // Right
        create_closure(line_len - 1, 1),   // Down-left
        create_closure(line_len, 1),       // Down
        create_closure(line_len + 1, 1),   // Down-right
    ];
    return get_char;
}

pub struct Input {
    input: String,
    line_len: i32
}

#[aoc_generator(day4)]
pub fn input_generator(in_lines: &str) -> Input {
    let new_line_regex = Regex::new(r"\n").unwrap();
    Input {
        input: new_line_regex.replace_all(in_lines, "").to_string(),
        line_len: new_line_regex.find(in_lines).unwrap().start() as i32
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input_data: &Input) -> i32 {
    let line_len = input_data.line_len;
    let input = &input_data.input;
    let get_char = get_closures(Arc::new(input.clone()), line_len);
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'X' {
            i += 1;
            continue;
        }
        for j in 0 .. 8 {
            let is_xmas =
                get_char[j](i, 1) == 'M' &&
                get_char[j](i, 2) == 'A' &&
                get_char[j](i, 3) == 'S';
            xmas_count += is_xmas as i32;
        }
        i += 1;
    }
    xmas_count
}

#[aoc(day4, part2)]
pub fn solve_part2(input_data: &Input) -> i32 {
    let line_len = input_data.line_len;
    let input = &input_data.input;
    let get_char = get_closures(Arc::new(input.clone()), line_len);
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'A' {
            i += 1;
            continue;
        }
        let is_xmas_down =
            get_char[0](i, 1) == 'M' && get_char[7](i, 1) == 'S' ||
            get_char[7](i, 1) == 'M' && get_char[0](i, 1) == 'S';
        let is_xmas_up =
            get_char[5](i, 1) == 'M' && get_char[2](i, 1) == 'S' ||
            get_char[2](i, 1) == 'M' && get_char[5](i, 1) == 'S';
        xmas_count += (is_xmas_down && is_xmas_up) as i32;
        i += 1;
    }
    xmas_count
}
