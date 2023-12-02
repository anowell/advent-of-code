use anyhow::{anyhow as err, Result};
use once_cell::sync::Lazy;
use regex::Regex;

aoc::setup!("day1");

pub fn part1(input: &str) -> Result<u32> {
    input
        .trim()
        .lines()
        .map(get_line_val)
        .try_fold(0, |acc, res| res.map(|val| acc + val))
}

pub fn part2(input: &str) -> Result<u32> {
    input
        .trim()
        .lines()
        .map(get_line_val2)
        .try_fold(0, |acc, res| res.map(|val| acc + val))
}

fn get_line_val(line: &str) -> Result<u32> {
    let bytes = line.as_bytes();
    let pattern = |c: char| (c >= '0') && (c <= '9');
    let c1 = bytes[line
        .find(pattern)
        .ok_or_else(|| err!("No digit in {line}"))?];
    let c2 = bytes[line.rfind(pattern).unwrap()];

    let d1 = (c1 - b'0') as u32;
    let d2 = (c2 - b'0') as u32;
    Ok(10 * d1 + d2)
}

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap());

fn get_line_val2(line: &str) -> Result<u32> {
    let mut matches = RE.find_iter(&line);
    let m1 = matches.next().ok_or_else(|| err!("No digit in {line}"))?;
    let mut m2 = matches.last().unwrap_or(m1.clone());

    // Special handling for m2 to catch overlapping input like: "twone"
    for i in (m2.start() + 1)..line.len() {
        if let Some(m) = RE.find_at(&line, i) {
            m2 = m;
        }
    }

    let d1 = decode(m1.as_str())?;
    let d2 = decode(m2.as_str())?;
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
    use super::*;

    #[divan::bench]
    fn bench_part1(bencher: divan::Bencher) {
        let input = aoc::input("day1").unwrap();
        bencher.bench(|| part1(&input).unwrap());
    }

    #[divan::bench]
    fn bench_part2(bencher: divan::Bencher) {
        let input = aoc::input("day1").unwrap();
        bencher.bench(|| part2(&input).unwrap());
    }
}
