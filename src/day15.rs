use std::collections::VecDeque;
use regex::Regex;
use crate::grid::Grid;

pub struct Input {
    map_str: String,
    map: Vec<char>,
    line_len: i32,
    movements: Vec<usize>
}

pub fn input_generator(in_lines: &str, double_width: bool) -> Input {
    let movements = Regex::new(r"<|v|>|\^").unwrap()
        .find_iter(in_lines)
        .map(|a| match a.as_str() {
            "<" => 3,
            "v" => 6,
            ">" => 4,
            "^" => 1,
            _ => 0
        })
        .collect();
    let mut line_len = Regex::new(r"\n").unwrap()
        .find(in_lines).unwrap().start() as i32;
    let mut input_str = Regex::new(r"<|v|>|\^|\n").unwrap()
        .replace_all(in_lines, "")
        .to_string();
    if double_width {
        line_len *= 2;
        input_str = Regex::new(r"#").unwrap()
            .replace_all(&input_str, "##")
            .to_string();
        input_str = Regex::new(r"O").unwrap()
            .replace_all(&input_str, "[]")
            .to_string();
        input_str = Regex::new(r"\.").unwrap()
            .replace_all(&input_str, "..")
            .to_string();
        input_str = Regex::new(r"@").unwrap()
            .replace_all(&input_str, "@.")
            .to_string();
    }
    let input = input_str
        .chars()
        .collect();
    Input {
        map_str: input_str,
        map: input,
        line_len,
        movements
    }
}

fn move_object(pos: usize, direction: usize, grid: &Grid, map: &mut Vec<char>, swaps: &mut VecDeque<(usize, usize)>, double_width: bool) -> bool {
    let (_, next_pos) = grid.get_char(pos as i32, direction, 1);
    let next = next_pos as usize;
    let mut can_move = true;
    if double_width {
        // Direction is always vertical here
        if swaps.iter().any(|(swapped_pos, _)| *swapped_pos == pos) {
            return true;
        }
        if map[next] == '#' || map[next + 1] == '#' {
            return false;
        }
        if map[next] == '[' {
            can_move &= move_object(next, direction, grid, map, swaps, double_width);
        } else if map[next] == ']' {
            can_move &= move_object(next - 1, direction, grid, map, swaps, double_width);
        }
        if map[next + 1] == '[' {
            can_move &= move_object(next + 1, direction, grid, map, swaps, double_width);
        }
        if can_move {
            swaps.push_back((pos, next));
            swaps.push_back((pos + 1, next + 1));
        }
    } else {
        if map[next] == '#' {
            return false;
        }
        let is_vertical = direction == 1 || direction == 6;
        if is_vertical && (map[next] == '[' || map[next] == ']') {
            can_move &= match map[next] {
                '[' => move_object(next, direction, grid, map, swaps, true),
                ']' => move_object(next - 1, direction, grid, map, swaps, true),
                _ => false
            };
        } else if map[next] != '.' {
            can_move &= move_object(next, direction, grid, map, swaps, false);
        }
        if can_move {
            swaps.push_back((pos, next));
        }
    }
    can_move
}

fn do_movements(in_lines: &str, double_width: bool) -> usize {
    let input = input_generator(in_lines, double_width);
    let grid = Grid::new(input.map_str, input.line_len);
    let mut map = input.map;
    for direction in input.movements {
        let robot_pos = map.iter().position(|c| *c == '@').unwrap();
        let mut swaps = VecDeque::new();
        if move_object(robot_pos, direction, &grid, &mut map, &mut swaps, false) {
            while swaps.len() > 0 {
                let (pos, next) = swaps.pop_front().unwrap();
                let tmp = map[pos];
                map[pos] = map[next];
                map[next] = tmp;
            }
        }
    }
    map.iter().enumerate().fold(0, |accu: usize, (i, char)| {
        if *char == 'O' || *char == '[' {
            let x = i % input.line_len as usize;
            let y = i / input.line_len as usize;
            accu + 100 * y + x
        } else {
            accu
        }
    })
}

#[aoc(day15, part1)]
pub fn solve_part1(in_lines: &str) -> usize {
    do_movements(in_lines, false)
}

#[aoc(day15, part2)]
pub fn solve_part2(in_lines: &str) -> usize {
    do_movements(in_lines, true)
}
