use std::collections::HashSet;
use std::num::ParseIntError;
use Error;

// 2018 AoC Day 1 puzzle
// https://adventofcode.com/2018/day/1

pub fn part1(input: &str) -> Result<i32, Error> {
    let sum = parse_nums(input)?.iter().sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<i32, Error> {
    let mut found = HashSet::new();
    let mut total = 0;
    found.insert(total);

    // The problem description allows for infinite loops. We're capping it at 1M iterations
    for num in parse_nums(input)?.iter().cycle().take(1_000_000) {
        total += num;
        if !found.insert(total) {
            return Ok(total);
        }
    }
    Err("Duplicate frequency not found after 1 million iterations".into())
}

fn parse_nums(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .trim()
        .split(|c| c == ',' || c == '\n')
        .map(str::trim)
        .map(str::parse)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1("+1, -2, +3, +1").unwrap(), 3);
        assert_eq!(part1("+1, +1, +1").unwrap(), 3);
        assert_eq!(part1("+1, +1, -2").unwrap(), 0);
        assert_eq!(part1("-1, -2, -3").unwrap(), -6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("+1, -2, +3, +1").unwrap(), 2);
        assert_eq!(part2("+1, -1").unwrap(), 0);
        assert_eq!(part2("+3, +3, +4, -2, -4").unwrap(), 10);
        assert_eq!(part2("-6, +3, +8, +5, -6").unwrap(), 5);
        assert_eq!(part2("+7, +7, -2, -7, -4").unwrap(), 14);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-1.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }
}
