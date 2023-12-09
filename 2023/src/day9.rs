//! [Advent of Code Day 9](https://adventofcode.com/2023/day/9)

use crate::parse;
use crate::prelude::*;

/// Finds the next number in a sequence by deriving a difference tree
pub fn part1(input: &str) -> Result<i64> {
    let list = parse::parse_lines_with(input, parse::extract_nums)?;
    let sum = list.iter().map(|l| next_in_pattern(l)).sum();
    Ok(sum)
}

/// Finds the previous number in a sequence by deriving a difference tree
pub fn part2(input: &str) -> Result<i64> {
    let list = parse::parse_lines_with(input, parse::extract_nums)?;
    let sum = list.iter().map(|l| prev_in_pattern(l)).sum();
    Ok(sum)
}

fn derivatives(list: &[i64]) -> Vec<i64> {
    list.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

/// Recursively calculate the next item in the derived pattern
pub fn next_in_pattern(list: &[i64]) -> i64 {
    let derive_list = derivatives(list);
    let child_next = match derive_list.iter().all(|n| *n == 0) {
        true => 0,
        false => next_in_pattern(&derive_list),
    };
    list.last().unwrap() + child_next
}

pub fn prev_in_pattern(list: &[i64]) -> i64 {
    let derive_list = derivatives(list);
    let child_prev = match derive_list.iter().all(|n| *n == 0) {
        true => 0,
        false => prev_in_pattern(&derive_list),
    };
    list.first().unwrap() - child_prev
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_next_in_pattern() {
        assert_eq!(next_in_pattern(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(next_in_pattern(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(next_in_pattern(&[10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(next_in_pattern(&[-1, -3, -5, -7]), -9);
        assert_eq!(next_in_pattern(&[9, 4, -1, -6]), -11);
    }

    #[test]
    fn test_prev_in_pattern() {
        assert_eq!(prev_in_pattern(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(prev_in_pattern(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(prev_in_pattern(&[10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(prev_in_pattern(&[-1, -3, -5, -7]), 1);
        assert_eq!(prev_in_pattern(&[9, 4, -1, -6]), 14);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 2);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(9);
}
