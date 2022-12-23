pub fn p1(input: &str) -> String {
    let mut decrypter = parse_input(input);

    for id in 0..decrypter.entries.len() {
        decrypter.move_number(id);
    }

    let zero_idx = decrypter.index_of_val(0);

    [1000, 2000, 3000]
        .iter()
        .map(|offset| decrypter.entries[(zero_idx + offset) % decrypter.entries.len()].val)
        .sum::<i64>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut decrypter = parse_input(input);

    for entry in decrypter.entries.iter_mut() {
        entry.val *= 811589153;
    }

    for _ in 0..10 {
        for id in 0..decrypter.entries.len() {
            decrypter.move_number(id);
        }
    }

    let zero_idx = decrypter.index_of_val(0);

    [1000, 2000, 3000]
        .iter()
        .map(|offset| decrypter.entries[(zero_idx + offset) % decrypter.entries.len()].val)
        .sum::<i64>()
        .to_string()
}

fn parse_input(input: &str) -> Decrypter {
    Decrypter::new(input.split_whitespace().map(|s| s.parse().unwrap()))
}

struct Entry {
    id: usize,
    val: i64,
}

struct Decrypter {
    entries: Vec<Entry>,
}

impl Decrypter {
    fn new(items: impl IntoIterator<Item = i64>) -> Self {
        let mut entries: Vec<Entry> = items
            .into_iter()
            .enumerate()
            .map(|(id, val)| Entry { id, val })
            .collect();

        Self { entries }
    }

    fn index_of_val(&self, val: i64) -> usize {
        let (idx, _) = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.val == val)
            .expect("ID does not exist");

        idx
    }

    fn move_number(&mut self, id: usize) {
        let (idx, _) = self
            .entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.id == id)
            .expect("ID does not exist");

        let entry = self.entries.remove(idx);

        let next_idx = modulo(idx as i64 + entry.val, self.entries.len());

        self.entries.insert(next_idx, entry);
    }
}

fn modulo(dividend: i64, divisor: usize) -> usize {
    let ans = modulo_i64(dividend, divisor as i64) as usize;
    ans
}

fn modulo_i64(dividend: i64, divisor: i64) -> i64 {
    if divisor <= 0 {
        panic!("Noooo")
    }

    if dividend >= 0 {
        dividend % divisor
    } else {
        modulo_i64(
            dividend + (divisor * ((dividend.abs() / divisor) + 1)),
            divisor,
        )
    }
}

use crate::solution::Solution;
inventory::submit!(Solution::new(20, 1, p1));
inventory::submit!(Solution::new(20, 2, p2));
