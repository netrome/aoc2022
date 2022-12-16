pub fn p1(input: &str) -> String {
    //let graph = input.parse().unwrap();

    todo!();
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn find_max_pressure(graph: &Graph) -> u32 {
    todo!();

    for i in 0..30 {
        todo!();
    }
}

struct PathNode {
    score: u32,
    flow_rate: u32,
    pos: ValveId,
    open_valves: HashSet<ValveId>,
    last: Option<Rc<PathNode>>,
}

impl PathNode {
    fn genesis() -> Self {
        Self {
            score: 0,
            flow_rate: 0,
            pos: ValveId::genesis(),
            open_valves: HashSet::new(),
            last: None,
        }
    }

    fn grow(node: Rc<Self>, graph: &Graph) -> impl Iterator<Item = Self> + '_ {
        graph.neighbors(&node.pos).map(move |vault| Self {
            score: node.score + node.flow_rate,
            flow_rate: node.flow_rate,
            open_valves: node.open_valves.clone(),
            pos: vault.id,
            last: Some(node.clone()),
        })
    }

    fn open(node: Rc<Self>, graph: &Graph) -> Option<Self> {
        if !node.open_valves.contains(&node.pos) {
            let flow_add = graph.valves.get(&node.pos).unwrap().flow_rate;

            let mut open_valves = node.open_valves.clone();
            open_valves.insert(node.pos);

            Some(Self {
                score: node.score,
                flow_rate: node.flow_rate + flow_add,
                open_valves,
                pos: node.pos,
                last: Some(node),
            })
        } else {
            None
        }
    }
}

struct Graph {
    valves: HashMap<ValveId, Valve>,
}

impl Graph {
    fn neighbors(&self, valve: &ValveId) -> impl Iterator<Item = &Valve> {
        self.valves
            .get(valve)
            .expect("Waaat")
            .tunnels
            .iter()
            .map(|id| self.valves.get(id).unwrap())
    }
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

impl ValveId {
    fn genesis() -> Self {
        Self('A', 'A')
    }
}

impl FromStr for ValveId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();

        let (c1, c2) = (chars.next().unwrap(), chars.next().unwrap());

        Ok(Self(c1, c2))
    }
}

use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
    str::{FromStr, RSplitTerminator},
};

use crate::solution::Solution;
inventory::submit!(Solution::new(16, 1, p1));
inventory::submit!(Solution::new(16, 2, p2));
