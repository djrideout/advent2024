use crate::grid::dijkstra;

pub struct Input {
    drops: Vec<(usize, usize)>,
    width: usize,
    map_size: usize,
    start_pos: i32,
    end_pos: i32
}

#[aoc_generator(day18)]
pub fn input_generator(in_lines: &str) -> Input {
    let drops = in_lines
        .lines()
        .map(|l| {
            let pos: Vec<usize> = l.trim().split(",").map(|d| d.parse().unwrap()).collect();
            (pos[0], pos[1])
        })
        .collect();
    let width = 71;
    let height = 71;
    let map_size = width * height;
    Input {
        drops,
        width,
        map_size,
        start_pos: 0,
        end_pos: (width * height - 1) as i32
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let Input {
        width,
        map_size,
        start_pos,
        end_pos,
        drops
    } = input;
    let mut map = vec!['.'; *map_size];
    let drop_count = 1024;
    for i in 0 .. drop_count {
        let (x, y) = drops[i];
        map[y * width + x] = '#';
    }
    let result = dijkstra(&map.iter().collect(), *width as i32, *start_pos, *end_pos, 0, 0);
    *result.min_distances.get(&end_pos).unwrap()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Input) -> String {
    let Input {
        width,
        map_size,
        start_pos,
        end_pos,
        drops
    } = input;
    for drop_count in (0 .. drops.len() - 1).rev() {
        let mut map = vec!['.'; *map_size];
        for i in 0 .. drop_count {
            let (x, y) = drops[i];
            map[y * width + x] = '#';
        }
        let result = dijkstra(&map.iter().collect(), *width as i32, *start_pos, *end_pos, 0, 0);
        if result.min_distances.get(&end_pos).is_some() {
            return format!("{},{}", drops[drop_count].0, drops[drop_count].1);
        }
    }
    "".to_string()
}
