pub fn p1(input: &str) -> String {
    let (board, moves) = parse_input(input);
    let mut santa = Santa::new(board.start_pos());

    for movement in moves.0 {
        santa.advance(&board, &movement, false);
    }

    santa.password().to_string()
}

pub fn p2(input: &str) -> String {
    let (board, moves) = parse_input(input);
    let mut santa = Santa::new(board.start_pos());

    println!("Moves total: {}", moves.0.len());
    for (idx, movement) in moves.0.into_iter().enumerate() {
        println!("{}: Movement: {:?}, Santa: {:?}", idx, movement, santa);
        santa.advance(&board, &movement, true);
    }

    santa.password().to_string()
}

fn parse_input(input: &str) -> (Board, Moves) {
    let (board_input, move_input) = input.split_once("\n\n").unwrap();

    let moves: Moves = move_input.parse().unwrap();
    let board: Board = board_input.parse().unwrap();

    (board, moves)
}

#[derive(Debug)]
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

    fn advance(&mut self, board: &Board, movement: &Movement, cube_warp: bool) {
        match movement {
            Movement::TurnLeft => self.direction = self.direction.rotate_left(),
            Movement::TurnRight => self.direction = self.direction.rotate_right(),
            Movement::Forward(steps) => self.walk(&board, *steps, cube_warp),
        }
    }

    fn walk(&mut self, board: &Board, steps: usize, cube_warp: bool) {
        for _ in 0..steps {
            let (next_pos, obj, delta) = self.next_step(&board, cube_warp);
            self.direction = delta;

            match obj {
                Obj::Open => self.pos = next_pos,
                Obj::Solid => return,
            }
        }
    }

    fn next_step(&self, board: &Board, cube_warp: bool) -> (Pos, Obj, Delta) {
        let next_pos = self.pos.apply_delta(&self.direction);

        if let Some(obj) = board.items.get(&next_pos) {
            return (next_pos, *obj, self.direction.clone());
        }

        if cube_warp {
            self.cube_warp(next_pos, board)
        } else {
            self.warp_2d(next_pos, board)
        }
    }

    // Ugh, hard-coded for my input shape
    //  12
    //  3
    // 54
    // 6
    fn cube_warp(&self, next_pos: Pos, board: &Board) -> (Pos, Obj, Delta) {
        let (warped_pos, delta) = if next_pos.0 < 0 {
            if next_pos.1 < 100 {
                self.warp_16(next_pos)
            } else {
                self.warp_26(next_pos)
            }
        } else if next_pos.1 >= 150 {
            self.warp_24(next_pos)
        } else if next_pos.1 >= 100 && self.direction == Delta::right() {
            if next_pos.0 >= 100 {
                self.warp_42(next_pos)
            } else {
                self.warp_32(next_pos)
            }
        } else if next_pos.0 >= 200 {
            self.warp_62(next_pos)
        } else if next_pos.1 < 0 {
            if next_pos.0 >= 150 {
                self.warp_61(next_pos)
            } else {
                self.warp_51(next_pos)
            }
        } else if next_pos.0 >= 150 && self.direction == Delta::right() {
            self.warp_64(next_pos)
        } else if next_pos.0 == 99 && self.direction == Delta::up() {
            self.warp_53(next_pos)
        } else if next_pos.1 < 50 && self.direction == Delta::left() {
            if next_pos.0 >= 50 {
                self.warp_35(next_pos)
            } else {
                self.warp_15(next_pos)
            }
        } else if next_pos.0 == 50 && self.direction == Delta::down() {
            self.warp_23(next_pos)
        } else if next_pos.0 == 150 && self.direction == Delta::down() {
            self.warp_46(next_pos)
        } else if next_pos.1 == 50 && self.direction == Delta::right() {
            self.warp_64(next_pos)
        } else {
            panic!("Untouched edge")
        };

        println!("Warped pos: {:?}", warped_pos);
        let obj = *board.items.get(&warped_pos).unwrap();
        (warped_pos, obj, delta)
    }

    fn warp_16(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(next_pos.1 + 100, 0), Delta::right())
    }

    fn warp_15(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(149 - next_pos.0, 0), Delta::right())
    }

    fn warp_26(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(199, next_pos.1 - 100), Delta::up())
    }

    fn warp_24(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(100 + (49 - next_pos.0), 99), Delta::left())
    }

    fn warp_23(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(next_pos.1, 99), Delta::left())
    }

    fn warp_42(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(49 - (next_pos.0 - 100), 149), Delta::left())
    }

    fn warp_46(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(next_pos.1 + 100, 49), Delta::up())
    }

    fn warp_32(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(49, next_pos.0), Delta::up())
    }

    fn warp_35(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(100, next_pos.0 - 50), Delta::down())
    }

    fn warp_62(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(0, next_pos.1 + 100), Delta::down())
    }

    fn warp_64(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(149, next_pos.0 - 100), Delta::up())
    }

    fn warp_61(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(0, next_pos.0 - 100), Delta::down())
    }

    fn warp_51(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(149 - next_pos.0, 49), Delta::right())
    }

    fn warp_53(&self, next_pos: Pos) -> (Pos, Delta) {
        (Pos(next_pos.1 + 50, 50), Delta::right())
    }

    fn warp_2d(&self, next_pos: Pos, board: &Board) -> (Pos, Obj, Delta) {
        let warped_pos = match self.direction {
            Delta(-1, 0) => Pos(board.vranges[&self.pos.1].1, self.pos.1),
            Delta(1, 0) => Pos(board.vranges[&self.pos.1].0, self.pos.1),
            Delta(0, -1) => Pos(self.pos.0, board.hranges[&self.pos.0].1),
            Delta(0, 1) => Pos(self.pos.0, board.hranges[&self.pos.0].0),
            _ => panic!("😢"),
        };

        let obj = *board.items.get(&warped_pos).unwrap();
        (warped_pos, obj, self.direction.clone())
    }

    fn password(&self) -> i64 {
        let facing_val = match self.direction {
            Delta(0, 1) => 0,
            Delta(1, 0) => 1,
            Delta(0, -1) => 2,
            Delta(-1, 0) => 3,
            _ => panic!("Not supported"),
        };

        (self.pos.0 + 1) * 1000 + (self.pos.1 + 1) * 4 + facing_val
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

            let hrange = Range(left_edge.1, right_edge.1);
            let vrange = Range(top_edge.0, bottom_edge.0);

            self.hranges.insert(pos.0, hrange);
            self.vranges.insert(pos.1, vrange);
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

        if !number.is_empty() {
            let numstr: String = number.into_iter().collect();
            moves.push(Movement::Forward(numstr.parse().expect("Numstr???")));
            number = Vec::new();
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
        assert_eq!(Delta::left().rotate_right().rotate_right(), Delta::right());
        assert_eq!(Delta::left().rotate_left().rotate_left(), Delta::right());
        assert_eq!(
            Delta::left()
                .rotate_right()
                .rotate_right()
                .rotate_right()
                .rotate_right(),
            Delta::left()
        );
    }
}
