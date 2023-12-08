//! [Advent of Code Day 1](https://adventofcode.com/2023/day/1)

use crate::prelude::*;
use aho_corasick::AhoCorasick;

/// Calculates the sum of calibration values (digits only)
pub fn part1(input: &str) -> Result<u32> {
    input
        .trim()
        .lines()
        .map(get_line_val)
        .try_fold(0, |acc, res| res.map(|val| acc + val))
}

/// Calculates the sum of calibration values (digits and words only)
pub fn part2(input: &str) -> Result<u32> {
    input
        .trim()
        .lines()
        .map(get_line_val2)
        .try_fold(0, |acc, res| res.map(|val| acc + val))
}

fn get_line_val(line: &str) -> Result<u32> {
    let bytes = line.as_bytes();
    let pattern = |c: char| c.is_ascii_digit();
    let c1 = bytes[line
        .find(pattern)
        .ok_or_else(|| format_err!("No digit in {line}"))?];
    let c2 = bytes[line.rfind(pattern).unwrap()];

    let d1 = (c1 - b'0') as u32;
    let d2 = (c2 - b'0') as u32;
    Ok(10 * d1 + d2)
}

const PATTERNS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

static AC_NUMBERS: Lazy<AhoCorasick> = Lazy::new(|| AhoCorasick::new(PATTERNS).unwrap());

fn get_line_val2(line: &str) -> Result<u32> {
    let mut matches = AC_NUMBERS.find_overlapping_iter(line);
    let m1 = matches
        .next()
        .ok_or_else(|| format_err!("No digit in {line}"))?;
    let m2 = matches.last().unwrap_or(m1);

    let d1 = decode(PATTERNS[m1.pattern()])?;
    let d2 = decode(PATTERNS[m2.pattern()])?;
    Ok(10 * d1 + d2)
}

fn decode(digit: &str) -> Result<u32> {
    let res = match digit {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        d => d.parse()?,
    };
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_get_line_val() {
        assert_eq!(get_line_val("1abc2").unwrap(), 12);
        assert_eq!(get_line_val("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(get_line_val("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(get_line_val("treb7uchet").unwrap(), 77);
    }

    #[test]
    fn test_get_line_val2() {
        assert_eq!(get_line_val2("two1nine").unwrap(), 29);
        assert_eq!(get_line_val2("eightwothree").unwrap(), 83);
        assert_eq!(get_line_val2("abcone2threexyz").unwrap(), 13);
        assert_eq!(get_line_val2("xtwone3four").unwrap(), 24);
        assert_eq!(get_line_val2("4nineeightseven2").unwrap(), 42);
        assert_eq!(get_line_val2("zoneight234").unwrap(), 14);
        assert_eq!(get_line_val2("7pqrstsixteen").unwrap(), 76);
        assert_eq!(get_line_val2("twone").unwrap(), 21);
    }

    const SAMPLE: &str = indoc! {"\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 142);
    }

    const SAMPLE2: &str = indoc! {"\
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2).unwrap(), 281);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(1);
}
