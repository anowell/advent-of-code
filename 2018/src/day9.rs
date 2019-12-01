use linked_list::{Cursor, LinkedList};
use regex::Regex;
use crate::Error;

// 2018 AoC Day 9 puzzle
// https://adventofcode.com/2018/day/9

// The cursor in the LinkedList imported puts an "ghost" element between head and tail
// This alternative cursor automatically skips the ghost to act like a true circular linked list
struct RingCursor<'a, T: 'a> {
    cursor: Cursor<'a, T>,
}

impl<'a, T> RingCursor<'a, T> {
    fn insert(&mut self, elem: T) {
        self.cursor.insert(elem)
    }
    fn remove(&mut self) -> T {
        match self.cursor.remove() {
            Some(elem) => elem,
            None => self.cursor.remove().unwrap(),
        }
    }
    fn seek_forward(&mut self, n: usize) {
        for _ in 0..n {
            if self.cursor.next().is_none() {
                self.cursor.next().unwrap();
            }
        }
    }
    fn seek_backward(&mut self, n: usize) {
        for _ in 0..n {
            if self.cursor.prev().is_none() {
                self.cursor.prev().unwrap();
            }
        }
    }
}

fn high_score(players: usize, last_marble: u32) -> u32 {
    let mut scores = vec![0; players];

    let mut circle = LinkedList::new();
    circle.push_back(0);
    let mut cursor = RingCursor {
        cursor: circle.cursor(),
    };

    for i in 1..(last_marble + 1) {
        if i % 23 == 0 {
            let player = (i as usize - 1) % scores.len();
            scores[player] += i;
            cursor.seek_backward(7);
            scores[player] += cursor.remove();
        } else {
            cursor.seek_forward(2);
            cursor.insert(i);
        }
    }

    scores.into_iter().max().unwrap()
}

pub fn part1(input: &str) -> Result<u32, Error> {
    let Config {
        players,
        last_marble,
    } = parse_line(input.trim())?;
    Ok(high_score(players, last_marble))
}

pub fn part2(input: &str) -> Result<u32, Error> {
    let Config {
        players,
        last_marble,
    } = parse_line(input.trim())?;
    Ok(high_score(players, last_marble * 100))
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
}

struct Config {
    players: usize,
    last_marble: u32,
}

fn parse_line(input: &str) -> Result<Config, Error> {
    let caps = RE
        .captures(input)
        .ok_or_else(|| format!("Input could not be parsed: {}", input))?;

    Ok(Config {
        players: caps[1].parse()?,
        last_marble: caps[2].parse()?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(high_score(9, 25), 32);
        assert_eq!(high_score(10, 1618), 8317);
        assert_eq!(high_score(13, 7999), 146373);
        assert_eq!(high_score(17, 1104), 2764);
        assert_eq!(high_score(21, 6111), 54718);
        assert_eq!(high_score(30, 5807), 37305);
    }

    // No test case for part 2 as it uses the exact same function high_score function

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-9.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-9.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
