use crate::util;
use anyhow::Result;

// 2019 AoC Day 1 puzzle
// https://adventofcode.com/2019/day/1

pub fn part1(input: &str) -> Result<u64> {
    let sum = util::parse_lines(input)?
        .into_iter()
        .map(fuel_required)
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<u64> {
    let sum = util::parse_lines(input)?
        .into_iter()
        .map(fuel_recurse)
        .sum();
    Ok(sum)
}

fn fuel_required(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn fuel_recurse(mass: u64) -> u64 {
    let mut fuel = 0;
    let mut new_mass = mass;
    loop {
        new_mass = fuel_required(new_mass);
        fuel += new_mass;
        if new_mass == 0 {
            return fuel;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use crate::test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(fuel_recurse(14), 2);
        assert_eq!(fuel_recurse(1969), 966);
        assert_eq!(fuel_recurse(100756), 50346);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-1.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-1.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }
}
