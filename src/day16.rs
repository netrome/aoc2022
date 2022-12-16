pub fn p1(input: &str) -> String {
    let graph = input.parse().unwrap();

    find_max_pressure(&graph).to_string()
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn find_max_pressure(graph: &Graph) -> u32 {
    let mut step_nodes = vec![Rc::new(PathNode::genesis())];

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

    fn grow(node: Rc<Self>, graph: &Graph) -> impl Iterator<Item = Rc<Self>> + '_ {
        graph.neighbors(&node.pos).map(move |vault| {
            Rc::new(Self {
                score: node.score + node.flow_rate,
                flow_rate: node.flow_rate,
                open_valves: node.open_valves.clone(),
                pos: vault.id,
                last: Some(node.clone()),
            })
        })
    }

    fn open(node: Rc<Self>, graph: &Graph) -> Option<Rc<Self>> {
        if !node.open_valves.contains(&node.pos) {
            let flow_add = graph.valves.get(&node.pos).unwrap().flow_rate;

            if flow_add == 0 {
                return None;
            }

            let mut open_valves = node.open_valves.clone();
            open_valves.insert(node.pos);

            Some(Rc::new(Self {
                score: node.score,
                flow_rate: node.flow_rate + flow_add,
                open_valves,
                pos: node.pos,
                last: Some(node),
            }))
        } else {
            None
        }
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
