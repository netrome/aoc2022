pub fn p1(input: &str) -> String {
    let world = parse_input(input);
    world.monkey_yell("root").to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> World {
    let monkeys = input
        .lines()
        .map(|line| line.parse::<Monkey>().expect("Monkey bizniz failed"))
        .map(|monkey| (monkey.id.clone(), monkey))
        .collect();

    World { monkeys }
}

struct World {
    monkeys: HashMap<String, Monkey>,
}

impl World {
    fn monkey_yell(&self, id: &str) -> i64 {
        let monkey = self.monkeys.get(id).expect("No monkey!");

        self.evaluate_expr(&monkey.expr)
    }

    fn evaluate_expr(&self, expr: &Expr) -> i64 {
        match expr {
            Expr::Const(val) => *val,
            Expr::BinaryOp(left, op, right) => self.evaluate_op(left, *op, right),
        }
    }

    fn evaluate_op(&self, left: &str, op: Operator, right: &str) -> i64 {
        let left = self.monkey_yell(left);
        let right = self.monkey_yell(right);

        op.eval(left, right)
    }
}

#[derive(Debug)]
struct Monkey {
    id: String,
    expr: Expr,
}

#[derive(Debug)]
enum Expr {
    Const(i64),
    BinaryOp(String, Operator, String),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn eval(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, expr) =
            sscanf::sscanf!(s.trim(), "{String}: {String}").map_err(|_| anyhow::anyhow!("Crap"))?;

        Ok(Self {
            id,
            expr: expr.parse()?,
        })
    }
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.trim().parse() {
            return Ok(Self::Const(val));
        }

        let (left, op, right) = sscanf::sscanf!(s.trim(), "{String} {char} {String}")
            .map_err(|_| anyhow::anyhow!("Nope"))?;

        Ok(Self::BinaryOp(left, op.try_into()?, right))
    }
}

impl TryFrom<char> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => anyhow::bail!("Noooooo"),
        })
    }
}

use std::collections::HashMap;
use std::{convert::TryFrom, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(21, 1, p1));
inventory::submit!(Solution::new(21, 2, p2));
