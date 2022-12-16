pub fn p1(input: &str) -> String {
    let graph: Graph = input.parse().unwrap();
    let graph2: Graph2 = graph.into();

    find_max_pressure(&graph2).to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn find_max_pressure(graph: &Graph2) -> u32 {
    let all_nodes = graph.valves.keys().cloned().collect();
    let mut step_nodes = vec![Rc::new(PathNode::genesis(all_nodes))];

    for i in 0..30 {
        let mut next_step_nodes = Vec::new();

        for node in step_nodes {
            if let Some(opened) = PathNode::open(node.clone(), graph) {
                next_step_nodes.push(opened);
            }

            next_step_nodes.extend(PathNode::grow(node, graph));
        }

        step_nodes = next_step_nodes;
        step_nodes.sort_by_key(|node| u32::MAX - node.flow_rate);
        step_nodes.truncate(10000);
    }

    let max_score = step_nodes.iter().map(|node| node.score).max().unwrap();
    let best_node = step_nodes
        .iter()
        .find(|node| node.score == max_score)
        .unwrap();

    println!("{}", string_path(&PathNode::get_path(best_node.clone())));

    max_score
}

struct PathNode {
    minute: u32,
    score: u32,
    pos: ValveId,
    to_visit: HashSet<ValveId>,
    last: Option<Rc<PathNode>>,
}

impl PathNode {
    fn genesis(mut to_visit: HashSet<ValveId>) -> Self {
        Self {
            minute: 0,
            score: 0,
            pos: ValveId::genesis(),
            to_visit,
            last: None,
        }
    }

    fn children(node: Rc<Self>, graph: &Graph2) -> Vec<Rc<Self>> {
        let mut children = Vec::new();

        for valve_id in node.to_visit {
            let next = graph.valves.get(&valve_id).unwrap();
            let dist = next.tunnels.get(&node.pos).unwrap();

            let remaining_time = 30u32.saturating_sub(node.minute + dist + 1);
            let mut to_visit = node.to_visit.clone();
            to_visit.remove(&valve_id);

            if remaining_time > 0 {
                let child = Rc::new(Self {
                    minute: node.minute + dist + 1,
                    score: node.score + next.flow_rate * remaining_time,
                    pos: next.id,
                    to_visit,
                    last: Some(node.clone()),
                });
                children.push(child);
            }
        }

        children
    }

    fn get_path(node: Rc<Self>) -> Vec<ValveId> {
        if let Some(last) = node.last.clone() {
            let mut path = Self::get_path(last);
            path.push(node.pos);
            path
        } else {
            vec![node.pos]
        }
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

            valves.insert(valve.id, valve);
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
            }

            for other in self.neighbors(&valve) {
                to_explore.push_back((other.id, dist + 1))
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
        write!(f, "{}{}", self.0, self.1);
        Ok(())
    }
}

fn string_path(path: &[ValveId]) -> String {
    path.iter().map(|id| format!("{} >", id)).collect()
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
