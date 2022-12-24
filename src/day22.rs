pub fn p1(input: &str) -> String {
    let (board, moves) = parse_input(input);
    let santa = Santa::new(board.start_pos());
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> (Board, Moves) {
    let (board_input, move_input) = input.split_once("\n\n").unwrap();

    let moves: Moves = move_input.parse().unwrap();
    let board: Board = board_input.parse().unwrap();

    (board, moves)
}

struct Santa {
    pos: Pos,
    direction: Delta,
}

impl Santa {
    fn new(pos: Pos) -> Self {
        Self {
            pos,
            direction: Delta::right(),
        }
    }

    fn advance(&mut self, board: &Board, movement: &Movement) {
        match movement {
            Movement::TurnLeft => self.direction = self.direction.rotate_left(),
            Movement::TurnRight => self.direction = self.direction.rotate_right(),
            Movement::Forward(steps) => self.walk(&board, *steps),
        }
    }

    fn walk(&mut self, board: &Board, steps: usize) {
        for _ in 0..steps {
            let (next_pos, obj) = self.next_step(&board);
        }
    }

    fn next_step(&self, board: &Board) -> (Pos, Obj) {
        let next_pos = self.pos.apply_delta(&self.direction);

        if let Some(obj) = board.items.get(&next_pos) {
            return (next_pos, *obj);
        }

        let warped_pos = match self.direction {
            Delta(-1, 0) => Pos(self.pos.0, board.vranges[&self.pos.0].1),
            Delta(1, 0) => Pos(self.pos.0, board.vranges[&self.pos.0].0),
            Delta(0, -1) => Pos(board.hranges[&self.pos.1].1, self.pos.1),
            Delta(0, 1) => Pos(board.hranges[&self.pos.1].0, self.pos.1),
            _ => panic!("😢"),
        };

        let obj = *board.items.get(&warped_pos).unwrap();
        (warped_pos, obj)
    }
}

#[derive(Debug)]
struct Board {
    items: HashMap<Pos, Obj>,
    hranges: HashMap<i64, Range>,
    vranges: HashMap<i64, Range>,
}

impl Board {
    fn start_pos(&self) -> Pos {
        let mut all_positions: Vec<Pos> = self.items.keys().map(|pos| pos.clone()).collect();
        all_positions.sort();

        all_positions.into_iter().next().unwrap()
    }

    fn populate_ranges(&mut self) {
        for pos in self.items.keys() {
            let top_edge = self.find_edge(pos.clone(), &Delta::up());
            let bottom_edge = self.find_edge(pos.clone(), &Delta::down());

            let left_edge = self.find_edge(pos.clone(), &Delta::left());
            let right_edge = self.find_edge(pos.clone(), &Delta::right());

            let vrange = Range(top_edge.0, bottom_edge.0);
            let hrange = Range(left_edge.1, right_edge.1);

            self.vranges.insert(pos.0, vrange);
            self.hranges.insert(pos.1, hrange);
        }
    }

    fn find_edge(&self, mut pos: Pos, direction: &Delta) -> Pos {
        if !self.items.contains_key(&pos) {
            panic!("Oh noooo!")
        }

        loop {
            let next = pos.apply_delta(direction);

            if !self.items.contains_key(&next) {
                return pos;
            } else {
                pos = next
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Obj {
    Open,
    Solid,
}

#[derive(Debug)]
struct Range(i64, i64);

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos(i64, i64);

impl Pos {
    fn apply_delta(&self, delta: &Delta) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Delta(i64, i64);

impl Delta {
    const fn up() -> Self {
        Self(-1, 0)
    }

    const fn down() -> Self {
        Self(1, 0)
    }

    const fn left() -> Self {
        Self(0, -1)
    }

    const fn right() -> Self {
        Self(0, 1)
    }

    fn rotate_left(&self) -> Self {
        Self(-self.1, self.0)
    }

    fn rotate_right(&self) -> Self {
        Self(self.1, -self.0)
    }
}

#[derive(Debug, Clone)]
enum Movement {
    TurnLeft,
    TurnRight,
    Forward(usize),
}
#[derive(Debug, Clone)]
struct Moves(Vec<Movement>);

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, c)| (row, col, c)))
            .filter_map(|(row, col, c)| {
                let pos = Pos(row as i64, col as i64);
                match c {
                    '.' => Some((pos, Obj::Open)),
                    '#' => Some((pos, Obj::Solid)),
                    _ => None,
                }
            })
            .collect();

        let hranges = HashMap::new();
        let vranges = HashMap::new();

        let mut board = Self {
            items,
            hranges,
            vranges,
        };

        board.populate_ranges();

        Ok(board)
    }
}

impl FromStr for Moves {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = Vec::new();

        let mut number = Vec::new();
        for c in s.trim().chars() {
            if c.is_numeric() {
                number.push(c);
                continue;
            }

            if !number.is_empty() {
                let numstr: String = number.into_iter().collect();
                moves.push(Movement::Forward(numstr.parse().expect("Numstr???")));
                number = Vec::new();
            }

            match c {
                'L' => moves.push(Movement::TurnLeft),
                'R' => moves.push(Movement::TurnRight),
                _ => panic!("Noooo!"),
            }
        }

        Ok(Self(moves))
    }
}

use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(22, 1, p1));
inventory::submit!(Solution::new(22, 2, p2));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deltas_should_rotate_intuitively() {
        assert_eq!(Delta::up().rotate_left(), Delta::left());
        assert_eq!(Delta::up().rotate_right(), Delta::right());
        assert_eq!(Delta::left().rotate_left(), Delta::down());
        assert_eq!(Delta::left().rotate_right(), Delta::up());
    }
}
