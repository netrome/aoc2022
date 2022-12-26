fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_file = cli
        .input_file
        .unwrap_or(format!("./input/d{}.txt", cli.day));

    let input = std::fs::read_to_string(input_file)?;

    let solution = inventory::iter::<solution::Solution>
        .into_iter()
        .find(|solution| solution.day == cli.day && solution.part == cli.part)
        .unwrap_or_else(|| panic!("No solution found for day {} part {}", cli.day, cli.part));

    let ans = (solution.run)(&input);
    println!("{}", ans);

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// AoC day
    #[arg(short, long)]
    day: usize,

    /// AoC part
    #[arg(short, long)]
    part: usize,

    /// Input override, otherwise defaults to ./input/d{day}.txt
    #[arg(short, long)]
    input_file: Option<String>,
}

inventory::collect!(solution::Solution);

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solution;

use clap::Parser;
