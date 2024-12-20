use std::collections::HashMap;

pub struct Input {
    parent_towels: HashMap<String, Vec<String>>,
    atomic_towels: Vec<String>,
    designs: Vec<String>
}

fn _combo_count(design: String, atomic_towels: &Vec<String>, parent_towels: &HashMap<String, Vec<String>>, results: &mut HashMap<String, i64>) -> i64 {
    if results.contains_key(&design) {
        return *results.get(&design).unwrap();
    }
    let mut count = 0;
    for atom in atomic_towels {
        if design.starts_with(atom) {
            count += _combo_count(design.clone().split_off(atom.len()), atomic_towels, parent_towels, results);
            let prefixes = parent_towels.get(atom).unwrap();
            for prefix in prefixes {
                if design.starts_with(prefix) {
                    count += _combo_count(design.clone().split_off(prefix.len()), atomic_towels, parent_towels, results);
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

fn combo_count(design: String, atomic_towels: &Vec<String>, parent_towels: &HashMap<String, Vec<String>>) -> i64 {
    _combo_count(design, atomic_towels, &parent_towels, &mut HashMap::new())
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
                towels.append(&mut l.split(", ").map(|s| s.to_owned()).collect());
            } else if l.len() > 0 {
                designs.push(l.to_owned());
            }
        });
    let mut atomic_towels = vec![];
    let mut parent_towels = HashMap::new();
    for towel in towels {
        let count = combo_count(towel.clone(), &atomic_towels, &parent_towels);
        if count == 0 {
            parent_towels.insert(towel.clone(), vec![]);
            atomic_towels.push(towel);
        } else {
            let atomic_prefix = atomic_towels
                .iter()
                .find(|t| towel.starts_with(*t))
                .unwrap();
            parent_towels.get_mut(atomic_prefix).unwrap().push(towel);
        }
    }
    Input {
        parent_towels,
        atomic_towels,
        designs
    }
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    input.designs.iter().fold(0, |accu, design| {
        accu + (combo_count(design.clone(), &input.atomic_towels, &input.parent_towels) > 0) as i64
    })
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    input.designs.iter().fold(0, |accu, design| {
        accu + combo_count(design.clone(), &input.atomic_towels, &input.parent_towels)
    })
}
