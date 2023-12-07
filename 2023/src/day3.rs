//! [Advent of Code Day 3](https://adventofcode.com/2023/day/3)

use std::cmp;
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::{Match, Regex};
use itertools::Itertools;

/// Calculates the sum of part numbers in the schematic
pub fn part1(input: &str) -> Result<u32> {
    let schematic = Schematic::new(input);
    Ok(schematic.part_numbers().iter().sum())
}

/// Calculates the sum of gear ratios in the schematic
pub fn part2(input: &str) -> Result<u32> {
    let schematic = Schematic::new(input);
    Ok(schematic.gears().iter().map(Gear::ratio).sum())
}

#[derive(Debug, Clone, PartialEq)]
// Schematic that provides (x,y) matrix representation of the input
// (0,0) is visually the top-left
pub struct Schematic {
    buf: String,
    cols: usize,
    rows: usize,
}

static RE_NUM: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static RE_SYM: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\w\.\s]+").unwrap());

impl Schematic {
    pub fn new(s: &str) -> Schematic {
        let cols = s.lines().next().unwrap().len();
        let rows = s.lines().count();
        Schematic {
            rows,
            cols,
            buf: s.to_string(),
        }
    }

    pub fn part_numbers(&self) -> Vec<u32> {
        let part_filter = |m: &Match| {
            let (x, y) = (m.start() % (self.cols + 1), m.start() / (self.cols + 1));
            self.has_adjacent_symbol(x, y, m.len())
        };
        let nums: Vec<u32> = RE_NUM
            .find_iter(&self.buf)
            .filter(part_filter)
            .flat_map(|m| m.as_str().parse::<u32>())
            .collect();
        nums
    }

    pub fn gears(&self) -> Vec<Gear> {
        let nums = RE_NUM
            .find_iter(&self.buf)
            .map(|m| Num {
                x: m.start() % (self.cols + 1),
                y: m.start() / (self.cols + 1),
                len: m.as_str().len(),
                val: m.as_str().parse().unwrap(),
            })
            .collect_vec();

        let adjacent_parts = |sym_match: Match| {
            nums.iter()
                .filter(|p| {
                    p.adjacent_to(
                        sym_match.start() % (self.cols + 1),
                        sym_match.start() / (self.cols + 1),
                    )
                })
                .collect_vec()
        };

        RE_SYM
            .find_iter(&self.buf)
            // .inspect(|item| eprintln!("SYMBOL {}", item.as_str()))
            .map(adjacent_parts)
            // .inspect(|item| eprintln!("NUMS {:?}",item))
            .filter(|p| p.len() == 2)
            .map(|p| Gear(p[0].val, p[1].val))
            .collect()
    }

    fn has_symbol_at(&self, x: usize, y: usize) -> bool {
        assert!(x < self.cols);
        assert!(y < self.rows);

        // buf has '\n' on each row
        let width = self.cols + 1;

        let offset = y * width + x;
        let byte = self.buf.as_bytes()[offset];
        let c = char::from_u32(byte as u32).unwrap();
        c != '.' && c.is_ascii_punctuation()
    }

    fn has_adjacent_symbol(&self, x: usize, y: usize, len: usize) -> bool {
        assert!(x + len < self.cols + 1);
        assert!(y < self.rows);
        let x1 = x.saturating_sub(1);
        let y1 = y.saturating_sub(1);
        let x2 = cmp::min(x + len, self.cols - 1);
        let y2 = cmp::min(y + 1, self.rows - 1);

        for i in y1..=y2 {
            for j in x1..=x2 {
                if self.has_symbol_at(j, i) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Gear(u32, u32);

impl Gear {
    pub fn ratio(&self) -> u32 {
        self.0 * self.1
    }
}

impl PartialEq for Gear {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 ||
            self.0 == other.1 && self.1 == other.0
    }
}

#[derive(Debug, Copy, Clone)]
struct Num {
    val: u32,
    x: usize,
    y: usize,
    len: usize,
}

impl Num {
    fn adjacent_to(&self, x: usize, y: usize) -> bool {
        let x1 = self.x.saturating_sub(1);
        let y1 = self.y.saturating_sub(1);
        let x2 = self.x.saturating_add(self.len);
        let y2 = self.y.saturating_add(1);

        x >= x1 && x <= x2 && y >= y1 && y <= y2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_symbol_lookup() {
        let schematic = Schematic::new(SAMPLE);
        assert!(!schematic.has_symbol_at(0, 0));
        assert!(schematic.has_symbol_at(3, 1));
        assert!(schematic.has_symbol_at(6, 3));
        assert!(!schematic.has_symbol_at(9, 0));
    }

    #[test]
    fn test_adjacent_symbol() {
        let schematic = Schematic::new(SAMPLE);
        assert!(!schematic.has_adjacent_symbol(0, 0, 1));
        assert!(!schematic.has_adjacent_symbol(1, 0, 1));
        assert!(schematic.has_adjacent_symbol(2, 0, 1));
        assert!(schematic.has_adjacent_symbol(3, 0, 1));
        assert!(schematic.has_adjacent_symbol(4, 0, 1));
        assert!(!schematic.has_adjacent_symbol(5, 0, 1));
        assert!(schematic.has_adjacent_symbol(1, 0, 2));
        assert!(schematic.has_adjacent_symbol(0, 0, 3));
        assert!(!schematic.has_adjacent_symbol(0, 2, 1));
        assert!(schematic.has_adjacent_symbol(0, 1, 3));
        assert!(schematic.has_adjacent_symbol(5, 9, 1));
        assert!(schematic.has_adjacent_symbol(6, 9, 3));
        assert!(!schematic.has_adjacent_symbol(7, 9, 3));
        assert!(!schematic.has_adjacent_symbol(8, 9, 2));
        assert!(!schematic.has_adjacent_symbol(9, 9, 1));
    }

    #[test]
    fn test_part_numbers() {
        let schematic = Schematic::new(SAMPLE);
        let parts = schematic.part_numbers();
        assert_eq!(parts, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn test_gears() {
        let schematic = Schematic::new(SAMPLE);
        let mut gears = schematic.gears();
        assert_eq!(gears.len(), 2);

        gears.sort_by_key(|a| a.ratio());
        assert_eq!(gears, vec![Gear(467, 35), Gear(755, 598)]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 467835);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(3);
}
