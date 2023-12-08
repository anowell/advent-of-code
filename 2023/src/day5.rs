//! [Advent of Code Day 5](https://adventofcode.com/2023/day/5)

use crate::prelude::*;
use std::ops::Range;

/// Find the smallest seed location in the almanac
pub fn part1(input: &str) -> Result<u32> {
    let almanac = Almanac::from_str(input)?;
    almanac
        .seed_locations()
        .into_iter()
        .min()
        .ok_or_else(|| format_err!("No seeds to lookup"))
}

/// Find the smallest seed location in the almanac using seed ranges
pub fn part2(input: &str) -> Result<u32> {
    let almanac = Almanac::from_str(input)?;
    almanac
        .ranged_seed_locations()
        .into_iter()
        .map(|r| r.start)
        .min()
        .ok_or_else(|| format_err!("No seeds to lookup"))
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    /// Iterates through the maps to find the location of each seed
    /// Assumes maps are in lookup order, e.g. "seed-to-soil", then "soil-to-fertilizer", etc.
    pub fn seed_locations(&self) -> Vec<u32> {
        let mut locations = Vec::new();
        for seed in &self.seeds {
            let mut src = *seed;
            for map in &self.maps {
                src = map.dest(src);
            }
            locations.push(src);
        }
        locations
    }

    /// Iterates through the maps to find the location ranges corresponding to any of the seed ranges
    /// where the seeds line specifies ranges: "seed_range_start seed_range_len seed_range_start etc."
    pub fn ranged_seed_locations(&self) -> Vec<Range<u32>> {
        assert!(self.seeds.len() % 2 == 0);

        let seeds: Vec<Range<u32>> = self
            .seeds
            .chunks_exact(2)
            .map(|chunk| (chunk[0])..(chunk[0] + chunk[1]))
            .collect();

        let mut next = seeds;
        for map in &self.maps {
            next = map.dest_ranges(&next);
        }
        next
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Map {
    src_type: String,
    dest_type: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn dest(&self, src: u32) -> u32 {
        for m in &self.mappings {
            if m.src.contains(&src) {
                return src - m.src.start + m.dest.start;
            }
        }
        src
    }

    fn dest_ranges(&self, lookup: &[Range<u32>]) -> Vec<Range<u32>> {
        let mut mapped = Vec::new();
        let mut unmapped = lookup.to_owned();
        for m in &self.mappings {
            let mut next_unmapped = Vec::new();
            for range in unmapped {
                if let Some(mapped_range) = m.mapped(range.clone()) {
                    mapped.push(mapped_range);
                }
                next_unmapped.extend(m.unmapped(range))
            }
            unmapped = next_unmapped; // .extend(m.unmapped(src.clone()));
        }
        mapped.extend(unmapped);
        mapped
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Mapping {
    src: Range<u32>,
    dest: Range<u32>,
}

impl Mapping {
    fn unmapped(&self, lookup: Range<u32>) -> Vec<Range<u32>> {
        let mut unmapped = Vec::new();
        if lookup.start < self.src.start {
            let end = cmp::min(lookup.end, self.src.start);
            unmapped.push(lookup.start..end);
        }
        if lookup.end > self.src.end {
            let start = cmp::max(lookup.start, self.src.end);
            unmapped.push(start..lookup.end);
        }
        unmapped
    }

    fn mapped(&self, lookup: Range<u32>) -> Option<Range<u32>> {
        if lookup.start >= self.src.end || lookup.end <= self.src.start {
            return None;
        }
        let overlap_start = cmp::max(lookup.start, self.src.start);
        let overlap_end = cmp::min(lookup.end, self.src.end);

        let start_offset = overlap_start - self.src.start;
        let mapped_start = self.dest.start + start_offset;
        let mapped_end = mapped_start + (overlap_end - overlap_start);
        Some(mapped_start..mapped_end)
    }
}

impl FromStr for Mapping {
    type Err = Error;

    // Each line within a map contains three numbers:
    // the destination range start, the source range start, and the range length.
    fn from_str(s: &str) -> Result<Self> {
        let nums = crate::parse::extract_nums(s)?;
        assert!(nums.len() == 3);

        let src = nums[1]..(nums[1] + nums[2]);
        let dest = nums[0]..(nums[0] + nums[2]);
        Ok(Mapping { src, dest })
    }
}

static RE_LABELS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)-to-(\w+) map").unwrap());

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let seeds_line = lines
            .next()
            .ok_or_else(|| format_err!("No lines in input"))?;
        let seeds = crate::parse::extract_nums::<u32>(seeds_line)?;
        let mut maps: Vec<_> = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            if let Some(caps) = RE_LABELS.captures(line) {
                maps.push(Map {
                    src_type: caps[1].to_owned(),
                    dest_type: caps[2].to_owned(),
                    mappings: Vec::new(),
                });
            } else {
                maps.last_mut()
                    .unwrap()
                    .mappings
                    .push(Mapping::from_str(line)?)
            }
        }
        Ok(Almanac { seeds, maps })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use itertools::Itertools;

    const SAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_parse_map() {
        let mapping = Mapping::from_str("50 98 2").unwrap();
        assert_eq!(
            mapping,
            Mapping {
                src: 98..100,
                dest: 50..52
            }
        );
    }

    #[test]
    fn test_mapping_math() {
        // This mapping remaps the 50s into the 20s
        let mapping = Mapping::from_str("20 50 10").unwrap();

        assert_eq!(mapping.unmapped(10..20), vec![10..20]);
        assert_eq!(mapping.unmapped(100..200), vec![100..200]);
        assert_eq!(mapping.unmapped(50..60), vec![]);
        assert_eq!(mapping.unmapped(45..50), vec![45..50]);
        assert_eq!(mapping.unmapped(60..65), vec![60..65]);
        assert_eq!(mapping.unmapped(45..55), vec![45..50]);
        assert_eq!(mapping.unmapped(55..65), vec![60..65]);
        assert_eq!(mapping.unmapped(45..65), vec![45..50, 60..65]);
        assert_eq!(mapping.mapped(10..20), None);
        assert_eq!(mapping.mapped(100..200), None);
        assert_eq!(mapping.mapped(50..60), Some(20..30));
        assert_eq!(mapping.mapped(45..50), None);
        assert_eq!(mapping.mapped(60..65), None);
        assert_eq!(mapping.mapped(45..55), Some(20..25));
        assert_eq!(mapping.mapped(55..65), Some(25..30));
        assert_eq!(mapping.mapped(45..65), Some(20..30));
    }

    #[test]
    fn test_map_range_lookup() {
        let mappings = vec!["20 50 10", "50 70 5"]
            .into_iter()
            .map(|m| m.parse().unwrap())
            .collect_vec();

        let map = Map {
            src_type: String::new(),
            dest_type: String::new(),
            mappings,
        };

        let sorted_dest = |range| {
            let mut res = map.dest_ranges(&[range]);
            res.sort_by(|a, b| a.start.cmp(&b.start));
            res
        };

        assert_eq!(sorted_dest(10..20), vec![10..20]);
        assert_eq!(sorted_dest(50..60), vec![20..30]);
        assert_eq!(sorted_dest(40..60), vec![20..30, 40..50]);
        assert_eq!(
            sorted_dest(40..80),
            vec![20..30, 40..50, 50..55, 60..70, 75..80]
        );
    }

    #[test]
    fn test_parse_almanac() {
        let almanac = Almanac::from_str(SAMPLE).unwrap();
        assert_eq!(almanac.seeds.len(), 4);
        assert_eq!(almanac.maps.len(), 7);
        assert_eq!(almanac.maps[0].src_type, "seed");
        assert_eq!(almanac.maps[0].dest_type, "soil");
        assert_eq!(almanac.maps[6].src_type, "humidity");
        assert_eq!(almanac.maps[6].dest_type, "location");
        assert_eq!(almanac.maps[6].mappings[1].src, 93..97);
    }

    #[test]
    fn test_locations() {
        let almanac = Almanac::from_str(SAMPLE).unwrap();
        assert_eq!(almanac.seed_locations(), vec![82, 43, 86, 35]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 46);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(5);
}
