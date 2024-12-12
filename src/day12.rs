use crate::grid::{generate_input, Grid, Input};

#[aoc_generator(day12)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

struct Area {
    positions: Vec<i32>,
    perimeter: usize
}

fn compute_area(pos: i32, char: char, grid: &Grid, seen: &mut Vec<i32>, area: &mut Area) {
    seen.push(pos);
    area.positions.push(pos);
    let directions = vec![4, 6, 3, 1];
    for i in directions {
        let (next_char, next_pos) = grid.get_char(pos, i, 1);
        if next_char != char {
            area.perimeter += 1;
            continue;
        }
        if area.positions.contains(&next_pos) {
            continue;
        }
        compute_area(next_pos, char, grid, seen, area);
    }
}

fn count(input_data: &Input) -> usize {
    let Input { input, line_len } = input_data;
    let grid = Grid::new(input.clone(), *line_len);
    let mut total = 0;
    let mut seen: Vec<i32> = vec![];
    for (i, char) in input.chars().enumerate() {
        if seen.contains(&(i as i32)) {
            continue;
        }
        let mut area = Area {
            positions: vec![],
            perimeter: 0
        };
        compute_area(i as i32, char, &grid, &mut seen, &mut area);
        total += area.positions.len() * area.perimeter;
    }
    total
}

#[aoc(day12, part1)]
pub fn solve_part1(input_data: &Input) -> usize {
    count(input_data)
}
