use std::collections::HashMap;
use crate::grid::{Input as GridInput, generate_input, Grid};

pub struct Input {
    input: String,
    line_len: i32,
    combos: Vec<(Vec<i32>, usize, usize)>
}

#[aoc_generator(day8)]
pub fn input_generator(in_lines: &str) -> Input {
    let grid_input: GridInput = generate_input(in_lines);
    let combos: Vec<(Vec<i32>, usize, usize)> = grid_input.input
        .chars()
        .enumerate()
        .fold(HashMap::new(), |mut freqs: HashMap<char, Vec<i32>>, (i, char)| {
            if char != '.' {
                freqs.entry(char).and_modify(|vec| vec.push(i as i32)).or_insert(vec![i as i32]);
            }
            freqs
        })
        .into_iter()
        .flat_map(|(_, vec)| (0..vec.len()).map(move |i| (vec.clone(), i)))
        .flat_map(|(vec, i)| (0..vec.len()).map(move |j| (vec.clone(), i, j)))
        .collect();
    Input {
        input: grid_input.input,
        line_len: grid_input.line_len,
        combos
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input_data: &Input) -> i32 {
    let Input { input, line_len, combos } = input_data;
    let grid = Grid::new(input.clone(), *line_len);
    let mut antinodes: Vec<i32> = vec![];
    for (vec, i, j) in combos {
        if *i == *j {
            continue;
        }
        let pos = vec[*i] + (vec[*i] - vec[*j]);
        if !antinodes.contains(&pos) && grid.is_inbounds(pos, vec[*i], vec[*i] / line_len - vec[*j] / line_len) {
            antinodes.push(pos);
        }
    };
    antinodes.len() as i32
}

#[aoc(day8, part2)]
pub fn solve_part2(input_data: &Input) -> i32 {
    let Input { input, line_len, combos } = input_data;
    let grid = Grid::new(input.clone(), *line_len);
    let mut antinodes: Vec<i32> = vec![];
    for (vec, i, j) in combos {
        if *i == *j {
            continue;
        }
        if !antinodes.contains(&vec[*i]) {
            antinodes.push(vec[*i]);
        }
        let mut inbounds = true;
        let mut mult = 1;
        while inbounds {
            let pos = vec[*i] + mult * (vec[*i] - vec[*j]);
            inbounds = grid.is_inbounds(pos, vec[*i], mult * (vec[*i] / line_len - vec[*j] / line_len));
            if !antinodes.contains(&pos) && inbounds {
                antinodes.push(pos);
            }
            mult += 1;
        }
    }
    antinodes.len() as i32
}
