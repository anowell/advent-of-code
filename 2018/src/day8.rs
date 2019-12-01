use crate::Error;

// 2018 AoC Day 8 puzzle
// https://adventofcode.com/2018/day/8

struct RecursiveSum {
    sum: u32,
    len: usize,
}

fn sum_meta_count(list: &[u32], node_count: usize) -> RecursiveSum {
    let mut sum = 0;

    let mut i: usize = 0;
    for _j in 0..node_count {
        let child_count = list[i] as usize;
        let meta_count = list[i + 1] as usize;
        i += 2;

        if child_count > 0 {
            let res = sum_meta_count(&list[i..], child_count);
            i += res.len;
            sum += res.sum;
        }

        if meta_count > 0 {
            sum += list[i..(i + meta_count)].iter().sum::<u32>();
            i += meta_count;
        }
    }

    RecursiveSum { sum, len: i }
}

struct RecursiveVals {
    vals: Vec<u32>,
    len: usize,
}

fn get_node_values(list: &[u32], node_count: usize) -> RecursiveVals {
    let mut i: usize = 0;
    let mut node_vals = vec![0; node_count];
    for j in 0..node_count {
        let child_count = list[i] as usize;
        let meta_count = list[i + 1] as usize;
        i += 2;

        let mut child_vals = vec![0; child_count];
        if child_count > 0 {
            let res = get_node_values(&list[i..], child_count);
            i += res.len;
            child_vals = res.vals;
        }

        if child_count > 0 {
            for k in i..(i + meta_count) {
                if list[k] == 0 {
                    continue;
                }
                let node_idx = list[k] as usize - 1;
                if let Some(meta) = child_vals.get(node_idx) {
                    node_vals[j] += meta;
                }
            }
            i += meta_count;
        } else if meta_count > 0 {
            node_vals[j] = list[i..(i + meta_count)].iter().sum::<u32>();
            i += meta_count;
        }
    }

    RecursiveVals {
        vals: node_vals,
        len: i,
    }
}

pub fn part1(input: &str) -> Result<u32, Error> {
    let elements = parse_input(&input)?;
    Ok(sum_meta_count(&elements, 1).sum)
}

pub fn part2(input: &str) -> Result<u32, Error> {
    let elements = parse_input(&input)?;
    Ok(get_node_values(&elements, 1).vals[0])
}

fn parse_input(input: &str) -> Result<Vec<u32>, Error> {
    let vec = input
        .trim()
        .split(' ')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    Ok(vec)
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 66);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-8.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-8.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
