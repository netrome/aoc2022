pub fn p1(input: &str) -> String {
    let grid: Grid = input.parse().expect("Nooo");
    grid.surface_area().to_string()
}

pub fn p2(input: &str) -> String {
    let grid: Grid = input.parse().expect("Nooo");
    let steam = compute_steam(&grid);
    grid.surface_area_2(&steam).to_string()
}

fn compute_steam(grid: &Grid) -> HashSet<Cube> {
    let max_x = grid.cubes.iter().map(|c| c.x).max().unwrap();
    let max_y = grid.cubes.iter().map(|c| c.y).max().unwrap();
    let max_z = grid.cubes.iter().map(|c| c.z).max().unwrap();

    let mut to_expand = vec![Cube::origin()];
    let mut res = HashSet::new();

    while let Some(steam) = to_expand.pop() {
        for neighbor in steam.neighbors() {
            if neighbor.x > max_x + 1 || neighbor.x < -1 {
                continue;
            }

            if neighbor.y > max_y + 1 || neighbor.y < -1 {
                continue;
            }

            if neighbor.z > max_z + 1 || neighbor.z < -1 {
                continue;
            }

            if !grid.cubes.contains(&neighbor) && !res.contains(&neighbor) {
                to_expand.push(neighbor)
            }
        }

        res.insert(steam);
    }

    res
}

#[derive(Debug)]
struct Grid {
    cubes: HashSet<Cube>,
}

impl Grid {
    fn surface_area(&self) -> usize {
        self.cubes.iter().map(|cube| cube.surface_area(self)).sum()
    }

    fn surface_area_2(&self, steam: &HashSet<Cube>) -> usize {
        self.cubes
            .iter()
            .map(|cube| cube.surface_area_2(&steam))
            .sum()
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
    fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn surface_area(&self, grid: &Grid) -> usize {
        self.neighbors()
            .into_iter()
            .filter(|cube| !grid.cubes.contains(&cube))
            .count()
    }

    fn surface_area_2(&self, steam: &HashSet<Cube>) -> usize {
        self.neighbors()
            .into_iter()
            .filter(|cube| steam.contains(&cube))
            .count()
    }

    fn neighbors(&self) -> [Cube; 6] {
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
