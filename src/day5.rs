use std::{cmp::Ordering, collections::HashMap};

pub struct Input {
    after_rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>
}

#[aoc_generator(day5)]
pub fn input_generator(in_lines: &str) -> Input {
    let mut after_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates = vec![];
    in_lines
        .lines()
        .for_each(|l| {
            if l.contains('|') {
                let rules: Vec<i32> = l.trim().split("|").map(|d| d.parse().unwrap()).collect();
                after_rules.entry(rules[0]).and_modify(|vec| vec.push(rules[1])).or_insert(vec![rules[1]]);
            }
            if l.contains(',') {
                let pages: Vec<i32> = l.trim().split(",").map(|d| d.parse().unwrap()).collect();
                updates.push(pages);
            }
        });
    Input {
        after_rules,
        updates
    }
}

fn get_matching_updates(after_rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>, valid_cmp: bool) -> Vec<Vec<i32>> {
    let mut out = vec![];
    for i in 0 .. updates.len() {
        let pages = &updates[i];
        let mut valid = true;
        for j in 0 .. pages.len() {
            for k in 0 .. j {
                valid &= after_rules.get(&pages[k]).unwrap().contains(&pages[j]);
            }
        }
        if valid == valid_cmp {
            out.push(pages.clone());
        }
    }
    out
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    get_matching_updates(&input.after_rules, &input.updates, true)
        .iter().fold(0, |accu, vec| accu + vec[vec.len() / 2])
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> i32 {
    get_matching_updates(&input.after_rules, &input.updates, false)
        .iter_mut().fold(0, |accu, vec| {
            vec.sort_by(|a, b| {
                if input.after_rules.get(&a).unwrap().contains(&b) {
                    return Ordering::Greater;
                }
                Ordering::Less
            });
            accu + vec[vec.len() / 2]
        })
}
