use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.trim().split(" ").map(|d| d.parse().unwrap()).collect()
}

fn count_stones(input: &Vec<u64>, blinks: i32) -> usize {
    let mut counts = HashMap::<u64, usize>::new();
    input.iter().for_each(|stone_num| {
        counts.insert(*stone_num, 1);
    });
    for _ in 0 .. blinks {
        let mut next_counts = HashMap::<u64, usize>::new();
        for (stone_num, count) in counts {
            let mut stone_str = stone_num.to_string();
            let mut children: Vec<u64> = vec![];
            if stone_num == 0 {
                children.push(1);
            } else if stone_str.len() % 2 == 0 {
                children.push(stone_str.split_off(stone_str.len() / 2).parse().unwrap());
                children.push(stone_str.parse().unwrap());
            } else {
               children.push(stone_num * 2024);
            }
            for child in children {
                let child_count = next_counts.get(&child).unwrap_or(&0);
                next_counts.insert(child, child_count + count);
            }
        }
        counts = next_counts;
    }
    counts.iter().fold(0, |accu, (_, count)| accu + count)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<u64>) -> usize {
    count_stones(input, 25)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<u64>) -> usize {
    count_stones(input, 75)
}
