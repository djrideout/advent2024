fn _get_char(pos: i32, curr_pos: i32, line_offset: i32, input: &str, line_len: i32) -> char {
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

#[aoc(day4, part1)]
pub fn solve_part1(in_lines: &str) -> i32 {
    let mut line_len = 0;
    let mut input = String::new();
    in_lines
        .lines()
        .for_each(|l| {
            line_len = l.len() as i32;
            input.push_str(l);
        });
    let offsets = vec![
        -1,
        1,
        -line_len - 1,
        -line_len,
        -line_len + 1,
        line_len + 1,
        line_len,
        line_len - 1
    ];
    let get_char: Vec<Box<dyn Fn(i32, i32) -> char>> = vec![
        Box::new(|pos, distance| _get_char(pos + offsets[0] * distance, pos + offsets[0] * (distance - 1), 0, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[1] * distance, pos + offsets[1] * (distance - 1), 0, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[2] * distance, pos + offsets[2] * (distance - 1), -1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[3] * distance, pos + offsets[3] * (distance - 1), -1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[4] * distance, pos + offsets[4] * (distance - 1), -1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[5] * distance, pos + offsets[5] * (distance - 1), 1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[6] * distance, pos + offsets[6] * (distance - 1), 1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[7] * distance, pos + offsets[7] * (distance - 1), 1, &input, line_len)),
    ];
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'X' {
            i += 1;
            continue;
        }
        for j in 0 .. offsets.len() {
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
pub fn solve_part2(in_lines: &str) -> i32 {
    let mut line_len = 0;
    let mut input = String::new();
    in_lines
        .lines()
        .for_each(|l| {
            line_len = l.len() as i32;
            input.push_str(l);
        });
    let offsets = vec![
        -1,
        1,
        -line_len - 1,
        -line_len,
        -line_len + 1,
        line_len + 1,
        line_len,
        line_len - 1
    ];
    let get_char: Vec<Box<dyn Fn(i32, i32) -> char>> = vec![
        Box::new(|pos, distance| _get_char(pos + offsets[0] * distance, pos + offsets[0] * (distance - 1), 0, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[1] * distance, pos + offsets[1] * (distance - 1), 0, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[2] * distance, pos + offsets[2] * (distance - 1), -1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[3] * distance, pos + offsets[3] * (distance - 1), -1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[4] * distance, pos + offsets[4] * (distance - 1), -1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[5] * distance, pos + offsets[5] * (distance - 1), 1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[6] * distance, pos + offsets[6] * (distance - 1), 1, &input, line_len)),
        Box::new(|pos, distance| _get_char(pos + offsets[7] * distance, pos + offsets[7] * (distance - 1), 1, &input, line_len)),
    ];
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'A' {
            i += 1;
            continue;
        }
        let is_xmas_down =
            get_char[2](i, 1) == 'M' && get_char[5](i, 1) == 'S' ||
            get_char[5](i, 1) == 'M' && get_char[2](i, 1) == 'S';
        let is_xmas_up =
            get_char[4](i, 1) == 'M' && get_char[7](i, 1) == 'S' ||
            get_char[7](i, 1) == 'M' && get_char[4](i, 1) == 'S';
        xmas_count += (is_xmas_down && is_xmas_up) as i32;
        i += 1;
    }
    xmas_count
}
