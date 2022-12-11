pub fn p1(input: &str) -> String {
    let mut monkeys = read_monkeys(input);

    for _ in 0..20 {
        round(&mut monkeys);
    }

    crate::day1::k_largest(2, monkeys.values().map(|monkey| monkey.activity))
        .iter()
        .product::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn read_monkeys(input: &str) -> HashMap<usize, Monkey> {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.parse().expect("Monkey parsing failed"))
        .map(|monkey: Monkey| (monkey.id, monkey))
        .collect()
}

fn round(monkeys: &mut HashMap<usize, Monkey>) {
    for monkey_id in 0..monkeys.len() {
        let mut monkey = monkeys.remove(&monkey_id).expect("Boom");

        while let Some(item) = monkey.items.pop_front() {
            let worry_level = monkey.op.apply(item);
            monkey.increment_activity();

            let new_level = relax(worry_level);
            let next_monkey = monkey.test.next_monkey(new_level);

            monkeys
                .get_mut(&next_monkey)
                .expect("Pam!")
                .items
                .push_back(new_level);
        }

        monkeys.insert(monkey_id, monkey);
    }
}

fn relax(worry_level: u64) -> u64 {
    worry_level / 3
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    op: Op,
    test: Test,
    activity: usize,
}

impl Monkey {
    fn increment_activity(&mut self) {
        self.activity += 1;
    }
}

#[derive(Debug)]
enum Op {
    Add(u64),
    AddSelf,
    Mul(u64),
    MulSelf,
}

impl Op {
    fn apply(&self, item: u64) -> u64 {
        match self {
            Self::Add(val) => item + val,
            Self::AddSelf => item + item,
            Self::Mul(val) => item * val,
            Self::MulSelf => item * item,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn next_monkey(&self, val: u64) -> usize {
        if val % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().split('\n');

        let id = lines
            .next()
            .map(|id_line| {
                sscanf::sscanf!(id_line.trim(), "Monkey {usize}:").expect("ID parse fail")
            })
            .expect("No ID line");

        let items: VecDeque<u64> = lines
            .next()
            .map(|items_line| {
                let items = sscanf::sscanf!(items_line.trim(), "Starting items: {String}")
                    .expect("No starting items");
                items
                    .split(", ")
                    .map(|s| s.parse().expect("Oh crap"))
                    .collect()
            })
            .expect("No starting items line");

        let op: Op = lines
            .next()
            .expect("No op line")
            .parse()
            .expect("Failed to parse op");

        let divisible_by: u64 = sscanf::sscanf!(
            lines.next().expect("No div line").trim(),
            "Test: divisible by {u64}"
        )
        .expect("Failed to parse div line");

        let if_true: usize = sscanf::sscanf!(
            lines.next().expect("No happy line").trim(),
            "If true: throw to monkey {usize}"
        )
        .expect("Failed to parse happy line");

        let if_false: usize = sscanf::sscanf!(
            lines.next().expect("No sad line").trim(),
            "If false: throw to monkey {usize}"
        )
        .expect("Failed to parse sad line");

        let test = Test {
            divisible_by,
            if_true,
            if_false,
        };

        Ok(Self {
            id,
            items,
            op,
            test,
            activity: 0,
        })
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operator, val) = sscanf::sscanf!(s.trim(), "Operation: new = old {char} {String}")
            .expect("Failed to scan op");

        let op = match (operator, val.trim()) {
            ('*', "old") => Self::MulSelf,
            ('+', "old") => Self::AddSelf,
            ('*', val) => Self::Mul(val.parse().expect("Crapzy")),
            ('+', val) => Self::Add(val.parse().expect("Shitze")),
            _ => anyhow::bail!("Noooooo :("),
        };

        Ok(op)
    }
}

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(11, 1, p1));
inventory::submit!(Solution::new(11, 2, p2));
