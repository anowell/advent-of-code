//! [Advent of Code Day 3](https://adventofcode.com/2024/day/3)

use crate::prelude::*;
use winnow::ascii::dec_uint;
use winnow::combinator::{delimited, opt, separated_pair};
use winnow::token::any;

pub fn part1(input: &str) -> Result<u32> {
    let mut buf = input;
    let pairs = extract_multiples(&mut buf).map_err(Error::msg)?;
    let total = pairs.iter().map(|(a, b)| a * b).sum();
    Ok(total)
}

pub fn part2(input: &str) -> Result<u32> {
    let mut buf = input;
    let pairs = extract_multiples_conditional(&mut buf).map_err(Error::msg)?;
    let total = pairs.iter().map(|(a, b)| a * b).sum();
    Ok(total)
}

fn extract_multiples(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    let mut list = Vec::new();
    while !input.is_empty() {
        // parse: mul(UINT,UINT)
        let mul_parser = delimited("mul(", separated_pair(dec_uint, ',', dec_uint), ")");
        if let Some(output) = opt(mul_parser).parse_next(input)? {
            list.push(output);
        } else {
            // discard a single token
            let _: PResult<char> = any(input);
        }
    }
    Ok(list)
}
fn extract_multiples_conditional(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    let mut list = Vec::new();
    let mut enabled = true;
    while !input.is_empty() {
        // parse: mul(UINT,UINT)
        let mul_parser = delimited("mul(", separated_pair(dec_uint, ',', dec_uint), ")");
        if enabled {
            if let Some(output) = opt(mul_parser).parse_next(input)? {
                list.push(output);
                continue;
            }
        }

        // state machine to parse `don't()`, `do()`, or discard a single token
        if opt("don't()").parse_next(input)?.is_some() {
            enabled = false;
        } else if opt("do()").parse_next(input)?.is_some() {
            enabled = true
        } else {
            let _: PResult<char> = any(input);
        }
    }
    Ok(list)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_parse() {
        let mut input = SAMPLE;
        assert_eq!(
            extract_multiples(&mut input).unwrap(),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        );
    }

    #[test]
    fn test_parse_conditional() {
        let mut input = SAMPLE2;
        assert_eq!(
            extract_multiples_conditional(&mut input).unwrap(),
            vec![(2, 4), (8, 5)]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2).unwrap(), 48);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(3);
}
