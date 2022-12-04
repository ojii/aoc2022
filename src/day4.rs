use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

struct Pair {
    first: HashSet<u8>,
    second: HashSet<u8>,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').ok_or(())?;
        Ok(Pair {
            first: parse_range(first)?,
            second: parse_range(second)?,
        })
    }
}

impl Pair {
    fn contained(&self) -> bool {
        self.first.is_subset(&self.second) || self.second.is_subset(&self.first)
    }

    fn overlapping(&self) -> bool {
        self.first.intersection(&self.second).next().is_some()
    }
}

fn parse_range(s: &str) -> Result<HashSet<u8>, ()> {
    let (low, high) = s.split_once('-').ok_or(())?;
    Ok(HashSet::from_iter(
        (low.parse().map_err(|_| ())?)..=(high.parse().map_err(|_| ())?),
    ))
}

fn count_containing_sections(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|pair| pair.contained()).count()
}

fn count_overlapping_sections(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|pair| pair.overlapping()).count()
}

fn parse_pairs(input: &str) -> Vec<Pair> {
    input
        .lines()
        .flat_map(|line| Pair::from_str(line).ok())
        .collect_vec()
}

pub fn main() {
    let input = include_str!("data/day4");
    let pairs = parse_pairs(input);
    println!("{}", count_containing_sections(&pairs));
    println!("{}", count_overlapping_sections(&pairs));
    // 621 ng
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_containing() {
        assert_eq!(2, count_containing_sections(&parse_pairs(EXAMPLE)));
    }

    #[test]
    fn test_overlapping() {
        assert_eq!(4, count_overlapping_sections(&parse_pairs(EXAMPLE)));
    }
}
