use crate::grid::{Input, Grid, generate_input};

#[aoc_generator(day4)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

#[aoc(day4, part1)]
pub fn solve_part1(input_data: &Input) -> i32 {
    let line_len = input_data.line_len;
    let input = &input_data.input;
    let grid = Grid::new(input.clone(), line_len);
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'X' {
            i += 1;
            continue;
        }
        for j in 0 .. 8 {
            let is_xmas =
                grid.get_char(i, j, 1).0 == 'M' &&
                grid.get_char(i, j, 2).0 == 'A' &&
                grid.get_char(i, j, 3).0 == 'S';
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
    let grid = Grid::new(input.clone(), line_len);
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'A' {
            i += 1;
            continue;
        }
        let is_xmas_down =
            grid.get_char(i, 0, 1).0 == 'M' && grid.get_char(i, 7, 1).0 == 'S' ||
            grid.get_char(i, 7, 1).0 == 'M' && grid.get_char(i, 0, 1).0 == 'S';
        let is_xmas_up =
            grid.get_char(i, 5, 1).0 == 'M' && grid.get_char(i, 2, 1).0 == 'S' ||
            grid.get_char(i, 2, 1).0 == 'M' && grid.get_char(i, 5, 1).0 == 'S';
        xmas_count += (is_xmas_down && is_xmas_up) as i32;
        i += 1;
    }
    xmas_count
}
