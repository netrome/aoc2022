pub fn p1(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    for instr in instructions {
        instr.apply(&mut stacks)
    }

    stacks
        .into_iter()
        .map(|mut stack| stack.pop().expect("No item in stack"))
        .collect()
}

pub fn p2(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    for instr in instructions {
        instr.apply_9001(&mut stacks)
    }

    stacks
        .into_iter()
        .map(|mut stack| stack.pop().expect("No item in stack"))
        .collect()
}

fn parse_input(input: &str) -> (Stacks, Instructions) {
    let (stacks_inp, instructions_inp) = input.split_once("\n\n").expect("Oh no");

    let stacks = parse_stacks(stacks_inp);
    let instr = parse_instructions(instructions_inp.trim());

    (stacks, instr)
}

fn parse_stacks(input: &str) -> Stacks {
    let mut stacks: HashMap<usize, Vec<_>> = HashMap::new();

    for stack_line in input.split('\n').rev().map(parse_stack_line).skip(1) {
        for (idx, char) in stack_line.into_iter().enumerate() {
            if char != ' ' {
                stacks.entry(idx).or_default().push(char);
            }
        }
    }

    let mut keys: Vec<&usize> = stacks.keys().collect();
    keys.sort();
    let mut res = Vec::new();

    for key in keys {
        res.push(stacks[key].clone())
    }

    res
}

fn parse_stack_line(line: &str) -> Vec<char> {
    line.chars()
        .enumerate()
        .filter(|(idx, _)| idx % 4 == 1)
        .map(|(_, c)| c)
        .collect()
}

fn parse_instructions(input: &str) -> Instructions {
    input
        .split('\n')
        .map(|line| line.parse::<Instruction>().expect("Crap"))
        .collect()
}

type Stacks = Vec<Vec<char>>;
type Instructions = Vec<Instruction>;

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl Instruction {
    fn apply(&self, target: &mut Stacks) {
        let (from, to) = (self.from - 1, self.to - 1);

        for _ in 0..self.count {
            let item = target[from].pop().expect("Pop failed");
            target[to].push(item)
        }
    }

    fn apply_9001(&self, target: &mut Stacks) {
        let (from, to) = (self.from - 1, self.to - 1);

        let mut items = Vec::new();

        for _ in 0..self.count {
            items.push(target[from].pop().expect("Pop failed"));
        }

        for item in items.into_iter().rev() {
            target[to].push(item)
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, from, to) =
            sscanf::sscanf!(s, "move {usize} from {usize} to {usize}").expect("Shitze");

        Ok(Self { from, to, count })
    }
}

use std::collections::HashMap;
use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(4, 1, p1));
inventory::submit!(Solution::new(4, 2, p2));
