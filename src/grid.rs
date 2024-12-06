use regex::Regex;
use std::sync::Arc;

fn _get_char(pos: i32, curr_pos: i32, line_offset: i32, input: Arc<String>, line_len: i32) -> char {
    if pos <= 0 || pos >= input.len() as i32 || curr_pos <= 0 || curr_pos >= input.len() as i32 {
        return 'o';
    }
    let curr_row = curr_pos / line_len;
    let other_row_start = (curr_row + line_offset) * line_len;
    let other_row_end = other_row_start + line_len - 1;
    if pos >= other_row_start && pos <= other_row_end {
        return input.chars().nth(pos as usize).unwrap();
    }
    'o'
}

pub fn get_closures(input: Arc<String>, line_len: i32) -> Vec<Box<dyn Fn(i32, i32) -> (char, i32)>> {
    let create_closure = |char_offset: i32, line_offset: i32| {
        let input = input.clone();
        Box::new(move |pos, distance| (_get_char(pos + char_offset * distance, pos + char_offset * (distance - 1), line_offset, input.clone(), line_len), pos + char_offset * distance))
    };
    let get_char: Vec<Box<dyn Fn(i32, i32) -> (char, i32)>> = vec![
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
    pub input: String,
    pub line_len: i32
}

pub fn generate_input(in_lines: &str) -> Input {
    let new_line_regex = Regex::new(r"\n").unwrap();
    Input {
        input: new_line_regex.replace_all(in_lines, "").to_string(),
        line_len: new_line_regex.find(in_lines).unwrap().start() as i32
    }
}
