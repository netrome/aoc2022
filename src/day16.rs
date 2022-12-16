pub fn p1(input: &str) -> String {
    let graph = input.parse().unwrap();

    search_max_release(&graph);
    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn search_max_release(graph: &Graph) -> u32 {
    let mut seen = HashMap::new();

    let starting_point = SearchPoint::starting_point();
    let mut next = next_steps(&graph, &starting_point, 0);

    seen.insert(starting_point, 0);

    let mut i = 0;

    while let Some((score, point)) = next.pop() {
        for (score, step) in next_steps(graph, &point, score) {
            if seen
                .get(&step)
                .map(|step_score| step_score < &score)
                .unwrap_or(true)
                && step.minute < 30
            {
                seen.insert(step.clone(), score);
                next.push((score, step));
            }
        }

        if i % 100000 == 0 {
            println!("Totoal: {:?}", next.len());
        }
        i += 1;
    }

    println!("Done! Seen: {:?}", seen);

    todo!();
}

fn next_steps(graph: &Graph, point: &SearchPoint, score: u32) -> BinaryHeap<(u32, SearchPoint)> {
    let mut next_steps: BinaryHeap<_> =
        point.moves(graph).into_iter().map(|m| (score, m)).collect();
    if let Some(step) = point.open() {
        let added_score = graph
            .valves
            .get(&step.position)
            .expect("Crap")
            .release_value(step.minute);

        next_steps.push((score + added_score, step));
    }

    next_steps
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct SearchPoint {
    minute: u32,
    open_valves: BTreeSet<ValveId>,
    position: ValveId,
}

impl SearchPoint {
    fn starting_point() -> Self {
        let position = ValveId('A', 'A');
        let open_valves = BTreeSet::new();
        let minute = 0;

        Self {
            minute,
            position,
            open_valves,
        }
    }

    fn open(&self) -> Option<Self> {
        let mut res = self.clone();
        if res.open_valves.insert(self.position) {
            res.minute += 1;
            Some(res)
        } else {
            None
        }
    }

    fn moves(&self, graph: &Graph) -> Vec<Self> {
        graph
            .valves
            .get(&self.position)
            .expect("No?")
            .tunnels
            .iter()
            .map(|valve_id| Self {
                minute: self.minute + 1,
                open_valves: self.open_valves.clone(),
                position: *valve_id,
            })
            .collect()
    }
}

struct Graph {
    valves: HashMap<ValveId, Valve>,
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s
            .trim()
            .lines()
            .map(|line| line.parse().expect("Merda"))
            .map(|valve: Valve| (valve.id, valve))
            .collect();

        Ok(Self { valves })
    }
}

#[derive(Debug)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    tunnels: Vec<ValveId>,
}

impl Valve {
    fn release_value(&self, minute: u32) -> u32 {
        30u32.saturating_sub(minute) * self.flow_rate
    }
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, flow_rate, _, _, tunnels) = sscanf::sscanf!(
            s,
            "Valve {String} has flow rate={u32}; {String} to {String} {String}"
        )
        .expect("Noooooooparse");

        let id = id.parse().expect("Failed to parse ID");
        let tunnels = tunnels
            .trim()
            .split(", ")
            .map(|id| id.parse().expect("Crap"))
            .collect();

        Ok(Self {
            id,
            flow_rate,
            tunnels,
        })
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct ValveId(char, char);

impl FromStr for ValveId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();

        let (c1, c2) = (chars.next().unwrap(), chars.next().unwrap());

        Ok(Self(c1, c2))
    }
}

use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque},
    str::{FromStr, RSplitTerminator},
};

use crate::solution::Solution;
inventory::submit!(Solution::new(16, 1, p1));
inventory::submit!(Solution::new(16, 2, p2));
