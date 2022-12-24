pub fn p1(input: &str) -> String {
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
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

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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

use std::{convert::TryFrom, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(21, 1, p1));
inventory::submit!(Solution::new(21, 2, p2));
