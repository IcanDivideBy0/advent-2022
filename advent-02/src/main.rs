use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Shape {
    fn round_score(&self, other: &Self) -> u32 {
        let outcome = match self {
            Self::Rock => match other {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Loose,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Loose,
            },
            Self::Scissors => match other {
                Self::Rock => Outcome::Loose,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        };

        (*self as u32) + outcome as u32
    }

    fn with_outcome(&self, outcome: Outcome) -> Self {
        match self {
            Self::Rock => match outcome {
                Outcome::Loose => Self::Scissors,
                Outcome::Draw => Self::Rock,
                Outcome::Win => Self::Paper,
            },
            Self::Paper => match outcome {
                Outcome::Loose => Self::Rock,
                Outcome::Draw => Self::Paper,
                Outcome::Win => Self::Scissors,
            },
            Self::Scissors => match outcome {
                Outcome::Loose => Self::Paper,
                Outcome::Draw => Self::Scissors,
                Outcome::Win => Self::Rock,
            },
        }
    }
}

#[derive(Debug)]
struct InvalidInputError;

impl FromStr for Shape {
    type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(InvalidInputError),
        }
    }
}

impl FromStr for Outcome {
    type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(InvalidInputError),
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let score = input
        .lines()
        .map(|s| {
            let v = s
                .split(" ")
                .map(Shape::from_str)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            v[1].round_score(&v[0])
        })
        .sum::<u32>();

    println!("part1: {}", score);

    let score2 = input
        .lines()
        .map(|s| {
            let [s0, s1]: [&str; 2] = s.split(" ").collect::<Vec<_>>().try_into().unwrap();

            let s0 = Shape::from_str(s0).unwrap();
            let s1 = s0.with_outcome(Outcome::from_str(s1).unwrap());

            s1.round_score(&s0)
        })
        .sum::<u32>();

    println!("part2: {}", score2);
}
