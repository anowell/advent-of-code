//! [Advent of Code Day 11](https://adventofcode.com/2024/day/11)

use std::collections::HashMap;

use crate::parse;
use crate::prelude::*;

pub fn part1(input: &str) -> Result<usize> {
    let mut stones: Vec<u64> = parse::extract_nums(input)?;
    for _ in 0..25 {
        stones = process_stones(&stones);
    }
    Ok(stones.len())
}

pub fn part2(input: &str) -> Result<u64> {
    let stones: Vec<u64> = parse::extract_nums(input)?;

    let count = part2_helper(&stones, 75);
    Ok(count)
}

fn part2_helper(stones: &[u64], blinks: u32) -> u64 {
    let mut stone_map = StoneLine::default();
    let mut total = 0;
    for stone in stones {
        total += stone_map.count(*stone, blinks);
    }
    total
}

fn process_stones(stones: &[u64]) -> Vec<u64> {
    let mut next = Vec::with_capacity(stones.len() * 2);
    for stone in stones {
        if *stone == 0 {
            next.push(1);
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let divisor = 10_u64.pow(digits / 2);
                next.push(stone / divisor);
                next.push(stone % divisor);
            } else {
                next.push(stone * 2024);
            }
        }
    }
    next
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Idx {
    value: u64,
    blinks: u32,
}

#[derive(Default)]
struct StoneLine {
    map: HashMap<Idx, u64>,
}

impl StoneLine {
    // Given a specific rock, count how many rocks it will be after a number of blinks
    // Uses recursion and caches intermediate counts to avoid recalculating values at previously encountered depths
    fn count(&mut self, value: u64, blinks: u32) -> u64 {
        let idx = Idx { value, blinks };
        match self.map.get(&idx) {
            // Use the cached value
            Some(count) => *count,
            // Stop condition for recursion
            None if blinks == 0 => 1,
            // Recurse with 1 fewer blinks
            None => {
                let mut val = 0;
                for stone in process_stones(&[value]) {
                    val += self.count(stone, blinks - 1);
                }
                self.map.insert(idx, val);
                val
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_stones() {
        let stones = vec![0, 1, 10, 99, 999];
        assert_eq!(process_stones(&stones), vec![1, 2024, 1, 0, 9, 9, 2021976]);

        let stones = vec![125, 17];
        let stones = process_stones(&stones);
        assert_eq!(stones, vec![253000, 1, 7]);
        let stones = process_stones(&stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
    }

    #[test]
    fn test_calc() {
        let mut stone_map = StoneLine::default();
        assert_eq!(stone_map.count(0, 0), 1); // 0
        assert_eq!(stone_map.count(0, 1), 1); // 0 -> 1
        assert_eq!(stone_map.count(0, 2), 1); // 0 -> 1 -> 2024
        assert_eq!(stone_map.count(0, 3), 2); // 0 -> 1 -> 2024 -> 20 24
        assert_eq!(stone_map.count(1, 0), 1); // 1
        assert_eq!(stone_map.count(1, 1), 1); // 1 -> 2024
        assert_eq!(stone_map.count(1, 2), 2); // 1 -> 2024 -> 20 24

        let mut stone_map = StoneLine::default();
        assert_eq!(stone_map.count(125, 0), 1); // 125
        assert_eq!(stone_map.count(125, 1), 1); // 253000
        assert_eq!(stone_map.count(125, 2), 2); // 253 0
        assert_eq!(stone_map.count(125, 3), 2); // 512072 1
        assert_eq!(stone_map.count(125, 4), 3); // 512 72 2024
        assert_eq!(stone_map.count(125, 5), 5); // 1036288 7 2 20 24
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("125 17").unwrap(), 55312);
    }

    #[test]
    fn test_part2() {
        // No correct answer for the part2 was provided
        // This uses the part2 logic to test the expected value from part 1 (25 blinks instead of 75)
        let stones = [125, 17];
        assert_eq!(part2_helper(&stones, 25), 55312);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(11);
}
