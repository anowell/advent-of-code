//! [Advent of Code Day 7](https://adventofcode.com/2024/day/7)

use crate::{parse, prelude::*};

pub fn part1(input: &str) -> Result<u64> {
    let lines = parse::parse_lines_with(input, Line::from_str)?;
    let ops = &[Operator::Add, Operator::Mult];
    let calibration = lines
        .iter()
        .filter(|l| l.has_solution(ops))
        .map(|l| l.total)
        .sum();
    Ok(calibration)
}

pub fn part2(input: &str) -> Result<u64> {
    let lines = parse::parse_lines_with(input, Line::from_str)?;
    let ops = &[Operator::Add, Operator::Mult, Operator::Concat];
    let calibration = lines
        .iter()
        .filter(|l| l.has_solution(ops))
        .map(|l| l.total)
        .sum();
    Ok(calibration)
}

#[derive(Debug, Clone)]
struct Line {
    total: u64,
    operands: Vec<u64>,
}

impl Line {
    #[allow(unused)]
    fn new(total: u64, operands: &[u64]) -> Line {
        Line {
            total,
            operands: operands.to_vec(),
        }
    }

    // Determines if the total can be achieved left-to-right with any of the specified operators
    fn has_solution(&self, operators: &[Operator]) -> bool {
        let mut partials = vec![self.operands[0]];
        for n in &self.operands[1..] {
            let mut next = Vec::new();
            for p in partials.iter_mut() {
                for op in operators {
                    next.push(match op {
                        Operator::Add => *p + *n,
                        Operator::Mult => *p * *n,
                        Operator::Concat => *p * 10_u64.pow(n.ilog10() + 1) + *n,
                    })
                }
            }
            partials = next;
        }

        partials.contains(&self.total)
    }
}

enum Operator {
    Add,
    Mult,
    Concat,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let nums = parse::extract_nums(s)?;
        let total = nums[0];
        let operands = nums[1..].to_vec();
        Ok(Line { total, operands })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_parse() {
        let line = Line::from_str("190: 10 19").unwrap();
        assert_eq!(line.total, 190);
        assert_eq!(line.operands, vec![10, 19]);
    }

    #[test]
    fn test_ltr_solution() {
        let ops = &[Operator::Add, Operator::Mult];
        assert!(Line::new(190, &[10, 19]).has_solution(ops));
        assert!(Line::new(3267, &[81, 40, 27]).has_solution(ops));
        assert!(!Line::new(83, &[17, 5]).has_solution(ops));
        assert!(Line::new(292, &[11, 6, 16, 20]).has_solution(ops));
        assert!(!Line::new(165, &[15, 6]).has_solution(ops));
        assert!(!Line::new(192, &[17, 8, 4]).has_solution(ops));

        let ops = &[Operator::Add, Operator::Mult, Operator::Concat];
        assert!(Line::new(156, &[15, 6]).has_solution(ops));
        assert!(Line::new(192, &[17, 8, 14]).has_solution(ops));
        assert!(Line::new(1755, &[17, 55]).has_solution(ops));
        assert!(Line::new(123456789, &[1, 234, 5678, 9]).has_solution(ops));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 11387);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(7);
}
