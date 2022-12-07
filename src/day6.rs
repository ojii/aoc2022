use itertools::Itertools;

fn unique(arr: &[usize]) -> bool {
    arr.iter().all_unique()
}

fn find_marker(stream: &[char], size: usize) -> Option<usize> {
    stream
        .windows(size)
        .find_position(|w| w.iter().all_unique())
        .map(|(index, _)| index + size)
}

fn find_start_marker(s: &[char]) -> Option<usize> {
    find_marker(s, 4)
}
fn find_start_of_message(s: &[char]) -> Option<usize> {
    find_marker(s, 14)
}

pub fn main() {
    let input = include_str!("data/day6").chars().collect_vec();
    println!("{}", find_start_marker(&input).unwrap());
    println!("{}", find_start_of_message(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_find_start_marker(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(
            find_start_marker(&input.chars().collect_vec()),
            Some(expected)
        )
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_find_start_of_message(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(
            find_start_of_message(&input.chars().collect_vec()),
            Some(expected)
        )
    }
}
