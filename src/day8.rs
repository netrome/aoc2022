pub fn p1(input: &str) -> String {
    let mut grid = populate_grid(input);
    grid.observe_trees();

    grid.map
        .values()
        .filter(|tree| tree.is_visible)
        .count()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let grid = populate_grid(input);
    grid.map
        .keys()
        .map(|(row, col)| grid.compute_score(*row, *col))
        .max()
        .unwrap()
        .to_string()
}

fn populate_grid(input: &str) -> Grid {
    let mut map = HashMap::new();

    let rows = input.split_whitespace().enumerate().count();
    let cols = input.split_whitespace().next().unwrap().chars().count();

    for (row, line) in input.split_whitespace().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let height: usize = c.to_digit(10).expect("No digit") as usize;

            map.insert((row, col), Tree::new(height));
        }
    }

    Grid { map, rows, cols }
}

#[derive(Debug)]
struct Grid {
    map: HashMap<(usize, usize), Tree>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn observe_trees(&mut self) {
        for row in 0..self.rows {
            self.observe_lr(row);
            self.observe_rl(row);
        }

        for col in 0..self.cols {
            self.observe_td(col);
            self.observe_dt(col);
        }
    }

    fn observe_lr(&mut self, row: usize) {
        let tree = self.map.get_mut(&(row, 0)).unwrap();
        tree.is_visible = true;

        let mut horizon = tree.height;
        let mut idx = 1;

        while let Some(tree) = self.map.get_mut(&(row, idx)) {
            if tree.height > horizon {
                tree.is_visible = true;
                horizon = tree.height;
            }
            idx += 1;
        }
    }

    fn observe_rl(&mut self, row: usize) {
        let tree = self.map.get_mut(&(row, self.rows - 1)).unwrap();
        tree.is_visible = true;

        let mut horizon = tree.height;
        let mut idx = 1;

        while idx < self.cols {
            let tree = self.map.get_mut(&(row, self.cols - idx - 1)).unwrap();

            if tree.height > horizon {
                tree.is_visible = true;
                horizon = tree.height;
            }
            idx += 1;
        }
    }

    fn observe_td(&mut self, col: usize) {
        let tree = self.map.get_mut(&(0, col)).unwrap();
        tree.is_visible = true;

        let mut horizon = tree.height;
        let mut idx = 1;

        while let Some(tree) = self.map.get_mut(&(idx, col)) {
            if tree.height > horizon {
                tree.is_visible = true;
                horizon = tree.height;
            }
            idx += 1;
        }
    }

    fn observe_dt(&mut self, col: usize) {
        let tree = self.map.get_mut(&(self.rows - 1, col)).unwrap();
        tree.is_visible = true;

        let mut horizon = tree.height;
        let mut idx = 1;

        while idx < self.rows {
            let tree = self.map.get_mut(&(self.rows - idx - 1, col)).unwrap();
            if tree.height > horizon {
                tree.is_visible = true;
                horizon = tree.height;
            }
            idx += 1;
        }
    }

    fn compute_score(&self, row: usize, col: usize) -> usize {
        let height = self.map.get(&(row, col)).expect("No tree :O!").height;

        let (mut l, mut r, mut t, mut d) = (0, 0, 0, 0);

        while let Some(tree) = self.map.get(&(row + r + 1, col)) {
            r += 1;

            if tree.height >= height {
                break;
            }
        }

        while let Some(tree) = self
            .map
            .get(&((row - l).checked_sub(1).unwrap_or(usize::MAX), col))
        {
            l += 1;

            if tree.height >= height {
                break;
            }
        }

        while let Some(tree) = self.map.get(&(row, col + t + 1)) {
            t += 1;

            if tree.height >= height {
                break;
            }
        }

        while let Some(tree) = self
            .map
            .get(&(row, (col - d).checked_sub(1).unwrap_or(usize::MAX)))
        {
            d += 1;

            if tree.height >= height {
                break;
            }
        }

        l * r * t * d
    }
}

#[derive(Debug)]
struct Tree {
    height: usize,
    is_visible: bool,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self {
            height,
            is_visible: false,
        }
    }
}

use std::collections::HashMap;

use crate::solution::Solution;
inventory::submit!(Solution::new(8, 1, p1));
inventory::submit!(Solution::new(8, 2, p2));
