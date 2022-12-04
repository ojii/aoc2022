use std::str::FromStr;

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn play(&self, against: &RPS) -> Outcome {
        match (self, against) {
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                Outcome::Draw
            }
            (RPS::Rock, RPS::Scissors) | (RPS::Scissors, RPS::Paper) | (RPS::Paper, RPS::Rock) => {
                Outcome::Win
            }
            _ => Outcome::Loss,
        }
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

struct Match {
    you: RPS,
    opponent: RPS,
}

impl Match {
    fn play(&self) -> Outcome {
        self.you.play(&self.opponent)
    }

    fn score(&self) -> u32 {
        self.play().score() + self.you.score()
    }
}

impl FromStr for Match {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opp, me) = s.split_once(' ').ok_or(())?;
        Ok(Match {
            you: RPS::from_str(me)?,
            opponent: RPS::from_str(opp)?,
        })
    }
}

struct Strategy {
    opponent: RPS,
    outcome: Outcome,
}

impl Strategy {
    fn solve(&self) -> RPS {
        match self.outcome {
            Outcome::Win => match self.opponent {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            Outcome::Draw => self.opponent.clone(),
            Outcome::Loss => match self.opponent {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
        }
    }

    fn score(&self) -> u32 {
        self.outcome.score() + self.solve().score()
    }
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opp, out) = s.split_once(' ').ok_or(())?;
        Ok(Strategy {
            outcome: Outcome::from_str(out)?,
            opponent: RPS::from_str(opp)?,
        })
    }
}

fn score(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| Match::from_str(line).ok())
        .map(|m| m.score())
        .sum::<u32>()
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| Strategy::from_str(line).ok())
        .map(|m| m.score())
        .sum::<u32>()
}

pub fn main() {
    let input = include_str!("data/day1");
    println!("{}", score(input));
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_score() {
        assert_eq!(15, score(EXAMPLE));
    }

    #[test]
    fn test_solution() {
        assert_eq!(12, solve(EXAMPLE));
    }
}
