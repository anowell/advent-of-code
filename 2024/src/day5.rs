//! [Advent of Code Day 5](https://adventofcode.com/2024/day/5)

use crate::{parse, prelude::*};

pub fn part1(input: &str) -> Result<u32> {
    let input = Input::from_str(input).unwrap();
    let sum = input.well_ordered_middle_sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<u32> {
    let mut input = Input::from_str(input).unwrap();
    let sum = input.reordered_middle_sum();
    Ok(sum)
}

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl Input {
    fn well_ordered_middle_sum(&self) -> u32 {
        let mut sum = 0;
        for update in &self.updates {
            if well_ordered(&self.rules, update) {
                sum += update[update.len() / 2];
            }
        }
        sum
    }

    fn reordered_middle_sum(&mut self) -> u32 {
        let mut sum = 0;
        for update in &mut self.updates {
            if !well_ordered(&self.rules, update) {
                reorder(&self.rules, update);
                sum += update[update.len() / 2];
            }
        }
        sum
    }
}

fn well_ordered(rules: &[(u32, u32)], update: &[u32]) -> bool {
    for (page_a, page_b) in update.iter().tuple_combinations() {
        for rule in rules {
            if *page_b == rule.0 && *page_a == rule.1 {
                return false;
            }
        }
    }
    true
}

/// Tests every pair of elements according to the rules
/// Swaps any pair of elements that break any rule
fn reorder(rules: &[(u32, u32)], update: &mut [u32]) {
    for (left, right) in (0..update.len()).tuple_combinations() {
        for rule in rules {
            if update[right] == rule.0 && update[left] == rule.1 {
                update.swap(left, right)
            }
        }
    }
}

static RE_ORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\|(\d+)").unwrap());

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut orderings = Vec::new();
        let mut lines = s.lines();
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            let caps = RE_ORD
                .captures(line)
                .ok_or_else(|| Error::msg("Expected ordering rule"))?;
            orderings.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
        }

        let mut updates = Vec::new();
        for line in lines {
            updates.push(parse::extract_nums::<u32>(line)?);
        }
        Ok(Input {
            rules: orderings,
            updates,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn test_parse() {
        let input = Input::from_str(SAMPLE).unwrap();
        assert_eq!((47, 53), input.rules[0]);
        assert_eq!(vec![75, 47, 61, 53, 29], input.updates[0]);
    }

    #[test]
    fn test_well_ordered() {
        let input = Input::from_str(SAMPLE).unwrap();
        let rules = input.rules;

        // well-ordered
        assert!(well_ordered(&rules, &[75, 47, 61, 53, 29]));
        assert!(well_ordered(&rules, &[97, 61, 53, 29, 13]));
        assert!(well_ordered(&rules, &[75, 29, 13]));

        // Not well-ordered
        assert!(!well_ordered(&rules, &[75, 97, 47, 61, 53]));
        assert!(!well_ordered(&rules, &[61, 13, 29]));
        assert!(!well_ordered(&rules, &[97, 13, 75, 29, 47]));
    }

    #[test]
    fn test_reorder() {
        let input = Input::from_str(SAMPLE).unwrap();
        let rules = input.rules;

        // Not well-ordered
        let update = &mut [75, 97, 47, 61, 53];
        reorder(&rules, update);
        assert_eq!(update, &[97, 75, 47, 61, 53]);

        let update = &mut [61, 13, 29];
        reorder(&rules, update);
        assert_eq!(update, &[61, 29, 13]);

        let update = &mut [97, 13, 75, 29, 47];
        reorder(&rules, update);
        assert_eq!(update, &[97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 123);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(5);
}
