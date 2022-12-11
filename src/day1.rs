pub fn p1(input: &str) -> String {
    calorie_sums(input).max().unwrap().to_string()
}

pub fn p2(input: &str) -> String {
    k_largest(3, calorie_sums(input))
        .into_iter()
        .sum::<usize>()
        .to_string()
}

fn calorie_sums(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split("\n\n").map(|row| {
        row.split_whitespace()
            .map(|item| item.parse::<usize>().unwrap())
            .sum::<usize>()
    })
}

pub fn k_largest(k: usize, iter: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut iter = iter.map(Reverse);

    let mut heap = BinaryHeap::new();

    for _ in 0..k {
        heap.push(iter.next().unwrap());
    }

    iter.for_each(|item| {
        if item.0 > heap.peek().unwrap().0 {
            heap.pop();
            heap.push(item)
        }
    });

    heap.into_iter().map(|Reverse(item)| item).collect()
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::solution::Solution;
inventory::submit!(Solution::new(1, 1, p1));
inventory::submit!(Solution::new(1, 2, p2));
