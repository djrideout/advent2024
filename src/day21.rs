use std::collections::VecDeque;
use regex::Regex;

fn get_presses(curr_button: char, next_button: char) -> Vec<String> {
    match curr_button {
        '7' => match next_button {
            '7' => vec!["A"],
            '8' => vec![">A"],
            '9' => vec![">>A"],
            '4' => vec!["vA"],
            '5' => vec!["v>A", ">vA"],
            '6' => vec!["v>>A", ">>vA"],
            '1' => vec!["vvA"],
            '2' => vec!["vv>A", ">vvA"],
            '3' => vec!["vv>>A", ">>vvA"],
            '0' => vec![">vvvA"],
            'A' => vec![">>vvvA"],
            _ => vec![""]
        },
        '8' => match next_button {
            '7' => vec!["<A"],
            '8' => vec!["A"],
            '9' => vec![">A"],
            '4' => vec!["<vA", "v<A"],
            '5' => vec!["vA"],
            '6' => vec![">vA", "v>A"],
            '1' => vec!["<vvA", "vv<A"],
            '2' => vec!["vvA"],
            '3' => vec![">vvA", "vv>A"],
            '0' => vec!["vvvA"],
            'A' => vec!["vvv>A", ">vvvA"],
            _ => vec![""]
        },
        '9' => match next_button {
            '7' => vec!["<<A"],
            '8' => vec!["<A"],
            '9' => vec!["A"],
            '4' => vec!["<<vA", "v<<A"],
            '5' => vec!["<vA", "v<A"],
            '6' => vec!["vA"],
            '1' => vec!["<<vvA", "vv<<A"],
            '2' => vec!["<vvA", "vv<A"],
            '3' => vec!["vvA"],
            '0' => vec!["<vvvA", "vvv<A"],
            'A' => vec!["vvvA"],
            _ => vec![""]
        },
        '4' => match next_button {
            '7' => vec!["^A"],
            '8' => vec![">^A", "^>A"],
            '9' => vec![">>^A", "^>>A"],
            '4' => vec!["A"],
            '5' => vec![">A"],
            '6' => vec![">>A"],
            '1' => vec!["vA"],
            '2' => vec!["v>A", ">vA"],
            '3' => vec!["v>>A", ">>vA"],
            '0' => vec![">vvA"],
            'A' => vec![">>vvA"],
            _ => vec![""]
        },
        '5' => match next_button {
            '7' => vec!["<^A", "^<A"],
            '8' => vec!["^A"],
            '9' => vec![">^A", "^>A"],
            '4' => vec!["<A"],
            '5' => vec!["A"],
            '6' => vec![">A"],
            '1' => vec!["<vA", "v<A"],
            '2' => vec!["vA"],
            '3' => vec![">vA", "v>A"],
            '0' => vec!["vvA"],
            'A' => vec!["vv>A", ">vvA"],
            _ => vec![""]
        },
        '6' => match next_button {
            '7' => vec!["<<^A", "^<<A"],
            '8' => vec!["<^A", "^<A"],
            '9' => vec!["^A"],
            '4' => vec!["<<A"],
            '5' => vec!["<A"],
            '6' => vec!["A"],
            '1' => vec!["v<<A", "<<vA"],
            '2' => vec!["v<A", "<vA"],
            '3' => vec!["vA"],
            '0' => vec!["<vvA", "vv<A"],
            'A' => vec!["vvA"],
            _ => vec![""]
        },
        '1' => match next_button {
            '7' => vec!["^^A"],
            '8' => vec!["^^>A", ">^^A"],
            '9' => vec!["^^>>A", ">>^^A"],
            '4' => vec!["^A"],
            '5' => vec!["^>A", ">^A"],
            '6' => vec!["^>>A", ">>^A"],
            '1' => vec!["A"],
            '2' => vec![">A"],
            '3' => vec![">>A"],
            '0' => vec![">vA"],
            'A' => vec![">>vA"],
            _ => vec![""]
        },
        '2' => match next_button {
            '7' => vec!["^^<A", "<^^A"],
            '8' => vec!["^^A"],
            '9' => vec!["^^>A", ">^^A"],
            '4' => vec!["^<A", "<^A"],
            '5' => vec!["^A"],
            '6' => vec!["^>A", ">^A"],
            '1' => vec!["<A"],
            '2' => vec!["A"],
            '3' => vec![">A"],
            '0' => vec!["vA"],
            'A' => vec!["v>A", ">vA"],
            _ => vec![""]
        },
        '3' => match next_button {
            '7' => vec!["<<^^A", "^^<<A"],
            '8' => vec!["<^^A", "^^<A"],
            '9' => vec!["^^A"],
            '4' => vec!["<<^A", "^<<A"],
            '5' => vec!["<^A", "^<A"],
            '6' => vec!["^A"],
            '1' => vec!["<<A"],
            '2' => vec!["<A"],
            '3' => vec!["A"],
            '0' => vec!["v<A", "<vA"],
            'A' => vec!["vA"],
            _ => vec![""]
        },
        '0' => match next_button {
            '7' => vec!["^^^<A"],
            '8' => vec!["^^^A"],
            '9' => vec!["^^^>A", ">^^^A"],
            '4' => vec!["^^<A"],
            '5' => vec!["^^A"],
            '6' => vec!["^^>A", ">^^A"],
            '1' => vec!["^<A"],
            '2' => vec!["^A"],
            '3' => vec!["^>A", ">^A"],
            '0' => vec!["A"],
            'A' => vec![">A"],
            _ => vec![""]
        },
        'A' => match next_button {
            '7' => vec!["^^^<<A"],
            '8' => vec!["^^^<A", "<^^^A"],
            '9' => vec!["^^^A"],
            '4' => vec!["^^<<A"],
            '5' => vec!["^^<A", "<^^A"],
            '6' => vec!["^^A"],
            '1' => vec!["^<<A"],
            '2' => vec!["^<A", "<^A"],
            '3' => vec!["^A"],
            '0' => vec!["<A"],
            'A' => vec!["A"],
            '^' => vec!["<A"],
            '<' => vec!["v<<A"],
            'v' => vec!["v<A", "<vA"],
            '>' => vec!["vA"],
            _ => vec![""]
        },
        '^' => match next_button {
            '^' => vec!["A"],
            '<' => vec!["v<A"],
            'v' => vec!["vA"],
            '>' => vec![">vA", "v>A"],
            'A' => vec![">A"],
            _ => vec![""]
        },
        '<' => match next_button {
            '^' => vec![">^A"],
            '<' => vec!["A"],
            'v' => vec![">A"],
            '>' => vec![">>A"],
            'A' => vec![">>^A"],
            _ => vec![""]
        },
        'v' => match next_button {
            '^' => vec!["^A"],
            '<' => vec!["<A"],
            'v' => vec!["A"],
            '>' => vec![">A"],
            'A' => vec![">^A", "^>A"],
            _ => vec![""]
        },
        '>' => match next_button {
            '^' => vec!["<^A", "^<A"],
            '<' => vec!["<<A"],
            'v' => vec!["<A"],
            '>' => vec!["A"],
            'A' => vec!["^A"],
            _ => vec![""]
        },
        _ => vec![""]
    }.into_iter().map(|str| str.to_string()).collect()
}

fn shortest_path(seq: String, counter: i32) -> String {
    let chars: Vec<char> = seq.chars().collect();
    let mut min = Vec::<String>::new();
    let mut q = VecDeque::<(i32, String)>::new();
    q.push_back((-1, "".to_string()));
    while q.len() > 0 {
        let (i_i32, sub_seq) = q.pop_back().unwrap();
        let curr_button = match i_i32 {
            -1 => 'A',
            _ => chars[i_i32 as usize]
        };
        let i = i_i32 as usize;
        let options = get_presses(curr_button, chars[i + 1]);
        for option in options {
            let next_sub_seq = sub_seq.clone();
            let results = match counter {
                0 => {
                    let mut with_option = next_sub_seq.clone();
                    with_option.push_str(&option);
                    with_option
                },
                _ => {
                    let mut with_option = next_sub_seq.clone();
                    let shortest = shortest_path(option.clone(), counter - 1);
                    with_option.push_str(&shortest);
                    with_option
                }
            };
            if min.len() == i + 1 {
                min.push(results.clone());
            } else if results.len() < min[i + 1].len() {
                min[i + 1] = results.clone();
            }
            if i + 2 < chars.len() {
                q.push_back((i_i32 + 1, results));
            }
        }
        q.make_contiguous().sort_by(|(_, a), (_, b)| {
            a.len().cmp(&b.len())
        });
    }
    min.last().unwrap().clone()
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let digit_regex = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;
    input.lines().for_each(|l| {
        let result = shortest_path(l.to_string(), 2);
        let numeric_code: usize = digit_regex.find(l).unwrap().as_str().parse().unwrap();
        sum += result.len() * numeric_code;
    });
    sum
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &str) -> usize {
    let digit_regex = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;
    input.lines().for_each(|l| {
        let result = shortest_path(l.to_string(), 2);
        let numeric_code: usize = digit_regex.find(l).unwrap().as_str().parse().unwrap();
        sum += result.len() * numeric_code;
    });
    sum
}
