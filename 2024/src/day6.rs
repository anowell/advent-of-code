//! [Advent of Code Day 6](https://adventofcode.com/2024/day/6)

use std::collections::HashSet;

use crate::parse;
use crate::prelude::*;

pub fn part1(input: &str) -> Result<usize> {
    let map = Map::from_str(input).unwrap();
    let visits = map.count_unique_guard_visits();
    Ok(visits)
}

pub fn part2(input: &str) -> Result<usize> {
    let mut map = Map::from_str(input).unwrap();
    let count = map.count_injectable_loops();
    Ok(count)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn turn(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn deltas(&self) -> (i8, i8) {
        match self {
            Dir::N => (-1, 0),
            Dir::E => (0, 1),
            Dir::S => (1, 0),
            Dir::W => (0, -1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pos {
    Empty,
    Obstruction,
    Start(Dir),
}

impl TryFrom<char> for Pos {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Pos::Empty),
            '#' => Ok(Pos::Obstruction),
            '^' => Ok(Pos::Start(Dir::N)),
            '>' => Ok(Pos::Start(Dir::E)),
            'V' => Ok(Pos::Start(Dir::S)),
            '<' => Ok(Pos::Start(Dir::W)),
            _ => Err(format_err!("Unknown symbol {value}")),
        }
    }
}

struct Map(grid::Grid<Pos>);

impl Map {
    // Returns the start location
    fn start(&self) -> (usize, usize, Dir) {
        let cols = self.0.cols();
        for (idx, pos) in self.0.iter().enumerate() {
            if let Pos::Start(dir) = pos {
                return (idx / cols, idx % cols, *dir);
            }
        }
        panic!("No start found in map")
    }

    // Number of unique positions the guard will visit during their patrol
    fn count_unique_guard_visits(&self) -> usize {
        let (mut row, mut col, mut dir) = self.start();
        let mut visited = HashSet::new();
        visited.insert((row, col));

        while let Some((next_row, next_col, next_dir)) = self.next_patrol(row, col, dir) {
            row = next_row;
            col = next_col;
            dir = next_dir;
            visited.insert((row, col));
        }

        visited.len()
    }

    // Checking all squares in the patrol path benches at just under 1s
    fn count_injectable_loops(&mut self) -> usize {
        let (mut row, mut col, mut dir) = self.start();
        let mut obstructable = HashSet::new();

        while let Some((next_row, next_col, next_dir)) = self.next_patrol(row, col, dir) {
            let original = self.0[(next_row, next_col)];
            if original == Pos::Empty && !obstructable.contains(&(next_row, next_col)) {
                self.0[(next_row, next_col)] = Pos::Obstruction;
                if self.contains_loop() {
                    obstructable.insert((next_row, next_col));
                }
                self.0[(next_row, next_col)] = original;
            }

            row = next_row;
            col = next_col;
            dir = next_dir;
        }

        obstructable.len()
    }

    // Checking all squares benches at around 5s
    // fn count_injectable_loops(&mut self) -> usize {
    //     let mut count = 0;
    //     for r in 0..self.0.rows() {
    //         for c in 0..self.0.cols() {
    //             let original = self.0[(r, c)];
    //             if original == Pos::Empty {
    //                 self.0[(r, c)] = Pos::Obstruction;
    //                 if self.contains_loop() {
    //                     count += 1;
    //                 }
    //                 self.0[(r, c)] = original;
    //             }
    //         }
    //     }

    //     count
    // }

    fn contains_loop(&self) -> bool {
        let (mut row, mut col, mut dir) = self.start();
        let mut visited = HashSet::new();
        visited.insert((row, col, dir));

        while let Some((next_row, next_col, next_dir)) = self.next_patrol(row, col, dir) {
            row = next_row;
            col = next_col;
            dir = next_dir;

            if !visited.insert((row, col, dir)) {
                return true;
            }
        }

        false
    }

    fn next_patrol(&self, row: usize, col: usize, dir: Dir) -> Option<(usize, usize, Dir)> {
        match dir {
            Dir::N if row == 0 => None,
            Dir::S if row >= self.rows() - 1 => None,
            Dir::E if col >= self.cols() - 1 => None,
            Dir::W if col == 0 => None,
            _ => {
                let deltas = dir.deltas();
                let r = (row as isize + deltas.0 as isize) as usize;
                let c = (col as isize + deltas.1 as isize) as usize;
                match self.0[(r, c)] {
                    Pos::Obstruction => self.next_patrol(row, col, dir.turn()),
                    _ => Some((r, c, dir)),
                }
            }
        }
    }
}

impl std::ops::Deref for Map {
    type Target = grid::Grid<Pos>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = parse::parse_2d(s)?;
        Ok(Map(grid))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_parse() {
        let map = Map::from_str(SAMPLE).unwrap();
        assert_eq!(map[(0, 0)], Pos::Empty);
        assert_eq!(map[(0, 4)], Pos::Obstruction);
        assert_eq!(map[(9, 9)], Pos::Empty);
        assert_eq!(map[(6, 4)], Pos::Start(Dir::N));
    }

    #[test]
    fn test_next_patrol() {
        let map = Map::from_str(SAMPLE).unwrap();
        assert_eq!(map.next_patrol(0, 0, Dir::E), Some((0, 1, Dir::E)));
        assert_eq!(map.next_patrol(0, 3, Dir::E), Some((1, 3, Dir::S)));
    }

    const BASIC_LOOP: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#.#^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_contains_loop() {
        let map = Map::from_str(SAMPLE).unwrap();
        assert!(!map.contains_loop());
        let map = Map::from_str(BASIC_LOOP).unwrap();
        assert!(map.contains_loop());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 6);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(6);
}
