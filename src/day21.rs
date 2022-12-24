pub fn p1(input: &str) -> String {
    let world = parse_input(input);
    world.monkey_yell("root").unwrap().to_string()
}

pub fn p2(input: &str) -> String {
    let world2: World2 = parse_input(input).into();
    world2.solve("root").to_string()
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
    fn monkey_yell(&self, id: &str) -> Option<i64> {
        let monkey = self.monkeys.get(id).expect("No monkey!");

        self.evaluate_expr(&monkey.expr)
    }

    fn evaluate_expr(&self, expr: &Expr) -> Option<i64> {
        match expr {
            Expr::Const(val) => Some(*val),
            Expr::BinaryOp(left, op, right) => self.evaluate_op(left, *op, right),
            Expr::Unknown => None,
            Expr::Eq(_, _) => panic!("Not supported"),
        }
    }

    fn evaluate_op(&self, left: &str, op: Operator, right: &str) -> Option<i64> {
        let left = self.monkey_yell(left)?;
        let right = self.monkey_yell(right)?;

        Some(op.eval(left, right))
    }
}

struct World2 {
    monkeys: HashMap<String, Monkey>,
}

impl World2 {
    fn solve(&self, eq_id: &str) -> i64 {
        let monkey = self.monkeys.get(eq_id).expect("No monkey!");

        self.evaluate_eq(&monkey.expr)
    }

    fn evaluate_eq(&self, expr: &Expr) -> i64 {
        let (left, right) = match expr {
            Expr::Eq(left, right) => (left, right),
            _ => panic!("Not supported"),
        };

        let left_eval = self.monkey_yell(&left);
        let right_eval = self.monkey_yell(&right);

        let (next_id, should_be_equal_to) = match (left_eval, right_eval) {
            (Some(val), None) => (right, val),
            (None, Some(val)) => (left, val),
            _ => panic!("Unsatisfiable equation"),
        };

        self.solve_eq(next_id, should_be_equal_to)
    }

    fn solve_eq(&self, monkey_id: &str, should_be_equal_to: i64) -> i64 {
        let monkey = self.monkeys.get(monkey_id).expect("No monkey!");

        match &monkey.expr {
            Expr::Unknown => should_be_equal_to,
            Expr::BinaryOp(left, op, right) => self.next_eq(left, *op, right, should_be_equal_to),
            _ => panic!("Cannot solve eq"),
        }
    }

    fn next_eq(&self, left: &str, op: Operator, right: &str, should_be_equal_to: i64) -> i64 {
        let left_eval = self.monkey_yell(&left);
        let right_eval = self.monkey_yell(&right);

        let next_eq = op.solve(left_eval, right_eval, should_be_equal_to);

        let next_id = match (left_eval, right_eval) {
            (Some(_), None) => right,
            (None, Some(_)) => left,
            _ => panic!("Unsatisfiable equation 2"),
        };

        self.solve_eq(next_id, next_eq)
    }

    fn monkey_yell(&self, id: &str) -> Option<i64> {
        let monkey = self.monkeys.get(id).expect("No monkey!");

        self.evaluate_expr(&monkey.expr)
    }

    fn evaluate_expr(&self, expr: &Expr) -> Option<i64> {
        match expr {
            Expr::Const(val) => Some(*val),
            Expr::BinaryOp(left, op, right) => self.evaluate_op(left, *op, right),
            Expr::Unknown => None,
            Expr::Eq(_, _) => panic!("Not supported"),
        }
    }

    fn evaluate_op(&self, left: &str, op: Operator, right: &str) -> Option<i64> {
        let left = self.monkey_yell(left)?;
        let right = self.monkey_yell(right)?;

        Some(op.eval(left, right))
    }
}

impl From<World> for World2 {
    fn from(world: World) -> Self {
        let mut monkeys = world.monkeys;

        if let Expr::BinaryOp(left, _, right) = monkeys.get("root").unwrap().expr.clone() {
            monkeys.get_mut("root").unwrap().expr = Expr::Eq(left, right);
        }

        monkeys.get_mut("humn").unwrap().expr = Expr::Unknown;

        Self { monkeys }
    }
}

#[derive(Debug)]
struct Monkey {
    id: String,
    expr: Expr,
}

#[derive(Debug, Clone)]
enum Expr {
    Unknown,
    Const(i64),
    BinaryOp(String, Operator, String),
    Eq(String, String),
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

    fn solve(&self, left: Option<i64>, right: Option<i64>, ans: i64) -> i64 {
        match (self, left, right) {
            (Self::Add, Some(val), None) => ans - val,
            (Self::Add, None, Some(val)) => ans - val,
            (Self::Sub, Some(val), None) => val - ans,
            (Self::Sub, None, Some(val)) => ans + val,
            (Self::Mul, Some(val), None) => ans / val,
            (Self::Mul, None, Some(val)) => ans / val,
            (Self::Div, Some(val), None) => val / ans,
            (Self::Div, None, Some(val)) => ans * val,
            _ => panic!("Unsolveable eq for operator"),
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
