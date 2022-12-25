pub fn p1(input: &str) -> String {
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

struct World {
    minute: usize,
    santa_possible_points: HashSet<Pos>,
    blizzards: Vec<Blizzard>,
    goal: Pos,
    max_x: i64,
    max_y: i64,
}

struct Blizzard {
    pos: Pos,
    direction: Direction,
}

impl Blizzard {
    fn advance(&mut self, max_y: i64, max_x: i64) {
        let mut next_pos = self.pos.advance(self.direction);

        if next_pos.0 >= max_x {
            next_pos.0 = 1
        };

        if next_pos.1 >= max_y {
            next_pos.1 = 1
        };

        if next_pos.0 == 0 {
            next_pos.0 = max_x - 1
        };

        if next_pos.1 == 0 {
            next_pos.1 = max_y - 1
        };

        self.pos = next_pos
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Pos(i64, i64);

impl Pos {
    fn advance(&self, direction: Direction) -> Self {
        let mut next = self.clone();

        match direction {
            Direction::Up => next.0 -= 1,
            Direction::Down => next.0 += 1,
            Direction::Left => next.1 -= 1,
            Direction::Right => next.1 += 1,
        }

        next
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use std::collections::{HashMap, HashSet};

use crate::solution::Solution;
inventory::submit!(Solution::new(24, 1, p1));
inventory::submit!(Solution::new(24, 2, p2));
