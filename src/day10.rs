use crate::grid::{generate_input, Grid, Input};

#[aoc_generator(day10)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

fn test_path(pos: i32, direction: usize, height: i32, grid: &Grid, nines: &mut Vec<i32>, unique: bool) -> i32 {
    let (next_char, next_pos) = grid.get_char(pos, direction, 1);
    if next_char == '!' {
        return 0;
    }
    let next_height = next_char.to_digit(10).unwrap() as i32;
    if next_height != height + 1 {
        return 0;
    }
    if next_height == 9 {
        if !nines.contains(&next_pos) {
            if unique {
                nines.push(next_pos);
            }
            return 1;
        }
        return 0;
    }
    test_path(next_pos, 1, next_height, grid, nines, unique)
        + test_path(next_pos, 3, next_height, grid, nines, unique)
        + test_path(next_pos, 4, next_height, grid, nines, unique)
        + test_path(next_pos, 6, next_height, grid, nines, unique)
}

fn test_paths(input_data: &Input, unique: bool) -> i32 {
    let line_len = input_data.line_len;
    let input = &input_data.input;
    let grid = Grid::new(input.clone(), line_len);
    let mut sum = 0;
    for (i, char) in input.chars().enumerate() {
        if char != '0' {
            continue;
        }
        let mut nines = vec![];
        sum += test_path(i as i32, 1, 0, &grid, &mut nines, unique)
            + test_path(i as i32, 3, 0, &grid, &mut nines, unique)
            + test_path(i as i32, 4, 0, &grid, &mut nines, unique)
            + test_path(i as i32, 6, 0, &grid, &mut nines, unique)
    }
    sum
}

#[aoc(day10, part1)]
pub fn solve_part1(input_data: &Input) -> i32 {
    test_paths(input_data, true)
}

#[aoc(day10, part2)]
pub fn solve_part2(input_data: &Input) -> i32 {
    test_paths(input_data, false)
}
