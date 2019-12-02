use crate::util;
use anyhow::Result;

// 2019 AoC Day DAY puzzle
// https://adventofcode.com/2019/day/DAY

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

    #[cfg(feature = "bench")]
    use crate::test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(helper(1), 1);
        assert_eq!(helper(2), 2);
        assert_eq!(helper(3), 3);
    }

    #[test]
    fn test_part2() {
      assert_eq!(helper(1), 1);
      assert_eq!(helper(2), 2);
      assert_eq!(helper(3), 3);
  }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-DAY.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-DAY.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }
}
