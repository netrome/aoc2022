fn main() -> anyhow::Result<()> {
    let input_file = "./input/d1.txt";

    let input = std::fs::read_to_string(input_file)?;

    println!("Day 1 p1: {}", day_1_p1(&input));
    println!("Day 1 p2: {}", day_1_p2(&input));

    Ok(())
}

fn day_1_p1(input: &str) -> usize {
    calorie_sums(input).max().unwrap()
}

fn day_1_p2(input: &str) -> usize {
    k_largest(3, calorie_sums(input)).into_iter().sum()
}

fn calorie_sums(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split("\n\n").map(|row| {
        row.split_whitespace()
            .map(|item| item.parse::<usize>().unwrap())
            .sum::<usize>()
    })
}

fn k_largest(k: usize, iter: impl Iterator<Item = usize>) -> Vec<usize> {
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
