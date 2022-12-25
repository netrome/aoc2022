pub fn p1(input: &str) -> String {
    let mut world: World = input.parse().unwrap();
    println!("World {:?}", world);
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

#[derive(Debug)]
struct World {
    elf_positions: HashSet<Pos>,
}

impl World {
    fn simulate_round(&self, round: usize) -> Self {
        let (proposed_next_step_counts, proposed_next_steps) = self.propose_next_steps(round);
        let next_positions = self.move_to_proposals(proposed_next_step_counts, proposed_next_steps);

        World {
            elf_positions: next_positions,
        }
    }

    fn move_to_proposals(
        &self,
        proposed_next_step_counts: HashMap<Pos, usize>,
        proposed_next_steps: HashMap<Pos, Pos>,
    ) -> HashSet<Pos> {
        proposed_next_steps
            .into_iter()
            .map(|(old, new)| {
                if *proposed_next_step_counts.get(&new).unwrap() == 1 {
                    new
                } else {
                    old
                }
            })
            .collect()
    }

    fn propose_next_steps(&self, round: usize) -> (HashMap<Pos, usize>, HashMap<Pos, Pos>) {
        let mut proposed_next_step_counts = HashMap::new();
        let mut proposed_next_steps = HashMap::new();

        for elf_pos in self.elf_positions.iter() {
            for proposal in Proposal::proposals(round) {
                if !proposal
                    .check
                    .into_iter()
                    .any(|movement| self.elf_positions.contains(&elf_pos.add(movement)))
                {
                    let pos = elf_pos.add(proposal.head);
                    *proposed_next_step_counts.entry(pos.clone()).or_insert(0) += 1;
                    proposed_next_steps.insert(elf_pos.clone(), pos);
                    break;
                }
            }
        }

        (proposed_next_step_counts, proposed_next_steps)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Pos(i64, i64);

impl Pos {
    fn add(&self, movement: Movement) -> Self {
        let delta = movement.delta();
        Self(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Clone, Copy)]
enum Movement {
    N,
    NW,
    NE,
    S,
    SW,
    SE,
    W,
    E,
}

impl Movement {
    fn delta(&self) -> (i64, i64) {
        match self {
            Self::N => (-1, 0),
            Self::NW => (-1, -1),
            Self::NE => (-1, 1),
            Self::S => (1, 0),
            Self::SW => (1, -1),
            Self::SE => (1, 1),
            Self::W => (0, -1),
            Self::E => (0, 1),
        }
    }
}

#[derive(Clone)]
struct Proposal {
    check: Vec<Movement>,
    head: Movement,
}

impl Proposal {
    fn proposals(round: usize) -> impl IntoIterator<Item = Self> {
        [Self::north(), Self::south(), Self::west(), Self::east()]
            .into_iter()
            .cycle()
            .skip(round % 4)
            .take(4)
    }

    fn north() -> Self {
        Self {
            check: vec![Movement::N, Movement::NW, Movement::NE],
            head: Movement::N,
        }
    }

    fn south() -> Self {
        Self {
            check: vec![Movement::S, Movement::SW, Movement::SE],
            head: Movement::S,
        }
    }

    fn west() -> Self {
        Self {
            check: vec![Movement::W, Movement::SW, Movement::NW],
            head: Movement::W,
        }
    }

    fn east() -> Self {
        Self {
            check: vec![Movement::E, Movement::SE, Movement::NE],
            head: Movement::E,
        }
    }
}

impl FromStr for World {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elf_positions = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, c)| (row, col, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(row, col, _)| Pos(row as i64, col as i64))
            .collect();

        Ok(Self { elf_positions })
    }
}

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(23, 1, p1));
inventory::submit!(Solution::new(23, 2, p2));
