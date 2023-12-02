use crate::util;
use anyhow::Result;

fn main() -> Result<()> {

}

pub fn part1(input: &str) -> Result<u32> {
    // let val = util::parse_lines(input)?;
    unimplemented!("Day DAY Part 1 not implemented")
}

pub fn part2(input: &str) -> Result<u32> {
    // let val = util::parse_lines(input)?;
    unimplemented!("Day DAY Part 2 not implemented")
}

fn helper(val: u32) -> u32  {
    unimplemented!("helper not implemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const sample: &str = r#"\"#;

    #[test]
    fn test_helper() {
        assert_eq!(helper(), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(sample), 0);
    }

    #[test]
    fn test_part2() {
      assert_eq!(part2(sample), 0);
  }
}

#[cfg(feature = "bench")]
mod bench {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_part1(c: &mut Criterion) {
        let input = ::std::fs::read_to_string("inputs/dayDAY").expect("Unable to open file");
        c.bench_function("day DAY-1", |b| b.iter(|| part1(&input).unwrap()));
    }

    fn bench_part2(c: &mut Criterion) {
        let input = ::std::fs::read_to_string("inputs/dayDAY").expect("Unable to open file");
        c.bench_function("day DAY-2", |b| b.iter(|| part1(&input).unwrap()));
    }

    criterion_group!(benches, bench_part1, bench_part2);
    criterion_main!(benches);
}

