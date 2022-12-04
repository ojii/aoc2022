use itertools::Itertools;
use std::cmp::Reverse;

fn parse_food(data: &str) -> impl Iterator<Item = u32> + '_ {
    data.split("\n\n").map(|elf| {
        elf.split('\n')
            .flat_map(|food| food.parse::<u32>().ok())
            .sum()
    })
}

pub fn main() {
    let food: Vec<u32> = parse_food(include_str!("data/day1")).collect();
    println!("{}", food.iter().max().unwrap());
    println!(
        "{}",
        food.iter()
            .sorted_by_key(|i| Reverse(*i))
            .take(3)
            .sum::<u32>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_max() {
        assert_eq!(parse_food(EXAMPLE).max().unwrap(), 24000);
    }
}
