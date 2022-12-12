pub fn p1(input: &str) -> String {
    let map = read_map(input);
    shortest_path(map.start_position, &map).to_string()
}

pub fn p2(input: &str) -> String {
    let map = read_map(input);
    let mut ans: usize = usize::MAX;

    for start_position in map
        .heights
        .iter()
        .filter(|(_pos, height)| **height == 'a' as u32)
        .map(|(pos, _height)| pos)
    {
        ans = ans.min(shortest_path(*start_position, &map));
    }

    ans.to_string()
}

fn shortest_path(start_position: (usize, usize), map: &Map) -> usize {
    let mut search = Search::new(start_position);

    while search.has_step() {
        search.step(&map);
    }

    *search
        .shortest_distances
        .get(&map.target_location)
        .unwrap_or(&usize::MAX)
}

fn read_map(input: &str) -> Map {
    let mut heights = HashMap::new();
    let mut start_position = (12, 12);
    let mut target_location = (12, 12);

    for (row, line) in input.split_whitespace().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let height = match c {
                'S' => {
                    start_position = (row, col);
                    'a' as u32
                }
                'E' => {
                    target_location = (row, col);
                    'z' as u32
                }
                other => other as u32,
            };
            heights.insert((row, col), height);
        }
    }

    Map {
        heights,
        start_position,
        target_location,
    }
}

#[derive(Debug)]
struct Map {
    heights: HashMap<(usize, usize), u32>,
    start_position: (usize, usize),
    target_location: (usize, usize),
}

impl Map {
    fn neighbors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let north = (pos.0.wrapping_sub(1), pos.1);
        let south = (pos.0.wrapping_add(1), pos.1);
        let east = (pos.0, pos.1.wrapping_add(1));
        let west = (pos.0, pos.1.wrapping_sub(1));

        let height = self.heights.get(pos).expect("Crap");

        [north, east, south, west]
            .into_iter()
            .filter(|new_pos| {
                let new_height = self.heights.get(new_pos).unwrap_or(&u32::MAX);
                *new_height <= height + 1
            })
            .collect()
    }
}

struct Search {
    to_visit: VecDeque<(usize, usize)>,
    shortest_distances: HashMap<(usize, usize), usize>,
}

impl Search {
    fn new(start_pos: (usize, usize)) -> Self {
        let mut shortest_distances = HashMap::new();
        shortest_distances.insert(start_pos, 0);
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start_pos);

        Self {
            to_visit,
            shortest_distances,
        }
    }

    fn has_step(&self) -> bool {
        !self.to_visit.is_empty()
    }

    fn step(&mut self, map: &Map) {
        let visit = self.to_visit.pop_front().expect("No more steps");
        let distance = *self.shortest_distances.get(&visit).expect("Impossibru!");

        for next_step in map.neighbors(&visit) {
            if self.shortest_distances.contains_key(&next_step) {
                continue;
            } else {
                self.shortest_distances.insert(next_step, distance + 1);
                self.to_visit.push_back(next_step);
            }
        }
    }
}

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::solution::Solution;
inventory::submit!(Solution::new(12, 1, p1));
inventory::submit!(Solution::new(12, 2, p2));
