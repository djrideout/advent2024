use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};
use regex::Regex;
use crate::grid::{Input, Grid, generate_input, get_opposite_direction, get_perpendicular_directions};

#[aoc_generator(day16)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Edge {
    dest: i32,
    distance: i32,
    positions: Vec<i32>,
    cost: i32,
    direction: usize
}

fn get_turn_cost(curr_dir: usize, next_dir: usize) -> i32 {
    if curr_dir == next_dir {
        0
    } else if get_opposite_direction(curr_dir) == next_dir {
        2000
    } else {
        1000
    }
}

fn is_position_node(pos: i32, direction: usize, grid: &Grid) -> bool {
    for dir in get_perpendicular_directions(direction) {
        let (next_char, _) = grid.get_char(pos, dir, 1);
        if next_char != '#' {
            return true;
        }
    }
    false
}

fn get_neighbors(pos: i32, direction: usize, grid: &Grid) -> Vec<Edge> {
    let mut neighbors: Vec<Edge> = vec![];
    for next_direction in [1, 3, 4, 6] {
        let turn_cost = get_turn_cost(direction, next_direction);
        let mut positions = vec![pos];
        loop {
            let (next_char, next_pos) = grid.get_char(*positions.last().unwrap(), next_direction, 1);
            if next_char == '#' {
                break;
            }
            let distance = positions.len() as i32;
            positions.push(next_pos);
            if is_position_node(next_pos, next_direction, grid) {
                neighbors.push(Edge {
                    dest: next_pos,
                    distance,
                    positions: positions.clone(),
                    cost: turn_cost + distance,
                    direction: next_direction
                });
            }
        }
    }
    neighbors
}

fn shortest_paths(input: &Input, get_position_count: bool) -> i32 {
    let grid = Grid::new(input.input.clone(), input.line_len);
    let start_pos = Regex::new(r"S").unwrap().find(&input.input).unwrap().start() as i32;
    let end_pos = Regex::new(r"E").unwrap().find(&input.input).unwrap().start() as i32;
    let mut min_distances: HashMap::<i32, i32> = HashMap::new();
    let mut node_queue: VecDeque::<(i32, usize)> = VecDeque::new();
    let mut visited_positions: Vec<i32> = vec![];
    let mut prev_edges: HashMap::<i32, Vec<Edge>> = HashMap::new();
    min_distances.insert(start_pos, 0);
    node_queue.push_back((start_pos, 4));
    while node_queue.len() > 0 {
        let (pos, direction) = node_queue.pop_front().unwrap();
        let curr_distance = *min_distances.get(&pos).unwrap();
        for neighbor in get_neighbors(pos, direction, &grid) {
            if visited_positions.contains(&neighbor.dest) {
                continue;
            }
            let min_distance = min_distances.get(&neighbor.dest);
            let new_distance = curr_distance + neighbor.cost;
            if !min_distance.is_some() || new_distance <= *min_distance.unwrap() {
                min_distances.insert(neighbor.dest, new_distance);
                if let Some(destination_edges) = prev_edges.get_mut(&neighbor.dest) {
                    destination_edges.push(neighbor.clone());
                } else {
                    prev_edges.insert(neighbor.dest, vec![neighbor.clone()]);
                }
                if let Some(index) = node_queue.iter().position(|(p, _)| *p == neighbor.dest) {
                    node_queue.remove(index);
                }
                node_queue.push_back((neighbor.dest, neighbor.direction));
            }
            node_queue.make_contiguous().sort_by(|(a, _), (b, _)| {
                let a_dist = min_distances.get(&a).unwrap();
                let b_dist = min_distances.get(&b).unwrap();
                return a_dist.cmp(b_dist);
            });
        }
        visited_positions.push(pos);
    }
    if get_position_count {
        let mut positions: HashSet<i32> = HashSet::new();
        let mut edges = HashSet::from_iter(prev_edges.get(&end_pos).unwrap());
        while edges.len() > 0 {
            let mut new_edges: HashSet<&Edge> = HashSet::new();
            for edge in edges {
                edge.positions.iter().for_each(|p| {
                    positions.insert(*p);
                });
                let (_, source_pos) = grid.get_char(edge.dest, get_opposite_direction(edge.direction), edge.distance);
                if let Some(source_edges) = prev_edges.get(&source_pos) {
                    source_edges.iter().for_each(|edge| {
                        new_edges.insert(edge);
                    });
                }
            }
            edges = new_edges;
        }
        positions.len() as i32
    } else {
        *min_distances.get(&end_pos).unwrap()
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    shortest_paths(input, false)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> i32 {
    shortest_paths(input, true)
}
