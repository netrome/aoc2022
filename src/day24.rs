pub fn p1(input: &str) -> String {
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

#[derive(Debug)]
struct World {
    minute: usize,
    santa_possible_points: HashSet<Pos>,
    blizzards: Vec<Blizzard>,
    goal: Pos,
    max_x: i64,
    max_y: i64,
}

#[derive(Debug)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl FromStr for World {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blizzards = Vec::new();
        let mut santa_possible_points = HashSet::new();
        let max_x = (s.lines().count() - 1) as i64;
        let max_y = (s.lines().next().unwrap().len() - 1) as i64;

        for (x, line) in s.trim().lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                let direction = match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    'v' => Direction::Down,
                    _ => continue,
                };

                let blizzard = Blizzard {
                    pos: Pos(x as i64, y as i64),
                    direction,
                };

                blizzards.push(blizzard);
            }
        }

        let start = Pos(0, 1);
        let goal = Pos(max_x, max_y - 1);
        santa_possible_points.insert(start);

        Ok(Self {
            blizzards,
            santa_possible_points,
            max_x,
            max_y,
            goal,
            minute: 0,
        })
    }
}

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(24, 1, p1));
inventory::submit!(Solution::new(24, 2, p2));
