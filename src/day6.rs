use regex::Regex;
use std::sync::Arc;
use crate::grid::{Input, get_closures, generate_input};

#[aoc_generator(day6)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

fn follow_path(input: &String, line_len: i32, start_pos: i32, start_direction: usize, block_pos: i32, mut on_pos: impl FnMut(char, i32, usize) -> bool) -> bool {
    let get_char = get_closures(Arc::new(input.clone()), line_len);
    let mut pos = start_pos;
    let mut direction = start_direction;
    loop {
        let (next, i) = get_char[direction](pos, 1);
        let prev_direction = direction;
        if next == '!' {
            break;
        } else if next == '#' || i == block_pos {
            direction = match direction {
                1 => 4,
                4 => 6,
                6 => 3,
                3 => 1,
                _ => 1
            }
        } else {
            pos = i;
        }
        if on_pos(next, i, prev_direction) {
            return true;
        }
    }
    false
}

#[aoc(day6, part1)]
pub fn solve_part1(input_data: &Input) -> i32 {
    let line_len = input_data.line_len;
    let input = &input_data.input;
    let pos = Regex::new(r"\^").unwrap().find(input).unwrap().start() as i32;
    let mut positions = vec![pos];
    follow_path(input, line_len, pos, 1, -1, |next, i, _| {
        if next != '#' && !positions.contains(&i) {
            positions.push(i);
        }
        false
    });
    positions.len() as i32
}

struct Collision {
    pos: i32,
    direction: usize
}

#[aoc(day6, part2)]
pub fn solve_part2(input_data: &Input) -> i32 {
    let line_len = input_data.line_len;
    let input = &input_data.input;
    let pos = Regex::new(r"\^").unwrap().find(input).unwrap().start() as i32;
    let mut positions = vec![];
    follow_path(input, line_len, pos, 1, -1, |next, i, _| {
        if next != '#' && i != pos {
            let mut collisions = vec![];
            let will_loop = follow_path(input, line_len, pos, 1, i, |test_next, test_i, test_dir| {
                if test_next == '#' || test_i == i {
                    if collisions.iter().any(|c: &Collision| c.pos == test_i && c.direction == test_dir) {
                        return true;
                    }
                    collisions.push(Collision {
                        pos: test_i,
                        direction: test_dir
                    });
                }
                false
            });
            if will_loop && !positions.contains(&i) {
                positions.push(i);
            }
        }
        false
    });
    positions.len() as i32
}
