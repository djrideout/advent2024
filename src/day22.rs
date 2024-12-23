use std::collections::{HashMap, HashSet};

fn roll_secret(secret: i64, rolls: usize) -> i64 {
    let mut new_secret = secret;
    for _ in 0 .. rolls {
        new_secret = ((new_secret * 64) ^ new_secret) % 16777216;
        new_secret = ((new_secret / 32) ^ new_secret) % 16777216;
        new_secret = ((new_secret * 2048) ^ new_secret) % 16777216;
    }
    new_secret
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> i64 {
    input.lines().fold(0, |accu, l| {
        accu + roll_secret(l.parse().unwrap(), 2000)
    })
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut maps: Vec<HashMap<String, i64>> = vec![];
    let mut seqs: HashSet<String> = HashSet::new();
    input.lines().for_each(|l| {
        let mut diffs = vec![];
        let mut map = HashMap::new();
        let mut secret: i64 = l.parse().unwrap();
        for _ in 0 .. 2000 {
            let new_secret = roll_secret(secret, 1);
            let ones = new_secret % 10;
            let diff = ones - (secret % 10);
            diffs.push(diff);
            if diffs.len() > 4 {
                diffs.remove(0);
            }
            if diffs.len() == 4 {
                let key = format!("{},{},{},{}", diffs[0], diffs[1], diffs[2], diffs[3]);
                seqs.insert(key.clone());
                if !map.contains_key(&key) {
                    map.insert(key.clone(), ones);
                }
            }
            secret = new_secret;
        }
        maps.push(map);
    });
    seqs.iter().fold(0, |accu, seq| {
        let sum = maps.iter().fold(0, |accu, map| {
            accu + map.get(seq).unwrap_or(&0)
        });
        accu.max(sum)
    })
}
