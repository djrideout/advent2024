fn is_prev_line(pos: i32, curr_pos: i32, line_len: i32) -> bool {
    let curr_row = curr_pos / line_len;
    let prev_row_start = (curr_row - 1) * line_len;
    let prev_row_end = prev_row_start + line_len - 1;
    pos >= prev_row_start && pos <= prev_row_end
}

fn is_curr_line(pos: i32, curr_pos: i32, line_len: i32) -> bool {
    let curr_row_start = (curr_pos / line_len) * line_len;
    let curr_row_end = curr_row_start + line_len - 1;
    pos >= curr_row_start && pos <= curr_row_end
}

fn is_next_line(pos: i32, curr_pos: i32, line_len: i32) -> bool {
    let curr_row = curr_pos / line_len;
    let next_row_start = (curr_row + 1) * line_len;
    let next_row_end = next_row_start + line_len - 1;
    pos >= next_row_start && pos <= next_row_end
}

fn is_inbounds(pos: i32, input: &str) -> bool {
    pos >= 0 && pos < input.len() as i32
}

fn get_char(input: &str, pos: i32) -> char {
    input.chars().nth(pos as usize).unwrap()
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
    let tests: Vec<Box<dyn Fn(i32, i32) -> bool>> = vec![
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_curr_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_curr_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_prev_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_prev_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_prev_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_next_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_next_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_next_line(pos, curr_pos, line_len))
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
                tests[j](i + offsets[j] * 1, i + offsets[j] * 0) && get_char(&input, i + offsets[j] * 1) == 'M' &&
                tests[j](i + offsets[j] * 2, i + offsets[j] * 1) && get_char(&input, i + offsets[j] * 2) == 'A' &&
                tests[j](i + offsets[j] * 3, i + offsets[j] * 2) && get_char(&input, i + offsets[j] * 3) == 'S';
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
    let tests: Vec<Box<dyn Fn(i32, i32) -> bool>> = vec![
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_curr_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_curr_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_prev_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_prev_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_prev_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_next_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_next_line(pos, curr_pos, line_len)),
        Box::new(|pos, curr_pos| is_inbounds(pos, &input) && is_inbounds(curr_pos, &input) && is_next_line(pos, curr_pos, line_len))
    ];
    let mut xmas_count = 0;
    let mut i = 0;
    for char in input.chars() {
        if char != 'A' {
            i += 1;
            continue;
        }
        let is_xmas_down =
            tests[2](i + offsets[2], i) && get_char(&input, i + offsets[2]) == 'M' &&
            tests[5](i + offsets[5], i) && get_char(&input, i + offsets[5]) == 'S'
            ||
            tests[5](i + offsets[5], i) && get_char(&input, i + offsets[5]) == 'M' &&
            tests[2](i + offsets[2], i) && get_char(&input, i + offsets[2]) == 'S';
        let is_xmas_up =
            tests[4](i + offsets[4], i) && get_char(&input, i + offsets[4]) == 'M' &&
            tests[7](i + offsets[7], i) && get_char(&input, i + offsets[7]) == 'S'
            ||
            tests[7](i + offsets[7], i) && get_char(&input, i + offsets[7]) == 'M' &&
            tests[4](i + offsets[4], i) && get_char(&input, i + offsets[4]) == 'S';
        xmas_count += (is_xmas_down && is_xmas_up) as i32;
        i += 1;
    }
    xmas_count
}
