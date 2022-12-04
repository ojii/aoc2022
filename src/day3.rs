use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

struct Rucksack {
    first: Vec<u8>,
    second: Vec<u8>,
}

impl Rucksack {
    fn find_duplicate(&self) -> Option<u8> {
        let first: HashSet<u8> = HashSet::from_iter(self.first.iter().copied());
        let second: HashSet<u8> = HashSet::from_iter(self.second.iter().copied());
        first.intersection(&second).next().copied()
    }

    fn all_items(&self) -> HashSet<u8> {
        return HashSet::from_iter(
            self.first
                .iter()
                .copied()
                .chain(self.second.iter().copied()),
        );
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.chars().map(priority).collect_vec();
        let (first, second) = v.split_at(v.len() / 2);
        Ok(Self {
            first: first.to_vec(),
            second: second.to_vec(),
        })
    }
}

fn priority(c: char) -> u8 {
    if c.is_lowercase() {
        (c as u32 as u8) - 96
    } else {
        (c as u32 as u8) - 38
    }
}

fn get_rucksacks(input: &str) -> impl Iterator<Item = Rucksack> + '_ {
    input.lines().flat_map(|line| Rucksack::from_str(line).ok())
}

fn duplicates_priority(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .flat_map(|r| r.find_duplicate().map(|d| d as u32))
        .sum()
}

fn find_badge<'a, G: Iterator<Item = &'a Rucksack>>(group: G) -> u32 {
    *group
        .map(|rucksack| rucksack.all_items())
        .reduce(|acc, item| HashSet::from_iter(acc.intersection(&item).copied()))
        .unwrap()
        .iter()
        .next()
        .unwrap() as u32
}

fn groups_priority(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().chunks(3).into_iter().map(find_badge).sum()
}

pub fn main() {
    let input = include_str!("data/day3");
    let rucksacks = get_rucksacks(input).collect_vec();
    println!("{}", duplicates_priority(&rucksacks));
    println!("{}", groups_priority(&rucksacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_duplicates_priority() {
        assert_eq!(
            157,
            duplicates_priority(&get_rucksacks(EXAMPLE).collect_vec())
        );
    }
    #[test]
    fn test_groups_priority() {
        assert_eq!(70, groups_priority(&get_rucksacks(EXAMPLE).collect_vec()));
    }
}
