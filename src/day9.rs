use std::iter::repeat;

fn checksum(input: &str, move_whole_files: bool) -> usize {
    let mut files: Vec<(usize, usize)> = input.chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .flat_map(|(i, c)| {
            let file_num = i / 2;
            let file_len = c.to_digit(10).unwrap() as usize;
            match move_whole_files {
                true => vec![(file_num, file_len)],
                false => repeat((file_num, 1)).take(file_len).collect()
            }
        })
        .collect();
    input.chars()
        .flat_map(|c| {
            files.reverse();
            let mut filler: Vec<usize> = vec![];
            let mut remaining_space = c.to_digit(10).unwrap() as usize;
            for i in (0 .. files.len()).rev() {
                let (file_num, file_len) = files[i];
                if file_len <= remaining_space {
                    remaining_space -= file_len;
                    files.remove(i);
                    filler.append(&mut repeat(file_num).take(file_len).collect());
                }
            }
            filler.append(&mut repeat(0).take(remaining_space).collect());
            filler
        })
        .enumerate()
        .fold(0, |accu: usize, (i, file_num)| accu + i * file_num)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    checksum(input, false)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    checksum(input, true)
}
