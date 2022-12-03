pub fn p1(input: &str) -> String {
    strategy(input).map(score).sum::<i32>().to_string()
}

pub fn p2(input: &str) -> String {
    strategy(input)
        .map(patch_recommendation)
        .map(score)
        .sum::<i32>()
        .to_string()
}

fn strategy(input: &str) -> impl Iterator<Item = Round> + '_ {
    input
        .trim()
        .split("\n")
        .map(|line| line.parse::<Round>().expect("Parse failure"))
}

fn score(round: Round) -> i32 {
    let outcome_score = match (3 + round.recommendation.value() - round.opponent.value()) % 3 {
        0 => 3,
        1 => 6,
        _ => 0,
    };

    let score = outcome_score + round.recommendation.value();

    score
}

#[derive(Debug)]
struct Round {
    opponent: Shape,
    recommendation: Shape,
}

fn patch_recommendation(round: Round) -> Round {
    let recommendation = match round.recommendation {
        Shape::Rock => Shape::from_value(round.opponent.value() - 1),
        Shape::Paper => round.opponent.clone(),
        Shape::Scissors => Shape::from_value(round.opponent.value() + 1),
    };

    Round {
        recommendation,
        ..round
    }
}

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_value(val: i32) -> Self {
        let val = ((val + 2) % 3) + 1;

        match val {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => panic!("Not supported"),
        }
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Not supported"),
        }
    }
}

impl std::str::FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, recommendation) =
            sscanf::sscanf!(s, "{char} {char}").expect("Failed to scan line");

        Ok(Self {
            opponent: opponent.into(),
            recommendation: recommendation.into(),
        })
    }
}
