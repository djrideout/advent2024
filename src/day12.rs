use crate::grid::{generate_input, Grid, Input};

#[aoc_generator(day12)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

struct FenceSegment {
    direction: usize,
    positions: Vec<i32>
}

struct Area<'a> {
    grid: &'a Grid,
    positions: Vec<i32>,
    segments: Vec<FenceSegment>
}

impl Area<'_> {
    pub fn add_fence(&mut self, pos: i32, direction: usize) {
        let test_directions: Vec<usize> = match direction {
            1 => vec![3, 4],
            6 => vec![3, 4],
            3 => vec![1, 6],
            4 => vec![1, 6],
            _ => vec![]
        };
        let mut segments: Vec<usize> = vec![];
        for test_direction in test_directions {
            let (_, next_pos) = self.grid.get_char(pos, test_direction, 1);
            let segment_pos = self.segments.iter().position(|seg| seg.direction == direction && seg.positions.contains(&next_pos));
            if segment_pos.is_some() {
                segments.push(segment_pos.unwrap());
            }
        }
        let mut new_segment = FenceSegment {
            direction,
            positions: vec![pos]
        };
        segments.sort_by(|a, b| b.cmp(a));
        for old_pos in segments {
            new_segment.positions.append(&mut self.segments[old_pos].positions);
            self.segments.remove(old_pos);
        }
        self.segments.push(new_segment);
    }

    pub fn count(&self) -> usize {
        self.segments.len()
    }

    pub fn perimeter(&self) -> usize {
        self.segments.iter().map(|segment| segment.positions.len()).sum()
    }
}

fn compute_area(pos: i32, char: char, grid: &Grid, seen: &mut Vec<i32>, area: &mut Area) {
    seen.push(pos);
    area.positions.push(pos);
    let directions = vec![4, 6, 3, 1];
    for i in directions {
        let (next_char, next_pos) = grid.get_char(pos, i, 1);
        if next_char != char {
            area.add_fence(pos, i);
            continue;
        }
        if area.positions.contains(&next_pos) {
            continue;
        }
        compute_area(next_pos, char, grid, seen, area);
    }
}

fn count(input_data: &Input, discount: bool) -> usize {
    let Input { input, line_len } = input_data;
    let grid = Grid::new(input.clone(), *line_len);
    let mut total = 0;
    let mut seen: Vec<i32> = vec![];
    for (i, char) in input.chars().enumerate() {
        if seen.contains(&(i as i32)) {
            continue;
        }
        let mut area = Area {
            grid: &grid,
            positions: vec![],
            segments: vec![]
        };
        compute_area(i as i32, char, &grid, &mut seen, &mut area);
        if discount {
            total += area.positions.len() * area.count();
        } else {
            total += area.positions.len() * area.perimeter();
        }
    }
    total
}

#[aoc(day12, part1)]
pub fn solve_part1(input_data: &Input) -> usize {
    count(input_data, false)
}

#[aoc(day12, part2)]
pub fn solve_part2(input_data: &Input) -> usize {
    count(input_data, true)
}
