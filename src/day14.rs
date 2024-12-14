use regex::Regex;

fn move_robots(in_lines: &str, width: i32, height: i32, seconds: i32, check_tree: bool) -> i32 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut input: Vec<i32> = re.find_iter(in_lines)
        .map(|a| a.as_str().parse().unwrap())
        .collect();
    let mut quadrants = [0, 0, 0 ,0];
    let mut bots: Vec<(i32, i32)> = vec![];
    while input.len() > 0 {
        let vy = input.pop().unwrap();
        let vx = input.pop().unwrap();
        let py = input.pop().unwrap();
        let px = input.pop().unwrap();
        let mut next_px = px + vx * seconds;
        let mut next_py = py + vy * seconds;
        if next_px < 0 {
            next_px = width - (next_px + 1).abs() % width - 1;
        } else {
            next_px = next_px % width;
        }
        if next_py < 0 {
            next_py = height - (next_py + 1).abs() % height - 1;
        } else {
            next_py = next_py % height;
        }
        bots.push((next_px, next_py));
        if next_px < width / 2 {
            if next_py < height / 2 {
                quadrants[0] += 1;
            } else if next_py != height / 2 {
                quadrants[2] += 1;
            }
        } else if next_px != width / 2 {
            if next_py < height / 2 {
                quadrants[1] += 1;
            } else if next_py != height / 2 {
                quadrants[3] += 1;
            }
        }
    }
    if check_tree {
        // I first printed the output of 10000 seconds and scrolled through it manually
        // until I knew what I was looking for. Then I changed it to a basic check
        // for the pattern. Not the nicest, but weird problem.
        for y in 0 .. height {
            let mut in_a_row = 0;
            for x in 0 .. width {
                let count = bots.iter().fold(0, |accu, bot| accu + (bot.0 == x && bot.1 == y) as i32);
                if count > 0 {
                    in_a_row += 1;
                } else {
                    if in_a_row >= 30 {
                        return seconds;
                    }
                    in_a_row = 0;
                }
            }
        }
        return 0;
    }
    quadrants.into_iter().fold(1, |accu, count| accu * count)
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> i32 {
    move_robots(input, 101, 103, 100, false)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut result = 0;
    let mut i = 0;
    while result == 0 {
        result = move_robots(input, 101, 103, i + 1, true);
        i += 1;
    }
    result
}
