pub fn p1(input: &str) -> String {
    let mut cave = read_input(input);

    while cave.drop_sand(Point(0, 500)) {}

    cave.count_sand().to_string()
}

pub fn p2(input: &str) -> String {
    todo!()
}

fn read_input(input: &str) -> Cave {
    let mut objects = HashMap::new();
    let mut highest_x = i64::MIN;

    for path in input.lines().map(read_path) {
        for position in path.into_rock_positions() {
            highest_x = highest_x.max(position.0);
            objects.insert(position, Object::Rock);
        }
    }

    Cave { objects, highest_x }
}

fn read_path(line: &str) -> RockPath {
    RockPath(
        line.trim()
            .split("->")
            .map(|s| s.parse().expect("Nooo"))
            .collect(),
    )
}

#[derive(Debug)]
struct Cave {
    objects: HashMap<Point, Object>,
    highest_x: i64,
}

impl Cave {
    fn drop_sand(&mut self, point: Point) -> bool {
        if point.0 > self.highest_x {
            return false;
        }

        for candidate in point.drop_candidates() {
            if !self.objects.contains_key(&candidate) {
                return self.drop_sand(candidate);
            }
        }

        self.objects.insert(point, Object::Sand);
        true
    }

    fn count_sand(&self) -> usize {
        self.objects.values().filter(|val| val.is_sand()).count()
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point(i64, i64);

impl Point {
    fn interpolate(&self, other: &Point) -> Vec<Point> {
        let mut point = self.clone();
        let mut res = vec![point.clone()];

        let dx = (other.0 - self.0).signum();
        let dy = (other.1 - self.1).signum();

        while &point != other {
            point.0 += dx;
            point.1 += dy;

            res.push(point.clone());
        }

        res
    }

    fn drop_candidates(&self) -> impl Iterator<Item = Point> + '_ {
        let deltas = [(1, 0), (1, -1), (1, 1)];

        deltas
            .into_iter()
            .map(|(dx, dy)| Point(self.0 + dx, self.1 + dy))
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (y, x) = sscanf::sscanf!(s.trim(), "{i64},{i64}").expect("Failed to parse point");

        Ok(Self(x, y))
    }
}

#[derive(Debug)]
enum Object {
    Rock,
    Sand,
}

impl Object {
    fn is_sand(&self) -> bool {
        match self {
            Self::Sand => true,
            _ => false,
        }
    }
}

struct RockPath(Vec<Point>);

impl RockPath {
    fn into_rock_positions(self) -> Vec<Point> {
        let mut all_positions = Vec::new();
        let mut iter = self.0.into_iter();

        let mut line_start = iter.next().unwrap();

        while let Some(line_end) = iter.next() {
            let mut new_positions = line_start.interpolate(&line_end);
            all_positions.append(&mut new_positions);

            line_start = line_end;
        }

        all_positions
    }
}

use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(14, 1, p1));
inventory::submit!(Solution::new(14, 2, p2));
