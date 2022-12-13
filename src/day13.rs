pub fn p1(input: &str) -> String {
    let pairs = parse_input(input);

    println!("Pairs: {:?}", pairs);
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .trim()
        .split("\n\n")
        .map(|pair_line| parse_pair_line(&pair_line))
        .collect()
}

fn parse_pair_line(line: &str) -> Pair {
    let (left, right) = line.split_once('\n').expect("No pair lines???");

    let left = left.parse().expect("Failed to parse left");
    let right = right.parse().expect("Failed to parse right");

    Pair(left, right)
}

#[derive(Debug)]
struct Pair(Packet, Packet);

#[derive(Debug)]
enum Packet {
    UInt(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn from_tokens(mut tokens: impl Iterator<Item = Token>) -> Self {
        Self::from_tokens_inner(&mut tokens)
    }

    fn from_tokens_inner(tokens: &mut impl Iterator<Item = Token>) -> Self {
        match tokens.next().expect("Why you give me empty input?") {
            Token::Digit(val) => Self::UInt(val),
            Token::StartList => Self::build_list(tokens),
            Token::EndList => panic!("Nonononooo!"),
        }
    }

    fn build_list(tokens: &mut impl Iterator<Item = Token>) -> Self {
        let mut packets = Vec::new();

        loop {
            packets.push(match tokens.next().expect("No next token???") {
                Token::Digit(val) => Self::UInt(val),
                Token::StartList => Self::build_list(tokens),
                Token::EndList => break,
            })
        }

        Self::List(packets)
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(s);

        Ok(Self::from_tokens(tokens.into_iter()))
    }
}

#[derive(Debug)]
enum Token {
    Digit(usize),
    StartList,
    EndList,
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    tokenize_inner(line, &mut tokens);
    tokens
}

fn tokenize_inner(line: &str, output: &mut Vec<Token>) {
    let mut digit_buffer = Vec::new();

    for c in line.chars() {
        match c {
            '[' => {
                make_digit(&mut digit_buffer, output);
                output.push(Token::StartList)
            }
            ']' => {
                make_digit(&mut digit_buffer, output);
                output.push(Token::EndList)
            }
            ',' => {
                make_digit(&mut digit_buffer, output);
            }
            other => {
                if other.is_digit(10) {
                    digit_buffer.push(other)
                } else if other.is_whitespace() {
                } else {
                    panic!("Unexpected token: {}", other);
                }
            }
        }
    }
}

fn make_digit(buffer: &mut Vec<char>, output: &mut Vec<Token>) {
    if buffer.len() > 0 {
        let mut local = Vec::new();
        std::mem::swap(&mut local, buffer);

        let token = Token::Digit(
            local
                .into_iter()
                .collect::<String>()
                .parse()
                .expect("Unparsable digit"),
        );

        output.push(token)
    }
}

impl Packet {
    fn scan(iter: &mut impl Iterator<Item = char>) -> Self {
        let peek = iter.peekable();
        let c = iter.next();

        todo!();
    }
}

use std::{iter::FromIterator, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(13, 1, p1));
inventory::submit!(Solution::new(13, 2, p2));
