pub fn p1(input: &str) -> String {
    let mut rocks_iter = rocks();
    let mut chamber = Chamber::new();
    chamber.insert_rock(rocks_iter.next().unwrap());

    let mut fallen_rocks = 0;

    for push in pushes(input) {
        chamber.push(push);
        chamber.fall();

        if chamber.falling_rock.is_none() {
            fallen_rocks += 1;

            if fallen_rocks == 2022 {
                break;
            }

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

#[derive(Debug, Clone)]
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

        if !self.is_collision(new_pos) {
            self.falling_rock.as_mut().unwrap().0 = new_pos;
        }
    }

    fn is_collision(&self, new_pos: Pos) -> bool {
        let rock = self.falling_rock.as_ref().unwrap();

        let ans = rock
            .1
            .points
            .iter()
            .map(|point| (new_pos.0 + point.0, new_pos.1 + point.1))
            .any(|point| Self::out_of_bounds(&point) || self.items.contains(&point));

        if ans {}

        ans
    }

    fn out_of_bounds(pos: &Pos) -> bool {
        pos.0 >= 7
    }

    fn fall(&mut self) {
        let rock_position = self.falling_rock.as_ref().unwrap().0;
        let next_position = (rock_position.0, rock_position.1.saturating_sub(1));

        if rock_position == next_position || self.is_collision(next_position) {
            self.materialize();
        } else {
            self.falling_rock.as_mut().unwrap().0 = next_position;
        }
    }

    fn materialize(&mut self) {
        for point in self.rock_points() {
            if point.1 >= self.height {
                self.height = point.1 + 1
            }
            self.items.insert(point);
        }

        self.falling_rock = None;
    }

    fn insert_rock(&mut self, rock: Rock) {
        self.falling_rock = Some(((2, self.height + 3), rock));
    }

    fn rock_points(&self) -> Vec<Pos> {
        self.points(self.falling_rock.as_ref().unwrap().0)
    }

    fn points(&self, new_pos: Pos) -> Vec<Pos> {
        let rock = self.falling_rock.as_ref().unwrap();

        rock.1
            .points
            .iter()
            .map(|point| (new_pos.0 + point.0, new_pos.1 + point.1))
            .collect()
    }

    fn show(&self) -> String {
        let rock_points: HashSet<_> = if self.falling_rock.is_some() {
            self.rock_points().into_iter().collect()
        } else {
            HashSet::new()
        };

        let mut out = Vec::new();
        for y in (0..self.height + 9).rev() {
            for x in 0..7 {
                let is_item = self.items.contains(&(x, y));
                let is_falling = rock_points.contains(&(x, y));

                let c = match (is_item, is_falling) {
                    (true, true) => '$',
                    (true, false) => '#',
                    (false, true) => 'o',
                    (false, false) => '.',
                };

                out.push(c)
            }
            out.push('\n')
        }

        out.into_iter().collect()
    }
}

#[derive(Clone, Debug)]
struct Rock {
    width: usize,
    points: Vec<Pos>,
}

impl From<&[Pos]> for Rock {
    fn from(points: &[Pos]) -> Self {
        let width = points.iter().map(|point| point.1).max().unwrap() + 1;

        let points = points.into_iter().cloned().collect();
        Self { width, points }
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
