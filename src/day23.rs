use std::collections::{HashMap, HashSet};

fn get_links(input: &str) -> HashMap<String, HashSet<String>> {
    let mut all_links: HashMap<String, HashSet<String>> = HashMap::new();
    input.lines().for_each(|l| {
        let pair: Vec<String> = l.split("-").map(|s| s.to_string()).collect();
        for (i, pc) in pair.iter().enumerate() {
            if !all_links.contains_key(pc) {
                all_links.insert(pc.to_owned(), HashSet::new());
            }
            let links = all_links.get_mut(pc).unwrap();
            links.insert(pair[!i & 1].to_owned());
        }
    });
    all_links
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &str) -> usize {
    let all_links = get_links(input);
    let mut triplets = HashSet::new();
    for (pc, links) in &all_links {
        if !pc.starts_with("t") {
            continue;
        }
        let other_pcs: Vec<&String> = links.into_iter().collect();
        for i in 0 .. other_pcs.len() {
            for j in 0 .. other_pcs.len() {
                if !all_links.get(other_pcs[i]).unwrap().contains(other_pcs[j]) {
                    continue;
                }
                let mut triplet = [pc, other_pcs[i], other_pcs[j]];
                triplet.sort();
                triplets.insert(triplet);
            }
        }
    }
    triplets.len()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &str) -> String {
    let all_links = get_links(input);
    let mut cycles: HashMap<String, HashSet<String>> = HashMap::new();
    all_links.keys().for_each(|key| {
        cycles.insert(key.clone(), HashSet::new());
    });
    let mut largest_key = String::new();
    let mut largest_len = 0;
    for (a, set_a) in &all_links {
        for b in set_a {
            let set_b = all_links.get(b).unwrap();
            let mut cycle_a = cycles.get(a).unwrap().clone();
            let mut cycle_b = cycles.get(b).unwrap().clone();
            let mutuals = set_a.intersection(set_b);
            for mutual in mutuals {
                let mut connected_to_all = true;
                let union = cycle_a.union(&cycle_b);
                for pc in union {
                    let set_pc = all_links.get(pc).unwrap();
                    connected_to_all &= set_pc.contains(mutual);
                }
                if connected_to_all {
                    cycle_a.insert(mutual.to_string());
                    cycle_b.insert(mutual.to_string());
                }
            }
            if cycle_a.len() > largest_len {
                largest_len = cycle_a.len();
                largest_key = a.to_string();
            }
            if cycle_b.len() > largest_len {
                largest_len = cycle_b.len();
                largest_key = b.to_string();
            }
            cycles.insert(a.to_string(), cycle_a);
            cycles.insert(b.to_string(), cycle_b);
        }
    }
    let mut out = vec![largest_key.clone()];
    let mut largest_cycle: Vec<String> = cycles.get(&largest_key).unwrap().iter().map(|str| str.to_string()).collect();
    out.append(&mut largest_cycle);
    out.sort();
    out.join(",")
}
