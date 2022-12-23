pub fn p1(input: &str) -> String {
    let world = parse_input(input);

    points_without_beacon(&world, 2000000).to_string()
}

pub fn p2(input: &str) -> String {
    let world = parse_input(input);

    find_beacon(&world, 4000000).to_string()
}

fn find_beacon(world: &World, ymax: i64) -> i64 {
    for y in 0..ymax {
        let intervals = find_intervals(world, y);
        if intervals.len() > 1 {
            let mut inter = intervals.into_iter();
            let mut current = inter.next().unwrap();

            for next in inter {
                if next.0 - current.1 == 2 {
                    return (next.0 + 1) * 4000000 + y;
                }

                current = next;
            }
        }
    }
    0
}

fn points_without_beacon(world: &World, y: i64) -> usize {
    let joined_intervals = find_intervals(world, y);

    let total_number_of_positions: usize = joined_intervals.iter().map(|int| int.len()).sum();
    let positions_with_sensor = world
        .sensors
        .iter()
        .filter(|sensor| sensor.pos.1 == y && sensor.pos.in_any_interval(&joined_intervals))
        .count();

    let positions_with_beacon: usize = world
        .beacons
        .iter()
        .filter(|beacon| beacon.pos.1 == y && beacon.pos.in_any_interval(&joined_intervals))
        .count();

    total_number_of_positions - positions_with_sensor - positions_with_beacon
}

fn find_intervals(world: &World, y: i64) -> Vec<Interval> {
    let mut intervals: Vec<Interval> = world
        .sensors
        .iter()
        .filter_map(|sensor| sensor.no_beacon_land(y))
        .collect();

    intervals.sort();

    let mut joined_intervals = Vec::new();
    let mut iter = intervals.into_iter();

    let mut int1 = iter.next().expect("No intervals at this y");

    for int2 in iter {
        if int1.overlaps(&int2) {
            int1 = int1.merge(&int2);
        } else {
            joined_intervals.push(int1);
            int1 = int2;
        }
    }
    joined_intervals.push(int1);
    joined_intervals
}

fn parse_input(input: &str) -> World {
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();

    for (sensor, beacon) in input.trim().lines().map(read_line) {
        sensors.push(sensor);
        beacons.insert(beacon);
    }

    World {
        sensors,
        beacons: beacons.into_iter().collect(),
    }
}

fn read_line(line: &str) -> (Sensor, Beacon) {
    let (sx, sy, bx, by) = sscanf::sscanf!(
        line.trim(),
        "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
    )
    .expect("Failed to parse line");

    let sensor_pos = Pos(sx, sy);
    let beacon_pos = Pos(bx, by);

    let dist = sensor_pos.distance(&beacon_pos);

    let sensor = Sensor {
        pos: sensor_pos,
        beacon_distance: dist,
    };

    let beacon = Beacon { pos: beacon_pos };

    (sensor, beacon)
}

struct World {
    sensors: Vec<Sensor>,
    beacons: Vec<Beacon>,
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon_distance: i64,
}

impl Sensor {
    fn no_beacon_land(&self, y: i64) -> Option<Interval> {
        let diff = (self.pos.1 - y).abs();

        if diff <= self.beacon_distance {
            let dist = self.beacon_distance - diff;
            let interval = Interval::new(self.pos.0 - dist, self.pos.0 + dist);
            Some(interval)
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Beacon {
    pos: Pos,
}

impl Pos {
    fn in_any_interval(&self, intervals: &[Interval]) -> bool {
        intervals.iter().any(|interval| interval.contains(self.0))
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos(i64, i64);

impl Pos {
    fn distance(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Interval(i64, i64);

impl Interval {
    fn new(min: i64, max: i64) -> Self {
        if max < min {
            panic!("Impossibruuu!")
        } else {
            Self(min, max)
        }
    }

    fn merge(&self, other: &Self) -> Interval {
        if !self.overlaps(other) {
            panic!("Noooooo overlap noo")
        } else {
            Self(self.0.min(other.0), self.1.max(other.1))
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(self.is_strictly_lower(other) || other.is_strictly_lower(self))
    }

    fn is_strictly_lower(&self, other: &Self) -> bool {
        self.1 < other.0
    }

    fn len(&self) -> usize {
        (self.1 - self.0 + 1) as usize
    }

    fn contains(&self, pos: i64) -> bool {
        self.0 <= pos && pos <= self.1
    }
}

use std::collections::HashSet;

use crate::solution::Solution;
inventory::submit!(Solution::new(15, 1, p1));
inventory::submit!(Solution::new(15, 2, p2));
