use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

aoc::setup!("dayX");

pub fn part1(input: &str) -> Result<u32> {
    let lines = aoc::parse::parse_lines::<Line>(input)?;
    todo!("Implement Part1");
}

pub fn part2(input: &str) -> Result<u32> {
    let lines = aoc::parse::parse_lines::<Line>(input)?;
    todo!("Implement Part2");
}

#[derive(Debug, Clone)]
struct Line {}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"PATTERN").unwrap());

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = RE.captures(s).ok_or_else(|| err!("Invalid line: {s}"))?;

        todo!("Implement Line::from_str")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        sample
        line
        data
    "};

    #[test]
    fn test_parse() {
        let sample = "TODO";
        let line = Line::from_str(sample);
        assert_eq(line, Line{});
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
    use super::*;

    #[divan::bench]
    fn bench_part1(bencher: divan::Bencher) {
        let input = aoc::input("dayX").unwrap();
        bencher.bench(|| part1(&input).unwrap());
    }

    #[divan::bench]
    fn bench_part2(bencher: divan::Bencher) {
        let input = aoc::input("dayX").unwrap();
        bencher.bench(|| part2(&input).unwrap());
    }
}
