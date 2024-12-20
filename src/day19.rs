use std::collections::HashMap;

pub struct Input {
    towels: Vec<String>,
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
                towels = l.split(", ").map(|s| s.to_owned()).collect();
            } else if l.len() > 0 {
                designs.push(l.to_owned());
            }
        });
    Input {
        towels,
        designs
    }
}

fn _combo_count(design: String, towels: &Vec<String>, results: &mut HashMap<String, i64>) -> i64 {
    if results.contains_key(&design) {
        return *results.get(&design).unwrap();
    }
    let mut count = 0;
    for towel in towels {
        if design.starts_with(towel) {
            count += _combo_count(design.clone().split_off(towel.len()), towels, results);
        }
    }
    if design.len() == 0 {
        results.insert(design, 1);
        1
    } else {
        results.insert(design, count);
        count
    }
}

fn combo_count(design: String, towels: &Vec<String>) -> i64 {
    _combo_count(design, towels, &mut HashMap::new())
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    input.designs.iter().fold(0, |accu, design| {
        accu + (combo_count(design.clone(), &input.towels) > 0) as i64
    })
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    input.designs.iter().fold(0, |accu, design| {
        accu + combo_count(design.clone(), &input.towels)
    })
}
