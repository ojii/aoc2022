use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

type Stacks = HashMap<usize, VecDeque<char>>;

fn to_usize(s: &str) -> Result<usize, ()> {
    s.parse().map_err(|_| ())
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, direction) = &s[5..].split_once(" from ").ok_or(())?;
        let (from, to) = direction.split_once(" to ").ok_or(())?;
        Ok(Move {
            amount: to_usize(amount)?,
            from: to_usize(from)?,
            to: to_usize(to)?,
        })
    }
}

fn parse_initial(s: &str) -> Stacks {
    let mut initial: Stacks = HashMap::new();
    for line in s.lines() {
        for (index, chunk) in line.chars().chunks(4).into_iter().enumerate() {
            match chunk.into_iter().nth(1) {
                Some(c) if c.is_ascii_alphabetic() => {
                    initial.entry(index + 1).or_default().push_back(c);
                }
                _ => (),
            }
        }
    }
    initial
}

fn crate_mover_9000(mut stacks: Stacks, instruction: &Move) -> Stacks {
    let mut crane = VecDeque::with_capacity(instruction.amount);
    for _ in 0..instruction.amount {
        crane.push_back(
            stacks
                .get_mut(&instruction.from)
                .unwrap()
                .pop_front()
                .unwrap(),
        )
    }
    for c in crane {
        stacks.get_mut(&instruction.to).unwrap().push_front(c)
    }
    stacks
}

fn crate_mover_9001(mut stacks: Stacks, instruction: &Move) -> Stacks {
    let mut crane = VecDeque::with_capacity(instruction.amount);
    for _ in 0..instruction.amount {
        crane.push_front(
            stacks
                .get_mut(&instruction.from)
                .unwrap()
                .pop_front()
                .unwrap(),
        )
    }
    for c in crane {
        stacks.get_mut(&instruction.to).unwrap().push_front(c)
    }
    stacks
}

struct Plan {
    initial: Stacks,
    moves: Vec<Move>,
}

impl Plan {
    fn execute<Crane>(&self, crane: Crane) -> Stacks
    where
        Crane: FnMut(Stacks, &Move) -> Stacks,
    {
        self.moves.iter().fold(self.initial.clone(), crane)
    }
}

impl FromStr for Plan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (initial, moves) = s.split_once("\n\n").ok_or(())?;
        Ok(Plan {
            initial: parse_initial(initial),
            moves: moves
                .lines()
                .flat_map(|line| Move::from_str(line).ok())
                .collect(),
        })
    }
}

fn tops(stacks: &Stacks) -> String {
    (1..=(stacks.len()))
        .map(|index| stacks.get(&index).unwrap().front().unwrap())
        .join("")
}

pub fn main() {
    let plan = Plan::from_str(include_str!("data/day5")).unwrap();
    let stacks = plan.execute(crate_mover_9000);
    println!("{}", tops(&stacks));
    let stacks = plan.execute(crate_mover_9001);
    println!("{}", tops(&stacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_9000() {
        let plan = Plan::from_str(EXAMPLE).unwrap();
        let stacks = plan.execute(crate_mover_9000);
        assert_eq!(tops(&stacks), "CMZ");
    }

    #[test]
    fn test_9001() {
        let plan = Plan::from_str(EXAMPLE).unwrap();
        let stacks = plan.execute(crate_mover_9001);
        assert_eq!(tops(&stacks), "MCD");
    }
}
