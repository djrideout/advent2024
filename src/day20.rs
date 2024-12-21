use std::collections::HashMap;
use regex::Regex;
use crate::grid::{generate_input, Grid, Input, get_opposite_direction};

#[aoc_generator(day20)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

fn follow_path(grid: &Grid, start_position: i32, end_positions: &Vec<(i32, usize)>, initial_direction: usize) -> Vec<(i32, usize)> {
    let mut positions = vec![(start_position, initial_direction)];
    'follow: loop {
        let (pos, direction) = positions.last().unwrap();
        if *pos != start_position && end_positions.iter().any(|(end_pos, _)| pos == end_pos) {
            return positions;
        }
        let dirs = match direction {
            3 => [3, 4, 6, 1],
            4 => [4, 6, 1, 3],
            6 => [6, 1, 3, 4],
            _ => [1, 3, 4, 6],
        };
        for next_direction in dirs {
            let (next_char, next_pos) = grid.get_char(*pos, next_direction, 1);
            if next_direction == get_opposite_direction(*direction) || next_char == '#' || next_char == '!' {
                continue;
            }
            positions.push((next_pos, next_direction));
            continue 'follow;
        }
        return vec![];
    }
}

fn get_possible_cheats(pos: i32, cheat_length: i32, line_len: i32, grid: &Grid) -> Vec<(i32, i32)> {
    let step_count = cheat_length - 1;
    let mut positions = vec![];
    let x0 = pos % line_len;
    let y0 = pos / line_len;
    for x in x0 - step_count .. x0 + step_count + 1 {
        if x < 0 || x >= line_len {
            continue;
        }
        for y in y0 - step_count .. y0 + step_count + 1 {
            let linear = x + line_len * y;
            let (char, _) = grid.get_char(linear, 0, 0);
            if char != '#' && char != '!' && (x - x0).abs() + (y - y0).abs() < cheat_length {
                positions.push((linear, (x - x0).abs() + (y - y0).abs()));
            }
        }
    }
    positions
}

fn count_cheats(input: &Input, cheat_length: i32) -> i32 {
    let start_pos = Regex::new(r"S").unwrap().find(&input.input).unwrap().start() as i32;
    let end_pos = Regex::new(r"E").unwrap().find(&input.input).unwrap().start() as i32;
    let grid = Grid::new(input.input.clone(), input.line_len);
    let path = follow_path(&grid, start_pos, &vec![(end_pos, 0)], 0);
    let mut count = 0;
    for (i, (pos, _)) in path.iter().enumerate() {
        let mut cheats = HashMap::new();
        for test_direction in [1, 3, 4, 6] {
            let (_, test_pos) = grid.get_char(*pos, test_direction, 1);
            let positions = get_possible_cheats(test_pos, cheat_length, input.line_len, &grid);
            for (end_pos, steps) in &positions {
                let old_dist_to_end = (path.len() - i) as i32;
                let new_dist_to_end = steps + (path.len() - path.iter().position(|(pos, _)| *pos == *end_pos).unwrap()) as i32;
                let diff = new_dist_to_end - old_dist_to_end;
                if !cheats.contains_key(&end_pos.clone()) {
                    cheats.insert(end_pos.clone(), diff);
                } else {
                    let old = cheats.get(&end_pos.clone()).unwrap();
                    if diff <= *old {
                        cheats.insert(end_pos.clone(), diff);
                    }
                }
            }
        }
        for (_, diff) in cheats {
            if diff <= -100 {
                count += 1;
            }
        }
    }
    count as i32
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    count_cheats(input, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &Input) -> i32 {
    count_cheats(input, 20)
}
