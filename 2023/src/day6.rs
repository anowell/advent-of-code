use crate::parse;
use anyhow::Result;
use itertools::Itertools;
use std::ops::{Deref, RangeInclusive};

pub fn part1(input: &str) -> Result<u64> {
    let races = Races::part1_from_str(input)?;
    let res = races
        .iter()
        .map(Race::record_breaking_range)
        .map(|range| range.end() + 1 - range.start())
        .fold(1, |acc, res| res * acc);

    Ok(res)
}

pub fn part2(input: &str) -> Result<u64> {
    let race = Race::part2_from_str(input)?;
    let range = race.record_breaking_range();
    let ways_to_win = range.end() + 1 - range.start();
    Ok(ways_to_win)
}

#[derive(Debug, Clone)]
struct Races(Vec<Race>);

impl Deref for Races {
    type Target = Vec<Race>;

    // Required method
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Race {
    // Time limit for the race
    time: u64,
    // This is the record distance
    distance: u64,
}

impl Race {
    // record = velocity * (racetime - holdtime)
    // velocity = holdtime
    // record = holdtime * (racetime - holdtime)
    // 0 = holdtime^2 - racetime + record
    // Use quadratic equation with: a = 1; b = -racetime; c = record
    fn record_breaking_range(&self) -> RangeInclusive<u64> {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let (solve1, solve2) = quadratic(1.0, -time, distance);

        // We can't just tie the record, we have to beat it.
        // So we add/sub an incredibly small value (1e-10) so that we can
        // be sure to get the smallest integers above and below the solutions.
        // This is needed for the cases where the quadratic solutions are whole numbers
        ((solve1 + 1e-10).ceil() as u64)..=((solve2 - 1e-10).floor() as u64)
    }
}

// solutions = (-b +- sqrt(b^2 - 4ac)) / 2a
fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let solve1 = (-b - f64::sqrt(b.powi(2) - 4.0 * a * c)) / (2.0 * a);
    let solve2 = (-b + f64::sqrt(b.powi(2) - 4.0 * a * c)) / (2.0 * a);

    (solve1, solve2)
}

impl Races {
    fn part1_from_str(s: &str) -> Result<Self> {
        let mut lines = s.trim().lines().map(parse::extract_nums);

        let times = lines.next().unwrap()?;
        let distances = lines.next().unwrap()?;
        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect_vec();

        Ok(Races(races))
    }
}

impl Race {
    #[cfg(test)]
    fn new(time: u64, distance: u64) -> Race {
        Race { time, distance }
    }

    fn part2_from_str(s: &str) -> Result<Self> {
        let lines: Vec<u64> = s
            .trim()
            .lines()
            .map(|line| {
                let nums = parse::RE_NUMS
                    .find_iter(line)
                    .map(|m| m.as_str())
                    .collect_vec();
                nums.concat().parse::<u64>()
            })
            .try_collect()?;

        let time = lines[0];
        let distance = lines[1];
        let race = Race { time, distance };

        Ok(race)
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
    use super::*;

    #[divan::bench]
    fn bench_part1(bencher: divan::Bencher) {
        let input = crate::input("day6").unwrap();
        bencher.bench(|| part1(&input).unwrap());
    }

    #[divan::bench]
    fn bench_part2(bencher: divan::Bencher) {
        let input = crate::input("day6").unwrap();
        bencher.bench(|| part2(&input).unwrap());
    }
}
