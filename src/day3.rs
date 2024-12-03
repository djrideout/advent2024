use regex::Regex;

fn total(input: &str) -> i32 {
    let mut_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for captures in mut_regex.captures_iter(input) {
        let a: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        total += a * b;
    }
    total
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    total(input)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let remove_regex = Regex::new(r"don't\(\)(.|\n)*?(do\(\)|$)").unwrap();
    let cleaned_input = remove_regex.replace_all(input, "").to_string();
    total(&cleaned_input)
}
