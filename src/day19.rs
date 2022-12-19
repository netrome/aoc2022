pub fn p1(input: &str) -> String {
    for bp in parse_input(input) {
        println!("BP: {:?}", bp);
    }
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> impl IntoIterator<Item = Blueprint> + '_ {
    input.trim().lines().map(|line| line.parse().unwrap())
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    mines: Resource,
    price: Balance,
}

struct Factory {
    minute: usize,
    resources: Balance,
    income: Balance,
}

#[derive(Debug)]
struct Balance(HashMap<Resource, usize>);

impl Balance {
    fn try_sub(&self, other: &Self) -> Option<Self> {
        self.0
            .iter()
            .map(|(resource, amount)| {
                amount
                    .checked_sub(*other.0.get(resource).unwrap_or(&0))
                    .map(|diff| (*resource, diff))
            })
            .collect()
    }

    fn add(&mut self, other: &Self) {
        for (&resource, &amount) in other.0.iter() {
            self.add_single(resource, amount);
        }
    }

    fn add_single(&mut self, resource: Resource, amount: usize) {
        *self.0.entry(resource).or_insert(0) += amount;
    }
}

impl FromIterator<(Resource, usize)> for Balance {
    fn from_iter<T: IntoIterator<Item = (Resource, usize)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, robots_str) =
            sscanf::sscanf!(s.trim(), "Blueprint {usize}: {String}.").expect("Waaat");

        let robots = robots_str.trim().split(".").map(parse_robot).collect();

        Ok(Self { id, robots })
    }
}

fn parse_robot(s: &str) -> Robot {
    let (resource, price_str) =
        sscanf::sscanf!(s.trim(), "Each {String} robot costs {String}").expect("Noope");

    let price = price_str.split("and").map(parse_cost).collect();
    let mines = resource.parse().expect("Shit");

    Robot { mines, price }
}

fn parse_cost(s: &str) -> (Resource, usize) {
    let (amount, resource) = sscanf::sscanf!(s.trim(), "{usize} {String}").expect("Ouch");

    (resource.parse().expect("Crap"), amount)
}

impl FromStr for Resource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            other => anyhow::bail!("Noooo: {}", other),
        })
    }
}

use std::{collections::HashMap, iter::FromIterator, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(19, 1, p1));
inventory::submit!(Solution::new(19, 2, p2));
