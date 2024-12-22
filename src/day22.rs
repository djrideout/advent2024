#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut sum = 0;
    input.lines().for_each(|l| {
        let mut secret: i64 = l.parse().unwrap();
        for _ in 0 .. 2000 {
            secret = ((secret * 64) ^ secret) % 16777216;
            secret = ((secret / 32) ^ secret) % 16777216;
            secret = ((secret * 2048) ^ secret) % 16777216;
        }
        sum += secret;
    });
    sum
}
