use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Input {
    prefix_of: HashMap<String, Vec<String>>,
    atomic_towels: Vec<String>,
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
    let mut atomic_towels = vec![];
    let mut prefix_of = HashMap::new();
    for towel in towels.clone() {
        let sections = atomic_sections(&towel, &atomic_towels);
        if sections.len() == 0 {
            prefix_of.insert(towel.clone(), vec![]);
            atomic_towels.push(towel);
        } else {
            prefix_of.get_mut(&sections[0]).unwrap().push(towel);
        }
    }
    Input {
        prefix_of,
        atomic_towels,
        designs
    }
}

fn _atomic_sections(design: &String, towels: &Vec<String>, sections: &mut Vec<String>) -> bool {
    let mut is_possible = false;
    for towel in towels {
        if design.starts_with(towel) {
            sections.push(towel.clone());
            let next_design = design.clone().split_off(towel.len());
            if next_design.len() == 0 {
                return true;
            }
            is_possible |= _atomic_sections(&next_design,  towels, sections);
        }
    }
    is_possible
}

fn atomic_sections(design: &String, towels: &Vec<String>) -> Vec<String> {
    let mut sections: Vec<String> = vec![];
    let result = _atomic_sections(design, towels, &mut sections);
    if result {
        sections
    } else {
        vec![]
    }
}

fn possible_combos(design: String, towels: &Vec<String>, prefix_of: &HashMap<String, Vec<String>>, results: &mut HashMap<String, i64>) -> i64 {
    if results.contains_key(&design) {
        return *results.get(&design).unwrap();
    }
    let mut count = 0;
    for atom in towels {
        if design.starts_with(atom) {
            count += possible_combos(design.clone().split_off(atom.len()), towels, prefix_of, results);
            let prefixes = prefix_of.get(atom).unwrap();
            for prefix in prefixes {
                if design.starts_with(prefix) {
                    count += possible_combos(design.clone().split_off(prefix.len()), towels, prefix_of, results);
                }
            }
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

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let mut possible_count = 0;
    for design in &input.designs {
        let sections = atomic_sections(design, &input.atomic_towels);
        possible_count += (sections.len() > 0) as i64;
    }
    possible_count
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    let mut possible_count = 0;
    for design in &input.designs {
        let sections = atomic_sections(design, &input.atomic_towels);
        if sections.len() > 0 {
            possible_count += possible_combos(design.clone(), &input.atomic_towels, &input.prefix_of, &mut HashMap::new());
        }
    }
    possible_count
}
