use std::collections::HashMap;
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

fn shortest_len(seq: String, middle_remotes: i32, cache: &mut HashMap<String, usize>) -> usize {
    let key = format!("{}_{}", seq, middle_remotes);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let chars: Vec<char> = seq.chars().collect();
    let mut min = Vec::<usize>::new();
    let mut q = vec![(-1, 0)];
    while q.len() > 0 {
        let (i_i32, curr_len) = q.pop().unwrap();
        let i = i_i32 as usize;
        let curr_button = match i_i32 {
            -1 => 'A',
            _ => chars[i]
        };
        for option in get_presses(curr_button, chars[i + 1]) {
            let next_len = curr_len + match middle_remotes {
                0 => option.len(),
                _ => shortest_len(option.clone(), middle_remotes - 1, cache)
            };
            if min.len() == i + 1 {
                min.push(next_len);
            } else if next_len < min[i + 1] {
                min[i + 1] = next_len;
            }
            if i + 2 < chars.len() {
                q.push((i_i32 + 1, next_len));
            }
        }
        q.sort_by(|(_, a), (_, b)| a.cmp(&b));
    }
    let result = min.last().unwrap();
    cache.insert(key, *result);
    *result
}

fn compute_sum(input: &str, middle_remotes: i32) -> usize {
    let digit_regex = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;
    input.lines().for_each(|l| {
        let mut cache: HashMap<String, usize> = HashMap::new();
        let result = shortest_len(l.to_string(), middle_remotes, &mut cache);
        let numeric_code: usize = digit_regex.find(l).unwrap().as_str().parse().unwrap();
        sum += result * numeric_code;
    });
    sum
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    compute_sum(input, 2)
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &str) -> usize {
    compute_sum(input, 25)
}
