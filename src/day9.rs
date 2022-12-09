pub fn p1(input: &str) -> String {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = head.clone();

    let mut tail_positions = HashSet::new();
    tail_positions.insert(tail.clone());

    for m in moves(input) {
        head.apply(m);
        let mut path = tail.tail_path(&head);
        if let Some(pos) = path.last() {
            tail = pos.clone()
        }

        tail_positions.extend(path.into_iter());
    }

    tail_positions.len().to_string()
}

pub fn p2(input: &str) -> String {
    let mut rope = vec![Position { x: 0, y: 0 }; 10];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(rope[9].clone());

    for m in moves(input).flat_map(|m| m.break_up()) {
        rope[0].apply(m);

        for idx in 1..rope.len() - 1 {
            let prev = rope[idx - 1].clone();
            rope[idx].follow(&prev);
        }

        let mut path = rope[9].tail_path(&rope[8]);
        if let Some(pos) = path.last() {
            rope[9] = pos.clone()
        }

        tail_positions.extend(path.into_iter());
    }

    tail_positions.len().to_string()
}

fn moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.trim().split('\n').map(|line| line.parse().unwrap())
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn apply(&mut self, m: Move) {
        match m {
            Move::Horizontal(dist) => self.x += dist,
            Move::Vertical(dist) => self.y += dist,
        }
    }

    fn tail_path(&self, head: &Position) -> Vec<Position> {
        let mut position = self.clone();
        let mut path = Vec::new();

        while position != *head {
            let dx = head.x - position.x;
            let dy = head.y - position.y;

            if dx.abs() <= 1 && dy.abs() <= 1 {
                return path;
            }

            let x = position.x + dx.signum();
            let y = position.y + dy.signum();

            position = Position { x, y };

            path.push(position.clone())
        }

        path
    }

    fn follow(&mut self, head: &Position) {
        let new_pos = self.tail_path(head).last().cloned().unwrap_or(self.clone());
        *self = new_pos;
    }
}

#[derive(Clone)]
enum Move {
    Vertical(i32),
    Horizontal(i32),
}

impl Move {
    fn break_up(self) -> Vec<Move> {
        match self {
            Self::Vertical(n) => vec![Self::Vertical(n.signum()); n.abs() as usize],
            Self::Horizontal(n) => vec![Self::Horizontal(n.signum()); n.abs() as usize],
        }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = sscanf::sscanf!(s, "{char} {i32}").expect("Nope");

        let m = match direction {
            'R' => Self::Horizontal(steps),
            'L' => Self::Horizontal(-steps),
            'U' => Self::Vertical(steps),
            'D' => Self::Vertical(-steps),
            _ => anyhow::bail!("No move: {}", s),
        };

        Ok(m)
    }
}

use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(9, 1, p1));
inventory::submit!(Solution::new(9, 2, p2));
