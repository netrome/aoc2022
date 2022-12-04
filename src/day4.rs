pub fn p1(input: &str) -> String {
    read_range_pairs(input)
        .filter(|pair| pair.one_fully_contains_other())
        .count()
        .to_string()
}

pub fn p2(input: &str) -> String {
    read_range_pairs(input)
        .filter(|pair| pair.overlaps())
        .count()
        .to_string()
}

fn read_range_pairs(input: &str) -> impl Iterator<Item = Pair> + '_ {
    input
        .split_whitespace()
        .map(|line| line.parse().expect("Merda"))
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn is_strictly_lower(&self, other: &Self) -> bool {
        self.end < other.start
    }
}

#[derive(Debug, Copy, Clone)]
struct Pair(Range, Range);

impl Pair {
    fn one_fully_contains_other(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        !(self.0.is_strictly_lower(&self.1) || self.1.is_strictly_lower(&self.0))
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = sscanf::sscanf!(s, "{usize}-{usize}").expect("Nooo");

        Ok(Self { start, end })
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<Range> = s
            .split(',')
            .map(|range| range.parse::<Range>().expect("Crap"))
            .collect();

        assert_eq!(ranges.len(), 2);

        Ok(Self(ranges[0], ranges[1]))
    }
}

use std::str::FromStr;
