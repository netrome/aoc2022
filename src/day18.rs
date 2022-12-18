pub fn p1(input: &str) -> String {
    let grid: Grid = input.parse().expect("Nooo");
    grid.surface_area().to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

#[derive(Debug)]
struct Grid {
    cubes: HashSet<Cube>,
}

impl Grid {
    fn surface_area(&self) -> usize {
        self.cubes.iter().map(|cube| cube.surface_area(self)).sum()
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self { cubes })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    fn surface_area(&self, grid: &Grid) -> usize {
        let (mut left, mut right, mut in_, mut out, mut up, mut down) = (
            self.clone(),
            self.clone(),
            self.clone(),
            self.clone(),
            self.clone(),
            self.clone(),
        );

        left.x += 1;
        right.x -= 1;
        in_.y += 1;
        out.y -= 1;
        up.z += 1;
        down.z -= 1;

        [left, right, in_, out, up, down]
            .into_iter()
            .filter(|cube| !grid.cubes.contains(&cube))
            .count()
    }
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = sscanf::sscanf!(s.trim(), "{i64},{i64},{i64}")
            .map_err(|e| anyhow::anyhow!("Noo {}", e))?;

        Ok(Self { x, y, z })
    }
}

use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(18, 1, p1));
inventory::submit!(Solution::new(18, 2, p2));
