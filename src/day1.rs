use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];
    input
        .lines()
        .for_each(|l| {
            let mut pair = l.trim().split("   ").map(|d| d.parse().unwrap());
            a.push(pair.next().unwrap());
            b.push(pair.next().unwrap());
        });
    a.sort();
    b.sort();
    let mut out = 0;
    for i in 0..a.len() {
        out += (a[i] - b[i]).abs();
    }
    out
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut a: Vec<i32> = vec![];
    let mut b: HashMap<i32, i32> = HashMap::new();
    input
        .lines()
        .for_each(|l| {
            let mut pair = l.trim().split("   ").map(|d| d.parse().unwrap());
            a.push(pair.next().unwrap());
            let num_b = pair.next().unwrap();
            b.insert(num_b, match b.get(&num_b) {
                None => 1,
                Some(v) => *v + 1
            });
        });
    let mut out = 0;
    for i in 0..a.len() {
        out += a[i] * match b.get(&a[i]) {
            None => 0,
            Some(v) => *v
        };
    }
    out
}
