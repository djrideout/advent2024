use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.trim().split(" ").map(|d| d.parse().unwrap()).collect()
}

fn count_stones(input: &Vec<u64>, blinks: i32) -> usize {
    let mut child_graph = HashMap::<u64, Vec<u64>>::new();
    let mut appearances = HashMap::<u64, usize>::new();
    let mut to_compute_children = input.clone();
    to_compute_children.iter().for_each(|t| {
        appearances.insert(*t, 1);
    });
    for _ in 0 .. blinks {
        let mut next_to_compute_children: Vec<u64> = vec![];
        while let Some(stone) = to_compute_children.pop() {
            let mut stone_str = stone.to_string();
            let mut children: Vec<u64> = vec![];
            if stone == 0 {
                children.push(1);
            } else if stone_str.len() % 2 == 0 {
                children.push(stone_str.split_off(stone_str.len() / 2).parse().unwrap());
                children.push(stone_str.parse().unwrap());
            } else {
                children.push(stone * 2024);
            }
            for i in 0 .. children.len() {
                if !child_graph.contains_key(&children[i]) {
                    next_to_compute_children.push(children[i]);
                }
            }
            child_graph.insert(stone, children);
        }
        to_compute_children = next_to_compute_children;
        let mut next_appearances = appearances.clone();
        let mut adds =  HashMap::<u64, usize>::new();
        let mut removes =  HashMap::<u64, usize>::new();
        for (key, value) in appearances.into_iter() {
            if value > 0 {
                removes.insert(key, value);
                let children = child_graph.get(&key).unwrap();
                for i in 0 .. children.len() {
                    let child_count = adds.get(&children[i]);
                    adds.insert(children[i], child_count.or(Some(&0)).unwrap() + value);
                }
            }
        }
        for (remove, _) in removes {
            next_appearances.insert(remove, 0);
        }
        for (add, count) in adds {
            let value = next_appearances.get(&add);
            next_appearances.insert(add, value.or(Some(&0)).unwrap() + count);
        }
        appearances = next_appearances;
    }
    appearances.into_iter().fold(0, |accu, (_, b)| accu + b)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<u64>) -> usize {
    count_stones(input, 25)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<u64>) -> usize {
    count_stones(input, 75)
}
