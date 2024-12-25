pub struct Input {
    locks: Vec<[i32; 5]>,
    keys: Vec<[i32; 5]>
}

#[aoc_generator(day25)]
pub fn generate_input(input: &str) -> Input {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut parsing = false;
    let mut is_key = false;
    let mut counts = [0; 5];
    input.lines().for_each(|l| {
        if l == "" {
            parsing = false;
            if is_key {
                keys.push(counts);
            } else {
                locks.push(counts);
            }
        } else {
            if parsing == false {
                parsing = true;
                is_key = l == ".....";
                if is_key {
                    counts = [-1; 5];
                } else {
                    counts = [0; 5];
                }
            } else {
                l.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        counts[i] += 1;
                    }
                });
            }
        }
    });
    if is_key {
        keys.push(counts);
    } else {
        locks.push(counts);
    }
    Input {
        locks,
        keys
    }
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let mut count = 0;
    for lock in &input.locks {
        for key in &input.keys {
            let mut fits = true;
            for i in 0 .. 5 {
                fits &= lock[i] + key[i] <= 5;
            }
            if fits {
                count += 1;
            }
        }
    }
    count
}
