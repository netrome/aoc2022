pub fn p1(input: &str) -> String {
    find_marker(input.trim(), 4).to_string()
}

pub fn p2(input: &str) -> String {
    find_marker(input.trim(), 14).to_string()
}

fn find_marker(input: &str, k: usize) -> usize {
    let mut buffer = VecDeque::new();
    let mut idx = 0;
    for c in input.chars() {
        buffer.push_front(c);

        if idx >= k - 1 {
            if buffer.iter().collect::<HashSet<_>>().len() == k {
                break;
            }
            buffer.pop_back();
        }

        idx += 1;
    }

    idx + 1
}

use std::collections::HashSet;
use std::collections::VecDeque;

use crate::solution::Solution;
inventory::submit!(Solution::new(6, 1, p1));
inventory::submit!(Solution::new(6, 2, p2));
