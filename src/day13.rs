pub fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .enumerate()
        .filter(|(_, pair)| pair.is_in_order())
        .map(|(idx, _)| idx + 1)
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let divider_1: Packet = "[[2]]".parse().unwrap();
    let divider_2: Packet = "[[6]]".parse().unwrap();

    let mut all_packets: Vec<Packet> = input
        .trim()
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.parse().unwrap())
        .collect();

    all_packets.push(divider_1.clone());
    all_packets.push(divider_2.clone());

    all_packets.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

    let mut divider_1_index = 0;
    let mut divider_2_index = 0;

    for (idx, packet) in all_packets.into_iter().enumerate() {
        if packet == divider_1 {
            divider_1_index = idx + 1;
        } else if packet == divider_2 {
            divider_2_index = idx + 1;
        }
    }

    (divider_1_index * divider_2_index).to_string()
}

fn parse_input(input: &str) -> Vec<Pair> {
    input.trim().split("\n\n").map(parse_pair_line).collect()
}

fn parse_pair_line(line: &str) -> Pair {
    let (left, right) = line.split_once('\n').expect("No pair lines???");

    let left = left.parse().expect("Failed to parse left");
    let right = right.parse().expect("Failed to parse right");

    Pair(left, right)
}

#[derive(Debug)]
struct Pair(Packet, Packet);

impl Pair {
    fn is_in_order(&self) -> bool {
        self.0 <= self.1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::UInt(lhs), Self::UInt(rhs)) => lhs.partial_cmp(rhs),
            (Self::List(lhs), Self::List(rhs)) => compare_lists(lhs, rhs),
            (Self::List(lhs), rhs) => compare_lists(lhs, &[rhs.clone()]),
            (lhs, Self::List(rhs)) => compare_lists(&[lhs.clone()], rhs),
        }
    }
}

fn compare_lists(lhs: &[Packet], rhs: &[Packet]) -> Option<std::cmp::Ordering> {
    for (left, right) in lhs.iter().zip(rhs.iter()) {
        match left.partial_cmp(right).expect("Impossibruu!") {
            std::cmp::Ordering::Equal => continue,
            other => return Some(other),
        }
    }

    lhs.len().partial_cmp(&rhs.len())
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
                if other.is_ascii_digit() {
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
    if !buffer.is_empty() {
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

use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(13, 1, p1));
inventory::submit!(Solution::new(13, 2, p2));
