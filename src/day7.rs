pub fn p1(input: &str) -> String {
    let root = parse_input(input);
    sum_size_at_most_k(root, 100000).to_string()
}

pub fn p2(input: &str) -> String {
    let root = parse_input(input);
    let unused_space = 70000000 - root.lock().unwrap().size();
    let space_to_delete = 30000000 - unused_space;

    find_dir_to_delete(root, space_to_delete).to_string()
}

fn parse_input(input: &str) -> Arc<Mutex<Directory>> {
    let root = Arc::new(Mutex::new(Directory::new("/".to_string(), None)));
    let mut explorer = Explorer::new(root.clone());

    for line in input.trim().split('\n') {
        if let Ok(Cmd::Cd(dir)) = line.parse() {
            explorer.cd(&dir);
        }

        if let Ok(node) = line.parse() {
            explorer.add(node);
        }
    }

    root
}

fn sum_size_at_most_k(dir: Arc<Mutex<Directory>>, k: usize) -> usize {
    let mut sum = dir.lock().unwrap().size();

    if sum > k {
        sum = 0
    }

    for item in dir.lock().unwrap().items.values() {
        if let FSNode::Dir(dir) = item {
            sum += sum_size_at_most_k(dir.clone(), k)
        }
    }

    sum
}

fn find_dir_to_delete(dir: Arc<Mutex<Directory>>, space: usize) -> usize {
    let mut min = dir.lock().unwrap().size();

    if min < space {
        min = usize::MAX;
    }

    for item in dir.lock().unwrap().items.values() {
        if let FSNode::Dir(dir) = item {
            min = find_dir_to_delete(dir.clone(), space).min(min)
        }
    }

    min
}

struct Explorer {
    root: Arc<Mutex<Directory>>,
    workdir: Arc<Mutex<Directory>>,
}

impl Explorer {
    fn new(root: Arc<Mutex<Directory>>) -> Self {
        Self {
            workdir: root.clone(),
            root,
        }
    }

    fn cd(&mut self, dir: &str) {
        let new_dir = match dir {
            ".." => self
                .workdir
                .lock()
                .unwrap()
                .parent
                .clone()
                .expect("No parent to cd"),

            "/" => self.root.clone(),

            dir => self
                .workdir
                .lock()
                .unwrap()
                .items
                .get(dir)
                .expect("Dir does not exist")
                .as_dir_arc(),
        };

        self.workdir = new_dir
    }

    fn add(&self, mut node: FSNode) {
        node.set_parent(self.workdir.clone());

        self.workdir
            .lock()
            .unwrap()
            .items
            .insert(node.name().to_string(), node);
    }
}

enum Cmd {
    Ls,
    Cd(String),
}

impl FromStr for Cmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = sscanf::sscanf!(s, "$ {String}")
            .map_err(|_| anyhow::anyhow!("Failed to parse command: {}", s))?;

        let parts = cmd.split_whitespace().collect::<Vec<&str>>();

        let command = match parts.as_slice() {
            ["ls"] => Cmd::Ls,
            ["cd", dir] => Cmd::Cd(dir.to_string()),
            _ => anyhow::bail!("Failed to identify command: {}", s),
        };

        Ok(command)
    }
}

#[derive(Debug, Clone)]
enum FSNode {
    Dir(Arc<Mutex<Directory>>),
    File(File),
}

impl FSNode {
    fn as_dir_arc(&self) -> Arc<Mutex<Directory>> {
        match self {
            Self::Dir(directory) => directory.clone(),
            _ => panic!("Node is not directory"),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Dir(dir) => dir.lock().unwrap().name.to_string(),
            Self::File(file) => file.name.to_string(),
        }
    }

    fn set_parent(&mut self, parent: Arc<Mutex<Directory>>) {
        match self {
            Self::Dir(dir) => {
                dir.lock().unwrap().parent = Some(parent);
            }
            _ => (),
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::Dir(dir) => dir.lock().unwrap().size(),
            Self::File(file) => file.size,
        }
    }
}

impl FromStr for FSNode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(file) = s.parse() {
            return Ok(Self::File(file));
        };

        if let Ok(dir) = s.parse() {
            return Ok(Self::Dir(Arc::new(Mutex::new(dir))));
        };

        return Err(anyhow::anyhow!("Failed to parse FSNode: {}", s));
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    items: HashMap<String, FSNode>,
    parent: Option<Arc<Mutex<Directory>>>,
}

impl Directory {
    fn new(name: String, parent: Option<Arc<Mutex<Directory>>>) -> Self {
        Self {
            name,
            items: HashMap::new(),
            parent,
        }
    }

    fn size(&self) -> usize {
        self.items.values().map(|node| node.size()).sum()
    }
}

impl FromStr for Directory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = sscanf::sscanf!(s, "dir {String}")
            .map_err(|_| anyhow::anyhow!("Couldn't parse Dir: {}", s))?;

        Ok(Self {
            name,
            items: HashMap::new(),
            parent: None,
        })
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, name) = sscanf::sscanf!(s, "{usize} {String}")
            .map_err(|_| anyhow::anyhow!("Couldn't parse File: {}", s))?;

        Ok(Self { name, size })
    }
}

use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;

use crate::solution::Solution;
inventory::submit!(Solution::new(7, 1, p1));
inventory::submit!(Solution::new(7, 2, p2));
