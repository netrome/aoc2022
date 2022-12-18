pub fn p1(input: &str) -> String {
    let mut rocks_iter = rocks();
    let mut chamber = Chamber::new();
    chamber.insert_rock(rocks_iter.next().unwrap());

    for push in pushes(input).take(2022) {
        chamber.push(push);
        chamber.fall();

        if chamber.falling_rock.is_none() {
            chamber.insert_rock(rocks_iter.next().unwrap())
        }
    }

    chamber.height.to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn rocks() -> impl Iterator<Item = Rock> {
    let rock1 = [(0, 0), (1, 0), (2, 0), (3, 0)].as_ref().into();
    let rock2 = [(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)].as_ref().into();
    let rock3 = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].as_ref().into();
    let rock4 = [(0, 0), (0, 1), (0, 2), (0, 3)].as_ref().into();
    let rock5 = [(0, 0), (1, 0), (0, 1), (1, 1)].as_ref().into();

    [rock1, rock2, rock3, rock4, rock5].into_iter().cycle()
}

fn pushes(input: &str) -> impl Iterator<Item = Push> + '_ {
    input
        .trim()
        .chars()
        .map(|c| c.try_into().expect("Crap!"))
        .cycle()
}

struct Chamber {
    items: HashSet<Pos>,
    falling_rock: Option<(Pos, Rock)>,
    height: usize,
}

impl Chamber {
    fn new() -> Self {
        Self {
            items: HashSet::new(),
            falling_rock: None,
            height: 0,
        }
    }

    fn push(&mut self, push: Push) {
        let rock = self.falling_rock.as_ref().unwrap();
        let new_pos = push.on(&rock.0);

        if new_pos.0 + rock.1.width < 7 {
            self.falling_rock.as_mut().unwrap().0 = new_pos;
        }
    }

    fn rock_points(&self) -> Vec<Pos> {
        let rock = self.falling_rock.as_ref().unwrap();

        rock.1
            .points
            .iter()
            .map(|point| (rock.0 .0 + point.0, rock.0 .1 + point.1))
            .collect()
    }

    fn fall(&mut self) {
        let rock = self.falling_rock.as_ref().unwrap();

        if self
            .rock_points()
            .iter()
            .any(|pos| pos.1 == 0 || self.items.contains(&(pos.0, pos.1.saturating_sub(1))))
        {
            self.materialize();
        } else {
            self.falling_rock.as_mut().unwrap().0 .1 = rock.0 .1.saturating_sub(1);
        }
    }

    fn materialize(&mut self) {
        for point in self.rock_points() {
            self.items.insert(point);
        }

        self.height += self.falling_rock.as_ref().unwrap().1.height;
        self.falling_rock = None;
    }

    fn insert_rock(&mut self, rock: Rock) {
        self.falling_rock = Some(((3, self.height + 4), rock));
    }
}

#[derive(Clone)]
struct Rock {
    width: usize,
    height: usize,
    points: Vec<Pos>,
}

impl From<&[Pos]> for Rock {
    fn from(points: &[Pos]) -> Self {
        let width = points.iter().map(|point| point.1).max().unwrap() + 1;
        let height = points.iter().map(|point| point.0).max().unwrap() + 1;

        let points = points.into_iter().cloned().collect();
        Self {
            width,
            height,
            points,
        }
    }
}

#[derive(Clone, Copy)]
enum Push {
    Left,
    Right,
}

impl Push {
    fn on(&self, pos: &Pos) -> Pos {
        match self {
            Self::Left => (pos.0.saturating_sub(1), pos.1),
            Self::Right => (pos.0.saturating_add(1), pos.1),
        }
    }
}

impl TryFrom<char> for Push {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => anyhow::bail!("Imparsibru!"),
        })
    }
}

type Pos = (usize, usize);

use std::collections::HashSet;

use crate::solution::Solution;
inventory::submit!(Solution::new(17, 1, p1));
inventory::submit!(Solution::new(17, 2, p2));
