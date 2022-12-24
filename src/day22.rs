pub fn p1(input: &str) -> String {
    let (board, moves) = parse_input(input);
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> (Board, Moves) {
    let (board_input, move_input) = input.trim().split_once("\n\n").unwrap();

    let moves: Moves = move_input.parse().unwrap();
    println!("{:?}", moves);

    todo!();
}

struct Board {
    items: HashMap<Pos, Obj>,
    hranges: HashMap<i64, Range>,
    vranges: HashMap<i64, Range>,
}

enum Obj {
    Open,
    Solid,
}

struct Range {
    min: i64,
    max: i64,
}

#[derive(Debug, Clone)]
struct Pos(i64, i64);
#[derive(Debug, Clone)]
struct Delta(i64, i64);
#[derive(Debug, Clone)]
enum Move {
    TurnLeft,
    TurnRight,
    Forward(i64),
}
#[derive(Debug, Clone)]
struct Moves(Vec<Move>);

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
                moves.push(Move::Forward(numstr.parse().expect("Numstr???")));
                number = Vec::new();
            }

            match c {
                'L' => moves.push(Move::TurnLeft),
                'R' => moves.push(Move::TurnRight),
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
