use regex::Regex;

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

pub struct Grid {
    data: String,
    line_len: i32
}

impl Grid {
    pub fn new(data: String, line_len: i32) -> Grid {
        Grid {
            data,
            line_len
        }
    }

    pub fn is_inbounds(&self, pos: i32, curr_pos: i32, line_offset: i32) -> bool {
        if pos < 0 || pos >= self.data.len() as i32 || curr_pos < 0 || curr_pos >= self.data.len() as i32 {
            return false;
        }
        let curr_row = curr_pos / self.line_len;
        let other_row_start = (curr_row + line_offset) * self.line_len;
        let other_row_end = other_row_start + self.line_len - 1;
        if pos >= other_row_start && pos <= other_row_end {
            return true;
        }
        false
    }

    pub fn get_char(&self, pos: i32, direction: usize, distance: i32) -> (char, i32) {
        let (char_offset, line_offset) = match direction {
            0 => (-self.line_len - 1, -1), // Up-left
            1 => (-self.line_len, -1),     // Up
            2 => (-self.line_len + 1, -1), // Up-right
            3 => (-1, 0),                  // Left
            4 => (1, 0),                   // Right
            5 => (self.line_len - 1, 1),   // Down-left
            6 => (self.line_len, 1),       // Down
            7 => (self.line_len + 1, 1),   // Down-right
            _ => (0, 0)                    // Oh no
        };
        let char_pos = pos + char_offset * distance;
        if self.is_inbounds(char_pos, pos + char_offset * (distance - 1), line_offset) {
            return (self.data.chars().nth(char_pos as usize).unwrap(), char_pos);
        }
        ('!', char_pos)
    }
}

pub fn get_opposite_direction(direction: usize) -> usize {
    7 - direction
}

pub fn get_perpendicular_directions(direction: usize) -> [usize; 2] {
    match direction {
        0 => [2, 5],
        1 => [3, 4],
        2 => [0, 7],
        3 => [1, 6],
        4 => [1, 6],
        5 => [0, 7],
        6 => [3, 4],
        7 => [2, 5],
        _ => [0, 0]
    }
}
