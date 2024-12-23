use regex::Regex;
use rand::seq::SliceRandom;

fn get_remote_presses(press_buttons: String, button_states: &mut Vec<char>, index: usize, out: &mut String) {
    if index == button_states.len() {
        out.push_str(&button_states[index - 1].to_string());
        return;
    }
    let get_presses = |curr_button: char, next_button: char| {
        match curr_button {
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
            'A' => match next_button {
                '^' => vec!["<A"],
                '<' => vec!["v<<A"],
                'v' => vec!["v<A", "<vA"],
                '>' => vec!["vA"],
                'A' => vec!["A"],
                _ => vec![""]
            },
            _ => vec![""]
        }
    };
    press_buttons.chars().for_each(|button| {
        let presses = get_presses(button_states[index], button);
        let random_seq = presses.choose(&mut rand::thread_rng()).unwrap();
        button_states[index] = button;
        get_remote_presses(random_seq.to_string(), button_states, index + 1, out);
    });
}

fn get_keypad_presses(press_buttons: String, button_states: &mut Vec<char>, index: usize, out: &mut String) {
    let get_presses = |curr_button: char, next_button: char| {
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
                _ => vec![""]
            },
            _ => vec![""]
        }
    };
    press_buttons.chars().for_each(|button| {
        let presses = get_presses(button_states[index], button);
        let random_seq = presses.choose(&mut rand::thread_rng()).unwrap();
        button_states[index] = button;
        get_remote_presses(random_seq.to_string(), button_states, index + 1, out);
    });
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let digit_regex = Regex::new(r"(\d+)").unwrap();
    let mut button_states = vec!['A'; 4];
    let mut min = 10000000;
    for _ in 0 .. 10000 {
        let mut sum = 0;
        input.lines().for_each(|l| {
            let mut out = String::new();
            get_keypad_presses(l.to_owned(), &mut button_states, 0, &mut out);
            let numeric_code: usize = digit_regex.find(l).unwrap().as_str().parse().unwrap();
            sum += out.len() * numeric_code;
        });
        if sum < min {
            min = sum;
        }
    }
    min
}