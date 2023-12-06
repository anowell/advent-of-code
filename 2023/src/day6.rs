//! [Advent of Code Day 6](https://adventofcode.com/2023/day/6)
//!
//! Day 6 can be solved by hand with regular calculator.
//! Each race can be represented as a quadratic equation.
//! To find how long you have to hold the button to achieve the 9mm record in the 7ms race,
//! solve `x^2 -7x +9 = 0` which has two solutions: `x =~ 1.7` and `x =~ 5.3``.
//! Every number between them will break the record.
//! And it works for part 2 if you use a calculator that support 64-bit math.

use crate::{math, parse};
use anyhow::{Result, Context};
use itertools::Itertools;
use std::ops::{Deref, RangeInclusive};

/// Calculate the product of the number of ways to win each race
pub fn part1(input: &str) -> Result<u64> {
    let races = Races::part1_from_str(input)?;
    let res = races
        .iter()
        .map(Race::record_breaking_range)
        .map(|range| range.count() as u64)
        .product();

    Ok(res)
}

/// Calculate number of ways to win a massive race
pub fn part2(input: &str) -> Result<u64> {
    let race = Race::part2_from_str(input)?;
    let range = race.record_breaking_range();
    let ways_to_win = range.count() as u64;
    Ok(ways_to_win)
}

#[derive(Debug, Clone)]
pub struct Races(Vec<Race>);

impl Deref for Races {
    type Target = Vec<Race>;

    // Required method
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    /// Time limit for the race
    pub time: u64,
    /// This is the record distance
    pub distance: u64,
}
impl Race {
    pub fn new(time: u64, distance: u64) -> Race {
        Race { time, distance }
    }

    pub fn part2_from_str(s: &str) -> Result<Self> {
        let lines: Vec<u64> = s
            .trim()
            .lines()
            .map(|line| {
                let nums = parse::extract_digits(line);
                nums.concat().parse::<u64>()
            })
            .try_collect()?;

        let time = lines[0];
        let distance = lines[1];
        let race = Race { time, distance };

        Ok(race)
    }

    /// Finds the range of hold times that would beat the current record
    ///
    /// ```no_run
    /// velocity = holdtime
    /// time = holdtime + traveltime
    /// distance = velocity * traveltime
    /// distance = holdtime * (time - holdtime)
    /// 0 = holdtime^2 - time*holdtime + distance
    /// ```
    ///
    /// Solves using quadratic equation with: `a = 1; b = -time; c = distance`
    pub fn record_breaking_range(&self) -> RangeInclusive<u64> {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let (root1, root2) = math::quadratic(1.0, -time, distance);

        // We can't just tie the record, we have to beat it.
        // So we add/sub an incredibly small value (1e-10) so that we can
        // be sure to get the smallest integers above and below the solutions.
        // This is needed for the cases where the quadratic solutions are whole numbers
        ((root1 + 1e-10).ceil() as u64)..=((root2 - 1e-10).floor() as u64)
    }
}

impl Races {
    pub fn part1_from_str(s: &str) -> Result<Self> {
        let mut lines = s.trim().lines().map(parse::extract_nums);
        let times = lines.next().context("Expected list of times")??;
        let distances = lines.next().context("Expected list of distances")??;
        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect_vec();

        Ok(Races(races))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_parse_part1() {
        let races = Races::part1_from_str(SAMPLE).unwrap();
        assert_eq!(races[0], Race::new(7, 9));
        assert_eq!(races[1], Race::new(15, 40));
        assert_eq!(races[2], Race::new(30, 200));
    }

    #[test]
    fn test_parse_part2() {
        let race = Race::part2_from_str(SAMPLE).unwrap();
        assert_eq!(race, Race::new(71530, 940200));
    }

    #[test]
    fn test_record_breaking_range() {
        let races = Races::part1_from_str(SAMPLE).unwrap();
        assert_eq!(races[0].record_breaking_range(), 2..=5);
        assert_eq!(races[1].record_breaking_range(), 4..=11);
        assert_eq!(races[2].record_breaking_range(), 11..=19);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 71503);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(6);
}
