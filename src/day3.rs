pub fn p1(input: &str) -> String {
    input
        .split_whitespace()
        .map(parse_line)
        .map(priority)
        .sum::<u32>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut lines = input.split_whitespace();

    let mut chars = Vec::new();

    while let Some(line) = lines.next() {
        let s1: HashSet<char> = line.chars().collect();
        let s2: HashSet<char> = lines.next().unwrap().chars().collect();
        let s3: HashSet<char> = lines.next().unwrap().chars().collect();

        let intersect: Vec<char> = s3
            .intersection(&s1.intersection(&s2).copied().collect())
            .copied()
            .collect();

        assert_eq!(intersect.len(), 1);

        chars.push(*intersect.first().unwrap());
    }

    chars.into_iter().map(priority).sum::<u32>().to_string()
}

fn parse_line(line: &str) -> char {
    let (first, second) = line.split_at(line.len() / 2);
    assert_eq!(first.len(), second.len());

    let s1: HashSet<char> = first.chars().collect();
    let s2: HashSet<char> = second.chars().collect();

    let intersect: Vec<&char> = s1.intersection(&s2).collect();

    assert_eq!(intersect.len(), 1);

    **intersect.first().unwrap()
}

fn priority(c: char) -> u32 {
    if (c as u32) < ('a' as u32) {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

use std::collections::HashSet;

use crate::solution::Solution;
inventory::submit!(Solution::new(3, 1, p1));
inventory::submit!(Solution::new(3, 2, p2));
