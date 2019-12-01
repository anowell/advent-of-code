use fxhash::FxHashSet;
use matrix::format::conventional::Conventional;
use regex::Regex;
use crate::Error;

// 2018 AoC Day 3 puzzle
// https://adventofcode.com/2018/day/3

const WIDTH: usize = 1_000;
const HEIGHT: usize = 1_000;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Unused,
    Used,
    Overlapping,
}

impl ::matrix::Element for State {
    fn is_zero(&self) -> bool {
        *self == State::Unused
    }
    fn zero() -> Self {
        State::Unused
    }
}

pub fn part1(input: &str) -> Result<u32, Error> {
    let mut square = Conventional::new((WIDTH, HEIGHT));
    let mut overlapping = 0;

    for line in input.trim().lines() {
        let Claim { x, y, w, h, .. } = parse_claim(&line)?;
        for i in x..(x + w) {
            for j in y..(y + h) {
                square[(i, j)] = match square[(i, j)] {
                    State::Unused => State::Used,
                    State::Used => {
                        overlapping += 1;
                        State::Overlapping
                    }
                    State::Overlapping => State::Overlapping,
                };
            }
        }
    }

    Ok(overlapping)
}

pub fn part2(input: &str) -> Result<u32, Error> {
    let mut square = Conventional::new((WIDTH, HEIGHT));

    let mut claim_ids = FxHashSet::default();
    for line in input.trim().lines() {
        let Claim { id, x, y, w, h } = parse_claim(&line)?;
        let mut overlapped = false;
        for i in x..(x + w) {
            for j in y..(y + h) {
                // This zero check assumes no claim id will ever be 0.
                if square[(i, j)] == 0 {
                    square[(i, j)] = id;
                } else {
                    overlapped = true;
                    claim_ids.remove(&square[(i, j)]);
                }
            }
        }
        // Faster to insert once, than insert and remove many times
        if !overlapped {
            claim_ids.insert(id);
        }
    }

    match claim_ids.iter().next() {
        Some(id) => Ok(*id),
        None => Err("Failed to find any claims that did not overlap".into()),
    }
}

lazy_static! {
    static ref CLAIM_RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}

fn parse_claim(input: &str) -> Result<Claim, Error> {
    let caps = CLAIM_RE
        .captures(input)
        .ok_or_else(|| format!("Claim could not be parsed: {}", input))?;

    Ok(Claim {
        id: caps[1].parse()?,
        x: caps[2].parse()?,
        y: caps[3].parse()?,
        w: caps[4].parse()?,
        h: caps[5].parse()?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        let input = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;
        assert_eq!(part1(input).unwrap(), 4);
    }

    #[test]
    fn test_part2() {
        let input = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;
        assert_eq!(part2(input).unwrap(), 3);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-3.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-3.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
