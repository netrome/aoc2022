pub fn p1(input: &str) -> String {
    let graph: Graph = input.parse().unwrap();
    let graph2: Graph2 = graph.into();
    let subgraph = graph2.valves.keys().cloned().collect();

    find_max_pressure(&graph2, subgraph, 30).to_string()
}

pub fn p2(input: &str) -> String {
    let graph: Graph = input.parse().unwrap();
    let graph2: Graph2 = graph.into();

    find_max_pressure_with_elephant(&graph2).to_string()
}

fn find_max_pressure_with_elephant(graph: &Graph2) -> u32 {
    let all_nodes: HashSet<_> = graph.valves.keys().cloned().collect();

    let mut max_pressure = 0;

    let _i = 0;

    for elephant_share in all_nodes.iter().powerset() {
        let elephant_set: HashSet<_> = elephant_share.into_iter().cloned().collect();
        let santa_set: HashSet<_> = all_nodes.difference(&elephant_set).cloned().collect();

        let elephant_pressure = find_max_pressure(graph, elephant_set, 26);
        let santa_pressure = find_max_pressure(graph, santa_set, 26);

        let pressure = elephant_pressure + santa_pressure;
        if pressure > max_pressure {
            max_pressure = pressure
        }
    }

    max_pressure
}

fn find_max_pressure(graph: &Graph2, subgraph: HashSet<ValveId>, max_time: u32) -> u32 {
    let mut high_score = Rc::new(PathNode::genesis(subgraph));
    let mut nodes_to_visit = Vec::new();
    nodes_to_visit.push(high_score.clone());

    while let Some(node) = nodes_to_visit.pop() {
        if node.score > high_score.score {
            high_score = node.clone();
        }

        nodes_to_visit.append(&mut PathNode::children(node, graph, max_time));
    }

    high_score.score
}

#[derive(Debug)]
struct PathNode {
    minute: u32,
    score: u32,
    pos: ValveId,
    to_visit: HashSet<ValveId>,
}

impl PathNode {
    fn genesis(to_visit: HashSet<ValveId>) -> Self {
        Self {
            minute: 0,
            score: 0,
            pos: ValveId::genesis(),
            to_visit,
        }
    }

    fn children(node: Rc<Self>, graph: &Graph2, max_time: u32) -> Vec<Rc<Self>> {
        let mut children = Vec::new();

        for valve_id in node.to_visit.iter() {
            let next = graph.valves.get(&valve_id).unwrap();
            let dist = next.tunnels.get(&node.pos).unwrap();

            let remaining_time = max_time.saturating_sub(node.minute + dist + 1);
            let mut to_visit = node.to_visit.clone();
            to_visit.remove(&valve_id);

            if remaining_time > 0 {
                let child = Rc::new(Self {
                    minute: node.minute + dist + 1,
                    score: node.score + next.flow_rate * remaining_time,
                    pos: next.id,
                    to_visit,
                });
                children.push(child);
            }
        }

        children
    }
}

#[derive(Debug)]
struct Graph2 {
    valves: HashMap<ValveId, Valve2>,
}

impl From<Graph> for Graph2 {
    fn from(other: Graph) -> Self {
        let mut valves = HashMap::new();

        for valve in other.valves.values() {
            let distances = other
                .distances(&valve.id)
                .into_iter()
                .filter(|(valve_id, _)| {
                    let v = other.valves.get(valve_id).unwrap();
                    v.flow_rate > 0 || v.id == ValveId::genesis()
                })
                .collect();

            let valve = Valve2 {
                id: valve.id,
                flow_rate: valve.flow_rate,
                tunnels: distances,
            };

            if valve.flow_rate > 0 || valve.id == ValveId::genesis() {
                valves.insert(valve.id, valve);
            }
        }

        Self { valves }
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

    fn distances(&self, valve: &ValveId) -> HashMap<ValveId, u32> {
        let mut distances = HashMap::new();
        let mut to_explore = VecDeque::new();
        to_explore.push_back((*valve, 0));

        while let Some((valve, dist)) = to_explore.pop_front() {
            if !distances.contains_key(&valve) {
                distances.insert(valve, dist);

                for other in self.neighbors(&valve) {
                    to_explore.push_back((other.id, dist + 1))
                }
            }
        }

        distances
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
struct Valve2 {
    id: ValveId,
    flow_rate: u32,
    tunnels: HashMap<ValveId, u32>,
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

impl std::fmt::Display for ValveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1).unwrap();
        Ok(())
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
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
    str::FromStr,
};

use crate::solution::Solution;
use itertools::Itertools;
inventory::submit!(Solution::new(16, 1, p1));
inventory::submit!(Solution::new(16, 2, p2));
