fn main() -> anyhow::Result<()> {
    let day = 2;
    let part = 2;

    println!("Day {} part {}:", day, part);

    let input_file = format!("./input/d{}.txt", day);

    let input = std::fs::read_to_string(input_file)?;

    let solution = match (day, part) {
        (1, 1) => day1::p1,
        (1, 2) => day1::p2,
        (2, 1) => day2::p1,
        (2, 2) => day2::p2,
        (_, _) => panic!("Not solved"),
    };

    println!("{}", solution(&input));

    Ok(())
}

mod day1;
mod day2;
