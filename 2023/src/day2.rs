//! [Advent of Code Day 2](https://adventofcode.com/2023/day/2)

use std::{
    cmp::{self, Ordering},
    str::FromStr,
};
use itertools::Itertools;
use anyhow::{anyhow as err, bail, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;


static RE_GAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d*): (.*)").unwrap());

/// Calculates the sum of game IDs that can be played with 12 red, 13 blue, & 14 green cubes
pub fn part1(input: &str) -> Result<u32> {
    let complete = CubeSet::new(12, 13, 14);
    let games = crate::parse::parse_lines::<Game>(input)?;
    let sum = games
        .into_iter()
        .filter(|g| g.is_possible_with(&complete))
        .map(|g| g.id)
        .sum();
    Ok(sum)
}

/// Calculates the power of the minimum set of cubes needed for a cube-drawing game
pub fn part2(input: &str) -> Result<u32> {
    let games = crate::parse::parse_lines::<Game>(input)?;
    let sum = games
        .iter()
        .map(Game::min_superset)
        .map(|cs| cs.power())
        .sum();
    Ok(sum)
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<CubeSet>,
}

impl Game {
    pub fn is_possible_with(&self, complete: &CubeSet) -> bool {
        self.draws.iter().all(|d| d < complete)
    }

    // Min superset is the smallest cubeset where is_possible_with returns true
    pub fn min_superset(&self) -> CubeSet {
        let mut min_set = self.draws[0];
        for draw in self.draws.iter().skip(1) {
            min_set.red = cmp::max(draw.red, min_set.red);
            min_set.green = cmp::max(draw.green, min_set.green);
            min_set.blue = cmp::max(draw.blue, min_set.blue);
        }
        min_set
    }
}

// Parse a full line of input into a Game: "Game 1: 3 red, 4 blue; etc..."
impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = RE_GAME
            .captures(s)
            .ok_or_else(|| err!("Invalid game: {s}"))?;
        if caps.len() < 3 {
            bail!("Invalid game: {s}");
        }
        let id: u32 = caps[1].parse()?;
        let draws = caps[2]
            .split(";")
            .map(str::trim)
            .map(CubeSet::from_str)
            .try_collect()?;

        Ok(Game { id, draws })
    }
}

// Parse a string like: "3 red, 2 blue" into a CubeSet
impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let parts = s.split(',').map(str::trim);
        for part in parts {
            let frags: Vec<&str> = part.splitn(2, ' ').collect();
            if frags.len() < 2 {
                bail!("Expected count & color: {part}");
            }
            let count = frags[0].parse::<u32>()?;
            let color = frags[1];
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => bail!("Unknown color: {color}"),
            }
        }
        Ok(CubeSet { red, green, blue })
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    pub fn new(red: u32, green: u32, blue: u32) -> CubeSet {
        CubeSet { red, green, blue }
    }

    /// The power of a set of cubes as equal to the color counts multipled
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

// Custom Partial Ordering to use comparison orders
// Less implies strict subset; greater implies strict superset
// CubeSet::new(1,2,3) < CubeSet::new(3,3,3)
impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let r = self.red.cmp(&other.red);
        let g = self.green.cmp(&other.green);
        let b = self.blue.cmp(&other.blue);
        if r.is_eq() && g.is_eq() && b.is_eq() {
            Some(Ordering::Equal)
        } else if r.is_le() && g.is_le() && b.is_le() {
            Some(Ordering::Less)
        } else if r.is_ge() && g.is_ge() && b.is_ge() {
            Some(Ordering::Greater)
        } else {
            // Not comparable
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_cube_cmp() {
        assert_eq!(CubeSet::new(1, 2, 3), CubeSet::new(1, 2, 3));
        assert_eq!(CubeSet::new(1, 2, 3) < CubeSet::new(1, 2, 3), false);
        assert_eq!(CubeSet::new(1, 2, 3) < CubeSet::new(2, 3, 4), true);
        assert_eq!(CubeSet::new(1, 2, 3) < CubeSet::new(1, 2, 4), true);
        assert_eq!(CubeSet::new(1, 2, 3) < CubeSet::new(4, 4, 2), false);
        assert_eq!(CubeSet::new(1, 2, 3) > CubeSet::new(1, 2, 3), false);
        assert_eq!(CubeSet::new(1, 2, 3) > CubeSet::new(2, 3, 4), false);
        assert_eq!(CubeSet::new(1, 2, 3) > CubeSet::new(1, 2, 4), false);
        assert_eq!(CubeSet::new(1, 2, 3) > CubeSet::new(4, 4, 2), false);
        assert_eq!(CubeSet::new(5, 5, 5) > CubeSet::new(4, 5, 5), true);
    }

    #[test]
    fn test_game_parse() {
        let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
            .expect("Game parsing error");

        assert_eq!(game.id, 1);
        assert_eq!(game.draws.len(), 3);
        assert_eq!(game.draws[0].red, 4);
        assert_eq!(game.draws[0].green, 0);
        assert_eq!(game.draws[0].blue, 3);
        assert_eq!(game.draws[2].red, 0);
        assert_eq!(game.draws[2].green, 2);
        assert_eq!(game.draws[2].blue, 0);
    }

    fn game(s: &str) -> Game {
        Game::from_str(&format!("Game 100: {s}")).unwrap()
    }
    fn sample_games() -> Vec<Game> {
        vec![
            game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            game("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            game("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ]
    }

    #[test]
    fn test_game_possible() {
        let complete = CubeSet::new(12, 13, 14);
        let g = sample_games();
        assert_eq!(true, g[0].is_possible_with(&complete));
        assert_eq!(true, g[1].is_possible_with(&complete));
        assert_eq!(false, g[2].is_possible_with(&complete));
        assert_eq!(false, g[3].is_possible_with(&complete));
        assert_eq!(true, g[4].is_possible_with(&complete));
    }

    #[test]
    fn test_min_set() {
        let g = sample_games();
        assert_eq!(g[0].min_superset(), CubeSet::new(4, 2, 6));
        assert_eq!(g[1].min_superset(), CubeSet::new(1, 3, 4));
        assert_eq!(g[2].min_superset(), CubeSet::new(20, 13, 6));
        assert_eq!(g[3].min_superset(), CubeSet::new(14, 3, 15));
        assert_eq!(g[4].min_superset(), CubeSet::new(6, 3, 2));
    }

    const SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 8);
    }

    const SAMPLE2: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2).unwrap(), 2286);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(2);
}
