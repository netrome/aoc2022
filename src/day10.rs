pub fn p1(input: &str) -> String {
    let mut cpu = Cpu::new();

    for instruction in parse_instructions(input) {
        cpu.load(instruction);
    }

    let mut signal_strengths = Vec::new();
    while cpu.next_cycle() {
        if cpu.cycle % 40 == 19 {
            signal_strengths.push(cpu.signal_strength())
        }
    }

    signal_strengths.iter().sum::<i64>().to_string()
}

pub fn p2(input: &str) -> String {
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();

    for instruction in parse_instructions(input) {
        cpu.load(instruction);
    }

    crt.draw(cpu.register_x);
    while cpu.next_cycle() {
        crt.step();
        crt.draw(cpu.register_x);
    }

    crt.render()
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.trim().split('\n').map(|line| line.parse().unwrap())
}

#[derive(Default, Debug)]
struct Cpu {
    register_x: i64,
    cycle: usize,
    x_updates: VecDeque<i64>,
}

#[derive(Default, Debug)]
struct Crt {
    picture: HashSet<(i64, i64)>,
    x: i64,
    y: i64,
}

impl Crt {
    fn new() -> Self {
        Self {
            picture: HashSet::new(),
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self) {
        self.x += 1;
        if self.x >= 40 {
            self.x = 0;
            self.y += 1;
        }
    }

    fn draw(&mut self, sprite_x: i64) {
        if (self.x - sprite_x).abs() <= 1 {
            self.picture.insert((self.x, self.y));
        }
    }

    fn render(&self) -> String {
        (0..6)
            .map(|y| {
                format!(
                    "{}\n",
                    (0..40).map(|x| self.render_px(x, y)).collect::<String>()
                )
            })
            .collect()
    }

    fn render_px(&self, x: i64, y: i64) -> char {
        if self.picture.contains(&(x, y)) {
            '#'
        } else {
            ' '
        }
    }
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_x: 1,
            cycle: 0,
            x_updates: VecDeque::new(),
        }
    }
    fn load(&mut self, instr: Instruction) {
        match instr {
            Instruction::Addx(val) => {
                self.x_updates.push_back(0);
                self.x_updates.push_back(val);
            }
            Instruction::Noop => {
                self.x_updates.push_back(0);
            }
        }
    }

    fn next_cycle(&mut self) -> bool {
        self.cycle += 1;
        if let Some(val) = self.x_updates.pop_front() {
            self.register_x += val;
            true
        } else {
            false
        }
    }

    fn signal_strength(&self) -> i64 {
        self.register_x * (self.cycle as i64 + 1)
    }
}

enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = sscanf::sscanf!(s, "addx {i64}") {
            Ok(Self::Addx(val))
        } else if s == "noop" {
            Ok(Self::Noop)
        } else {
            anyhow::bail!("Failed to parse instruction")
        }
    }
}

use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(10, 1, p1));
inventory::submit!(Solution::new(10, 2, p2));
