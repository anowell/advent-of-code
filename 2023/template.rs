use anyhow::{anyhow as err, Result};
use once_cell::sync::Lazy;
use regex::Regex;

aoc::setup!("dayX");

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"PATTERN").unwrap());

pub fn part1(input: &str) -> Result<u32> {
    input
        .trim()
        .lines()
        .map(helper)
        .try_fold(0, |acc, res| res.map(|val| acc + val))
}

pub fn part2(input: &str) -> Result<u32> {
    input
        .trim()
        .lines()
        .map(helper)
        .try_fold(0, |acc, res| res.map(|val| acc + val))
}

fn helper(line: &str) -> Result<u32> {
    todo!("Implement helper")
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_helper() {
        assert_eq!(helper("sample").unwrap(), EXPECTED);
        assert_eq!(helper("sample2").unwrap(), EXPECTED);
    }

    const SAMPLE: &str = indoc! {"
        sample
        line
        data
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), EXPECTED);
    }

    const SAMPLE2: &str = indoc! {"
        more
        sample
        data
    "};

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2).unwrap(), EXPECTED);
    }
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
