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

fn print_map(map: &String, path: &Vec<(i32, usize)>, line_len: i32) {
    map.chars().enumerate().for_each(|(i, c)| {
        if i as i32 % line_len == 0 {
            println!();
        }
        if path.iter().any(|(path_pos, _)| i == (*path_pos) as usize) {
            print!("@");
        } else {
            print!("{c}");
        }
    });
    println!();
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let start_pos = Regex::new(r"S").unwrap().find(&input.input).unwrap().start() as i32;
    let end_pos = Regex::new(r"E").unwrap().find(&input.input).unwrap().start() as i32;
    let grid = Grid::new(input.input.clone(), input.line_len);
    let path = follow_path(&grid, start_pos, &vec![(end_pos, 0)], 0);
    let mut count = 0;
    for (i, (pos, _)) in path.iter().enumerate() {
        for test_direction in [1, 3, 4, 6] {
            let (test_char, test_pos) = grid.get_char(*pos, test_direction, 1);
            if test_char != '#' {
                continue;
            }
            for next_direction in [1, 3, 4, 6] {
                let mut test_map = input.input.clone();
                if next_direction == get_opposite_direction(test_direction) {
                    continue;
                }
                test_map.replace_range((test_pos as usize) .. (test_pos as usize) + 1, ".");
                let (next_char, next_pos) = grid.get_char(test_pos, next_direction, 1);
                if next_char == '#' || next_char == '!' {
                    continue;
                }
                test_map.replace_range((next_pos as usize) .. (next_pos as usize) + 1, ".");
                let test_grid = Grid::new(test_map.clone(), input.line_len);
                let test_path = follow_path(&test_grid, test_pos, &path, next_direction);
                if test_path.len() == 0 {
                    continue;
                }
                let last_test_pos = test_path.last().unwrap().0;
                let old_distance = path.iter().position(|(pos, _)| *pos == last_test_pos).unwrap() - i;
                let new_distance = test_path.len();
                let diff = new_distance as i32 - old_distance as i32;
                if diff <= -100 {
                    count += 1;
                }
            }
        }
    }
    count
}
