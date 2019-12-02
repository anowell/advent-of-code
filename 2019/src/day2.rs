use crate::util;
use anyhow::Result;

// 2019 AoC Day 2 puzzle
// https://adventofcode.com/2019/day/2

const OP_ADD: usize = 1;
const OP_MULT: usize = 2;
const OP_EXIT: usize = 99;

pub fn part1(input: &str) -> Result<u32> {
    let mut intcodes: Vec<usize> = util::split_parse(input, ',')?;
    // Fixed inputs from the problem description
    intcodes[1] = 12;
    intcodes[2] = 2;
    Ok(execute(intcodes))
}

pub fn part2(input: &str) -> Result<u32> {
    let intcodes: Vec<usize> = util::split_parse(input, ',')?;
    Ok(solve(intcodes, 19690720))
}

fn solve(intcodes: Vec<usize>, target: u32) -> u32 {
    // We know that the first opscode is 1 or 2
    // Therefore the next 2 positions (noun and verb) are "pointers"
    // and for those pointers to be valid, they need to point
    // to something within the intcodes array
    // hence the indices are less than the array length
    for noun in 0..intcodes.len() {
        for verb in 0..intcodes.len() {
            let mut intcodes = intcodes.clone();
            intcodes[1] = noun;
            intcodes[2] = verb;
            if execute(intcodes) == target {
                // magic formulat for proving that we have the right noun/verb
                return (100 * noun + verb) as u32;
            }
        }
    }
    unreachable!("bug in program or intcodes")
}

fn execute(intcodes: Vec<usize>) -> u32 {
    let mut intcodes = intcodes;
    let mut cursor = 0;
    let mut code = intcodes[cursor];

    // Beware: this doesn't bounds check. Lots of ways this could go wrong
    // 99: Exit opcode
    while code != OP_EXIT {
        let src1 = intcodes[cursor+1];
        let src2 = intcodes[cursor+2];
        let dest = intcodes[cursor+3];
        // println!("code {}, {} {} => {}", code, intcodes[src1], intcodes[src2], dest);
        intcodes[dest] = match code {
            // 1: add opcode
            OP_ADD => intcodes[src1] + intcodes[src2],
            // 2: multiply opcode
            OP_MULT => intcodes[src1] * intcodes[src2],
            _ => unreachable!(format!("Invalid code {}", code)),
        };
        // println!("head {}", intcodes[0]);
        cursor += 4;
        code = intcodes[cursor];
    }
    intcodes[0] as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use crate::test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(execute(vec![1,0,0,0,99]), 2);
        assert_eq!(execute(vec![2,3,0,3,99]), 2);
        assert_eq!(execute(vec![2,4,4,5,99,0]), 2);
        assert_eq!(execute(vec![1,1,1,4,99,5,6,0,99]), 30);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-2.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-2.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }
}
