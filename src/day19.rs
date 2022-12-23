pub fn p1(input: &str) -> String {
    let mut sum = 0;
    for bp in parse_input(input) {
        //println!("Max geodes: {:?}", maximize_geodes(&bp, 24));
        sum += quality_level(&bp, 24);
    }

    sum.to_string()
}

pub fn p2(input: &str) -> String {
    let mut prod = 1;
    for bp in parse_input(input).into_iter().take(3) {
        prod *= maximize_geodes(&bp, 32);
    }

    prod.to_string()
}

fn parse_input(input: &str) -> impl IntoIterator<Item = Blueprint> + '_ {
    input.trim().lines().map(|line| line.parse().unwrap())
}

fn quality_level(blueprint: &Blueprint, minutes: usize) -> usize {
    maximize_geodes(blueprint, minutes) * blueprint.id
}

fn maximize_geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut search = Vec::new();
    search.push(Factory::genesis());
    let mut visited: HashSet<Factory> = HashSet::new();

    let mut max_geodes = 0;

    while let Some(factory) = search.pop() {
        if visited.contains(&factory) {
            continue;
        }

        if factory.geode_upper_bound(minutes) <= max_geodes {
            continue;
        }

        let geodes_at_end = factory.geodes_at_minute(minutes);

        if geodes_at_end > max_geodes {
            max_geodes = geodes_at_end;
        }

        search.extend(blueprint.next_factories(&factory, minutes));
        visited.insert(factory);
    }

    max_geodes
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robots: Vec<Robot>,
}

impl Blueprint {
    fn next_factories<'a>(
        &'a self,
        factory: &'a Factory,
        max_minute: usize,
    ) -> impl Iterator<Item = Factory> + 'a {
        self.robots
            .iter()
            .filter_map(move |robot| factory.forecast_purchase(robot, max_minute))
    }
}

#[derive(Debug)]
struct Robot {
    mines: Resource,
    price: Balance,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Factory {
    minute: usize,
    resources: Balance,
    income: Balance,
}

impl Factory {
    fn genesis() -> Self {
        let mut income = Balance::new();
        income.add_single(Resource::Ore, 1);

        Self {
            minute: 0,
            resources: Balance::new(),
            income,
        }
    }

    fn geode_upper_bound(&self, minute: usize) -> usize {
        let remaining_minutes = minute.saturating_sub(self.minute);

        let upper_bound_extra_earnings = (remaining_minutes + 1) * remaining_minutes / 2;

        self.resources.get(&Resource::Geode)
            + self.income.get(&Resource::Geode) * remaining_minutes
            + upper_bound_extra_earnings
    }

    fn geodes_at_minute(&self, minute: usize) -> usize {
        self.resources.get(&Resource::Geode)
            + self.income.get(&Resource::Geode) * (minute - self.minute)
    }

    fn forecast_purchase(&self, robot: &Robot, max_minute: usize) -> Option<Self> {
        //println!("Forecast: {:?}, {:?}, {}", self, robot, max_minute);
        let _minutes_until_purchase = 0;

        if !robot
            .price
            .0
            .keys()
            .all(|resource| self.income.0.contains_key(resource))
        {
            return None;
        }

        let mut forecast = self.clone();

        loop {
            if forecast.minute >= max_minute {
                return None;
            }

            match forecast.resources.try_sub(&robot.price) {
                Some(resources) => {
                    forecast.resources = resources;
                    forecast.step();
                    break;
                }
                None => forecast.step(),
            }
        }

        forecast.income.add_single(robot.mines, 1);
        Some(forecast)
    }

    fn step(&mut self) {
        self.minute += 1;
        self.resources.add(&self.income);
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Balance(BTreeMap<Resource, usize>);

impl Balance {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn try_sub(&self, other: &Self) -> Option<Self> {
        let mut result = self.clone();

        for (resource, diff) in other.0.iter() {
            if let Some(res) = self.get(resource).checked_sub(*diff) {
                result.0.insert(*resource, res);
            } else {
                return None;
            }
        }

        Some(result)
    }

    fn add(&mut self, other: &Self) {
        for (&resource, &amount) in other.0.iter() {
            self.add_single(resource, amount);
        }
    }

    fn add_single(&mut self, resource: Resource, amount: usize) {
        *self.0.entry(resource).or_insert(0) += amount;
    }

    fn get(&self, resource: &Resource) -> usize {
        *self.0.get(resource).unwrap_or(&0)
    }
}

impl FromIterator<(Resource, usize)> for Balance {
    fn from_iter<T: IntoIterator<Item = (Resource, usize)>>(iter: T) -> Self {
        Self(BTreeMap::from_iter(iter))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
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

        let robots: Vec<_> = robots_str.trim().split(".").map(parse_robot).collect();

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

use std::{
    collections::{BTreeMap, HashSet},
    iter::FromIterator,
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(19, 1, p1));
inventory::submit!(Solution::new(19, 2, p2));
