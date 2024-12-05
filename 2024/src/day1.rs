//! [Advent of Code Day 1](https://adventofcode.com/2024/day/1)

use crate::parse;
use crate::prelude::*;

/// Calculates the sum of distances between number pairs
pub fn part1(input: &str) -> Result<u32> {
    let (mut a, mut b) = get_lists(input)?;
    a.sort();
    b.sort();
    let sum = a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum();
    Ok(sum)
}

/// Calculates the sum of similarity scores
pub fn part2(input: &str) -> Result<u32> {
    let (a, b) = get_lists(input)?;
    let bmap = b.into_iter().counts();
    let sum = a
        .iter()
        .map(|n| *bmap.get(n).unwrap_or(&0) as u32 * n)
        .sum();
    Ok(sum)
}

fn get_lists(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let nums = parse::extract_nums::<u32>(input)?;
    let mut toggle = true;
    let lists = nums.into_iter().partition(|_| {
        let is_left = toggle;
        toggle = !is_left;
        is_left
    });
    Ok(lists)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"\
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 31);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(1);
}
