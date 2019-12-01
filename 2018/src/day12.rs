use regex::Regex;
use crate::Error;

// 2018 AoC Day 12 puzzle
// https://adventofcode.com/2018/day/12

fn print(slice: &[bool], offset: i64) {
    for _ in -30..=offset {
        print!(" ");
    }
    for b in slice {
        if *b {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

fn bools_to_u32(slice: &[bool]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

// turns out the pattern starts repeating just before the 100th generation
pub fn part1(input: &str) -> Result<i64, Error> {
    let config = parse_input(input)?;

    let mut pots = config.initial;
    let _zero_point: usize = 0;
    let map = config.map;

    let mut offset: i64 = 0;
    print(&pots, offset);
    for _ in 0..150 {
        let mut next = Vec::with_capacity(pots.capacity());

        if !pots.ends_with(&[false; 5]) {
            pots.append(&mut vec![false; 5]);
        }

        if !pots.starts_with(&[false; 5]) {
            for i in 1..5 {
                let bits = bools_to_u32(&pots[0..i]);
                next.push(map[bits as usize]);
            }
            offset -= 2;
        } else {
            offset += 2;
        }

        for i in 0..(pots.len() - 5) {
            let bits = bools_to_u32(&pots[i..(i + 5)]);
            next.push(map[bits as usize]);
        }
        pots = next;

        // println!("{}",
        //     pots
        //     .iter()
        //     .enumerate()
        //     .map(|(i, b)| (i as i64 + offset, b))
        //     .filter(|(i, b)| **b)
        //     .map(|(i, b)| i)
        //     .sum::<i64>()
        // );
        // print(&pots, offset);
    }

    Ok(pots
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i as i64 + offset, b))
        .filter(|(_, b)| *b)
        .map(|(i, _)| i)
        .sum::<i64>())
}

pub fn part2(_input: &str) -> Result<i32, Error> {
    // let idx = 120
    // let prev = val[idx];
    // let next = val[idx+1];
    // let delta = next - prev;  # 32

    // next + (25_000_000_000_u64 - index) * delta)

    unimplemented!();
}
lazy_static! {
    static ref RE1: Regex = Regex::new(r"^initial state: ([#\.]+)$").unwrap();
    static ref RE2: Regex = Regex::new(r"^([#.]+) => ([#.])$").unwrap();
}

struct Config {
    initial: Vec<bool>,
    map: [bool; 1 << 5],
}

fn pattern_to_idx(pat: &str) -> usize {
    let mut sum = 0;
    let bytes = pat.as_bytes();
    for i in 0..5 {
        if bytes[pat.len() - i - 1] == b'#' {
            sum += 1 << i;
        }
    }
    sum
}

fn parse_input(input: &str) -> Result<Config, Error> {
    let mut parts = input.trim().splitn(2, "\n");
    let initial = parts.next().unwrap();
    let rest = parts.next().unwrap();

    let initial_caps = RE1
        .captures(initial)
        .ok_or_else(|| format!("Initial input could not be parsed: {}", initial))?;
    let initial = initial_caps[1].bytes().map(|c| c == b'#').collect();

    let mut map = [false; 1 << 5];
    for line in rest.trim().lines() {
        let map_caps = RE2
            .captures(line)
            .ok_or_else(|| format!("Input could not be parsed: {}", input))?;
        if &map_caps[2] == "#" {
            let idx = pattern_to_idx(&map_caps[1]);
            map[idx] = true;
        }
    }

    Ok(Config { initial, map })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    const INPUT: &str = r#"
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"#;

    #[test]
    fn test_pattern_to_idx() {
        assert_eq!(pattern_to_idx("....."), 0);
        assert_eq!(pattern_to_idx("....#"), 1);
        assert_eq!(pattern_to_idx("..#.."), 4);
        assert_eq!(pattern_to_idx("#...."), 16);
        assert_eq!(pattern_to_idx("..###"), 7);
        assert_eq!(pattern_to_idx("#####"), 31);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 325);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1(INPUT).unwrap(), 325);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-12.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-12.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
