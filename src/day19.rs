use std::cmp::Ordering;

pub struct Input {
    unique_towels: Vec<String>,
    designs: Vec<String>
}

#[aoc_generator(day19)]
pub fn generate_input(input: &str) -> Input {
    let mut towels = vec![];
    let mut designs = vec![];
    input
        .lines()
        .enumerate()
        .for_each(|(i, l)| {
            if i == 0 {
                towels.append(&mut l.split(", ").map(|s| s.to_string()).collect());
            } else if l.len() > 0 {
                designs.push(l.to_string());
            }
        });
    towels.sort_by(|a, b| {
        if a.len() < b.len() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let mut unique_towels = vec![];
    for towel in towels.clone() {
        if !test_design(&towel, &unique_towels) {
            unique_towels.push(towel);
        }
    }
    Input {
        unique_towels,
        designs
    }
}

fn test_design(design: &String, towels: &Vec<String>) -> bool {
    let mut is_possible = false;
    for towel in towels {
        if design.starts_with(towel) {
            let next_design = design.clone().split_off(towel.len());
            if next_design.len() == 0 {
                return true;
            }
            is_possible |= test_design(&next_design,  towels);
        }
    }
    is_possible
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let mut possible_count = 0;
    for design in &input.designs {
        possible_count += test_design(design, &input.unique_towels) as i32;
    }
    possible_count
}
