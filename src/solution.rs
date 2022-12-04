pub struct Solution {
    pub day: usize,
    pub part: usize,
    pub run: fn(&str) -> String,
}

impl Solution {
    pub const fn new(day: usize, part: usize, run: fn(&str) -> String) -> Self {
        Self { day, part, run }
    }
}
