use linked_list::LinkedList;
use rayon::prelude::*;
use Error;

// 2018 AoC Day 5 puzzle
// https://adventofcode.com/2018/day/5

// Constant helps us compare lower and upper case letters
const CASE_DIFF: u8 = 32;

// Helper to identify same character with opposite case
fn reacts(a: u8, b: u8) -> bool {
    a == b + CASE_DIFF || b == a + CASE_DIFF
}

// This version uses a linked list with a cursor to actually remove matching elements
#[allow(dead_code)]
fn chain_react_counter(list: &mut LinkedList<u8>) -> usize {
    {
        let mut cursor = list.cursor();

        let mut prev = *cursor.next().unwrap();
        loop {
            let next = match cursor.peek_next() {
                Some(n) => *n,
                _ => break,
            };

            if reacts(prev, next) {
                cursor.prev();
                cursor.remove();
                cursor.remove();

                // Hacking around the borrow_checker. Eventually we should be able to write:
                //   `prev = match cursor.peek_prev().or_else(Cursor::next)`
                let tmp = cursor.peek_prev().map(|n| *n);
                prev = match tmp.or_else(|| cursor.next().map(|n| *n)) {
                    Some(n) => n,
                    None => break,
                };
            } else {
                prev = next;
                cursor.next();
            }
        }
    }
    list.iter().count()
}

// Instead of actually removing elements, this uses a more compact byte slice marking removed elements, and is experimentally much faster
// which goes to show that even when you think a LinkedList is the solution, you should benchmark it against a vector
fn fast_chain_react_counter(input: &mut [u8]) -> usize {
    let mut count = input.len();
    let mut prev = 0;
    let mut next = 1;

    let mut start = 0;
    while next < input.len() {
        if reacts(input[prev], input[next]) {
            input[prev] = 0;
            input[next] = 0;
            count -= 2;
            if prev == start {
                start = next + 1;
                prev = next + 1;
                next = next + 2;
            } else {
                while input[prev] == 0 {
                    prev -= 1;
                }
                next = next + 1;
            }
        } else {
            prev = next;
            next = prev + 1;
        }
    }
    count
}

pub fn part1(input: &str) -> Result<usize, Error> {
    let mut bytes = input.trim().as_bytes().to_owned();
    Ok(fast_chain_react_counter(&mut bytes))
}

pub fn part2(input: &str) -> Result<usize, Error> {
    let bytes = input.trim().as_bytes().to_owned();

    // Let rayon iterate over each character in parallel
    let min = (b'A'..(b'Z' + 1))
        .into_par_iter()
        .map(|c| {
            let mut list: Vec<_> = bytes
                .iter()
                .map(|b| *b)
                .filter(|b| c % CASE_DIFF != b % CASE_DIFF)
                .collect();
            fast_chain_react_counter(&mut list)
        })
        .min()
        .unwrap();
    Ok(min)
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1("aA").unwrap(), 0);
        assert_eq!(part1("abBA").unwrap(), 0);
        assert_eq!(part1("abAB").unwrap(), 4);
        assert_eq!(part1("aabAAB").unwrap(), 6);
        assert_eq!(part1("dabAcCaCBAcCcaDA").unwrap(), 10);
        assert_eq!(part1("zaAcdeEDCZ").unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA").unwrap(), 4);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-5.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-5.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }

}
