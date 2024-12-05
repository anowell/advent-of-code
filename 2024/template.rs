//! [Advent of Code Day DAYNUM](https://adventofcode.com/2023/day/DAYNUM)

use crate::prelude::*;

pub fn part1(input: &str) -> Result<u32> {
    todo!("Implement Part1");
}

pub fn part2(input: &str) -> Result<u32> {
    todo!("Implement Part2");
}

#[derive(Debug, Clone)]
struct Line {}

impl Line {}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"PATTERN").unwrap());

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let _caps = RE
            .captures(s)
            .ok_or_else(|| format_err!("Invalid line: {s}"))?;
        todo!("Implement Line::from_str")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        sample
        data
    "};

    #[test]
    fn test_parse() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 0);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE).unwrap(), 0);
    // }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(DAYNUM);
}
