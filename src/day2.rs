use fxhash::FxHashMap;
use Error;

// 2018 AoC Day 2 puzzle
// https://adventofcode.com/2018/day/2

pub fn part1(input: &str) -> Result<u32, Error> {
    let mut doubles = 0;
    let mut triples = 0;

    for line in input.trim().lines() {
        // FxHashMap benches about 30% faster than HashMap for this solution
        // Also, unlike HashMap, instantiating a FxHashMap for this problem is marginally faster than clearing and reusing one
        let mut char_counts = FxHashMap::default();
        let mut found_double = false;
        let mut found_triple = false;

        for c in line.chars() {
            let val = char_counts.entry(c).or_insert(0);
            *val += 1;
        }

        for count in char_counts.values() {
            if !found_double && *count == 2 {
                doubles += 1;
                found_double = true;
            }
            if !found_triple && *count == 3 {
                triples += 1;
                found_triple = true;
            }

            // Unnecessary, but benchmarks about 2-4% faster
            if found_double && found_triple {
                break;
            }
        }
    }

    Ok(doubles * triples)
}

pub fn part2(input: &str) -> Result<String, Error> {
    let lines: Vec<_> = input.trim().lines().collect();
    let line_count = lines.len();

    for i in 0..line_count {
        'next_line: for j in i..line_count {
            let mut offset = None;
            // Using bytes is about 25% faster than chars, but we :heart: UTF-8
            for (k, (a, b)) in lines[i].chars().zip(lines[j].chars()).enumerate() {
                if a != b {
                    if offset.is_none() {
                        offset = Some(k);
                    } else {
                        continue 'next_line;
                    }
                }
            }

            // Only allocate new strings in the inner loop if we found the match
            // Previously, string allocating in the inner loop was ~90% of the execution time
            if let Some(offset) = offset {
                let a = lines[i].chars().take(offset);
                let b = lines[i].chars().skip(offset + 1);
                let ret: String = a.chain(b).collect();
                return Ok(ret);
            }
        }
    }

    Err("Did not find any strings with only a single character difference".into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        let input = r#"
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"#;
        assert_eq!(part1(input).unwrap(), 12);
    }

    #[test]
    fn test_part2() {
        let input = r#"
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"#;
        assert_eq!(&part2(input).unwrap(), "fgij");
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
        b.iter(|| part2(&input).unwrap());
    }

}
