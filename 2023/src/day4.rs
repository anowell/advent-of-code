//! [Advent of Code Day 4](https://adventofcode.com/2023/day/4)

use anyhow::{bail, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::BTreeSet, str::FromStr};
use itertools::Itertools;

/// Calculates the total points for a set of scratchcards
pub fn part1(input: &str) -> Result<u32> {
    let cards = crate::parse::parse_lines::<Card>(input)?;
    Ok(cards.iter().map(Card::points).sum())
}

/// Count total scratchcards where scratchcards win more scratchards
pub fn part2(input: &str) -> Result<u32> {
    let counts = card_counts(input)?;
    Ok(counts.iter().sum())
}

fn card_counts(input: &str) -> Result<Vec<u32>> {
    let cards = crate::parse::parse_lines::<Card>(input)?;
    let mut card_counts = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let original_points = card.matches();
        for j in (i + 1)..=(i + original_points) {
            card_counts[j] += card_counts[i];
        }
    }
    Ok(card_counts)
}

#[derive(Debug, Clone)]
pub struct Card {
    winners: BTreeSet<u32>,
    numbers: BTreeSet<u32>,
}

impl Card {
    /// Calculates points for a card. 1 point for first match, doubles for each match after
    pub fn points(&self) -> u32 {
        match self.matches() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }

    /// Counts number of winning numbers that match your numbers on a scratchcard
    pub fn matches(&self) -> usize {
        self.winners.intersection(&self.numbers).count()
    }
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split([':', '|']).skip(1).collect();
        if parts.len() < 2 {
            bail!("Invalid card: {s}");
        }
        let winners = RE
            .find_iter(parts[0])
            .map(|m| m.as_str().parse::<u32>())
            .try_collect()?;

        let numbers = RE
            .find_iter(parts[1])
            .map(|m| m.as_str().parse::<u32>())
            .try_collect()?;

        Ok(Card { winners, numbers })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    fn sample_card(n: usize) -> Card {
        let line = SAMPLE.lines().skip(n).next().unwrap();
        Card::from_str(&line).unwrap()
    }

    #[test]
    fn test_parse() {
        let card = sample_card(0);
        assert_eq!(card.winners, BTreeSet::from([41, 48, 83, 86, 17]));
        assert_eq!(card.numbers, BTreeSet::from([83, 86, 6, 31, 17, 9, 48, 53]));
    }

    #[test]
    fn test_points() {
        assert_eq!(sample_card(0).points(), 8);
        assert_eq!(sample_card(1).points(), 2);
        assert_eq!(sample_card(2).points(), 2);
        assert_eq!(sample_card(3).points(), 1);
        assert_eq!(sample_card(4).points(), 0);
        assert_eq!(sample_card(5).points(), 0);
    }

    #[test]
    fn test_card_counts() {
        assert_eq!(card_counts(SAMPLE).unwrap(), vec![1, 2, 4, 8, 14, 1])
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 30);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[divan::bench]
    fn bench_part1(bencher: divan::Bencher) {
        let input = crate::input("day4").unwrap();
        bencher.bench(|| part1(&input).unwrap());
    }

    #[divan::bench]
    fn bench_part2(bencher: divan::Bencher) {
        let input = crate::input("day4").unwrap();
        bencher.bench(|| part2(&input).unwrap());
    }
}
