use std::collections::VecDeque;

fn is_safe(levels: &mut VecDeque<i32>) -> bool {
    let mut prev_level = levels.pop_front().unwrap();
    let mut prev_total_diff = 0;
    for level in levels {
        let level_diff = *level - prev_level;
        let total_diff = prev_total_diff + level_diff;
        let same_sign = prev_total_diff == 0 || (prev_total_diff ^ total_diff) >= 0;
        let magnitude_increasing = prev_total_diff.abs() < total_diff.abs();
        let diff_within_range = level_diff.abs() >= 1 && level_diff.abs() <= 3;
        if !same_sign || !magnitude_increasing || !diff_within_range {
            return false;
        }
        prev_level = *level;
        prev_total_diff = total_diff;
    }
    true
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut safe_count = 0;
    input
        .lines()
        .for_each(|l| {
            let mut levels: VecDeque<i32> = l.trim().split(" ").map(|d| d.parse().unwrap()).collect();
            if is_safe(&mut levels) {
                safe_count += 1;
            }
        });
    safe_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut safe_count = 0;
    input
        .lines()
        .for_each(|l| {
            let levels: VecDeque<i32> = l.trim().split(" ").map(|d| d.parse().unwrap()).collect();
            let mut safe = is_safe(&mut levels.clone());
            let mut i = 0;
            while !safe && i < levels.len() {
                let mut levels_subset = levels.clone();
                levels_subset.remove(i);
                safe = is_safe(&mut levels_subset);
                i += 1;
            }
            if safe {
                safe_count += 1;
            }
        });
    safe_count
}
