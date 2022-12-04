fn main() -> anyhow::Result<()> {
    let day = 4;
    let part = 2;

    println!("Day {} part {}:", day, part);

    let input_file = format!("./input/d{}.txt", day);

    let input = std::fs::read_to_string(input_file)?;

    let solution = match (day, part) {
        (1, 1) => day1::p1,
        (1, 2) => day1::p2,
        (2, 1) => day2::p1,
        (2, 2) => day2::p2,
        (3, 1) => day3::p1,
        (3, 2) => day3::p2,
        (4, 1) => day4::p1,
        (4, 2) => day4::p2,
        (_, _) => panic!("Not solved"),
    };

    let ans = solution(&input);
    println!("{}", ans);

    Ok(())
}

inventory::collect!(solution::Solution);

mod day1;
mod day2;
mod day3;
mod day4;
mod solution;
