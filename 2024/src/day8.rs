//! [Advent of Code Day 8](https://adventofcode.com/2024/day/8)

use std::collections::{HashMap, HashSet};

use maplit::hashset;

use crate::prelude::*;

pub fn part1(input: &str) -> Result<usize> {
    let map = Map::from_str(input)?;
    let count = map.count_antinodes(false);
    Ok(count)
}

pub fn part2(input: &str) -> Result<usize> {
    let map = Map::from_str(input)?;
    let count = map.count_antinodes(true);
    Ok(count)
}

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct Antenna {
    freq: char,
    coord: Coord,
}

#[derive(Debug, Clone)]
struct Map {
    antennas: Vec<Antenna>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn count_antinodes(&self, include_harmonics: bool) -> usize {
        // Group all the antennas by frequency
        let mut antenna_map: HashMap<char, HashSet<Coord>> = HashMap::new();
        for a in &self.antennas {
            if let Some(set) = antenna_map.get_mut(&a.freq) {
                set.insert(a.coord);
            } else {
                antenna_map.insert(a.freq, hashset!(a.coord));
            }
        }

        // For each frequency, find the antinodes
        let mut antinodes = HashSet::new();
        for (_freq, coords) in antenna_map {
            let antinode_coords = calc_antinodes(self.rows, self.cols, &coords, include_harmonics);
            antinodes = antinodes.union(&antinode_coords).cloned().collect();
        }
        antinodes.len()
    }
}

// Rust learning
// HashSet<Coord> implements IntoIter<Item=Coord, IntoIter=hash_set::IntoIter
//   hash_set::IntoIter does NOT implement Clone, so we can't tuple_combinations on it
// HashSet<&Coord> implements IntoIter<Item=&Coord, IntoIter=hash_set::Iter
//   hash_set::Iter DOES implement Clone
// That's why we added the lifetime annotations.
//
// Of course, we could have implemented this to just take a &HashSet and use elided lifetimes,
// but where is the learning in that
fn calc_antinodes<'a, C>(
    rows: usize,
    cols: usize,
    coords: C,
    include_harmonics: bool,
) -> HashSet<Coord>
where
    C: IntoIterator<Item = &'a Coord>,
    <C as IntoIterator>::IntoIter: Clone,
{
    let mut antinodes = HashSet::new();
    for (a, b) in coords
        .into_iter()
        .map(|(r, c)| (*r as isize, *c as isize))
        .tuple_combinations()
    {
        let row_diff = b.0 - a.0;
        let col_diff = b.1 - a.1;

        let rows = rows as isize;
        let cols = cols as isize;
        let in_bounds = |(row, col)| row >= 0 && col >= 0 && row < rows && col < cols;

        if include_harmonics {
            // Antennas count as harmonics
            antinodes.insert((a.0 as usize, a.1 as usize));
            antinodes.insert((b.0 as usize, b.1 as usize));

            let mut an = (a.0 - row_diff, a.1 - col_diff);
            while in_bounds(an) {
                antinodes.insert((an.0 as usize, an.1 as usize));
                an = (an.0 - row_diff, an.1 - col_diff);
            }
            let mut an = (b.0 + row_diff, b.1 + col_diff);
            while in_bounds(an) {
                antinodes.insert((an.0 as usize, an.1 as usize));
                an = (an.0 + row_diff, an.1 + col_diff);
            }
        } else {
            let an1 = (a.0 - row_diff, a.1 - col_diff);
            if in_bounds(an1) {
                antinodes.insert((an1.0 as usize, an1.1 as usize));
            }
            let an2 = (b.0 + row_diff, b.1 + col_diff);
            if in_bounds(an2) {
                antinodes.insert((an2.0 as usize, an2.1 as usize));
            }
        }
    }
    antinodes
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut antennas = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        for (row, line) in s.lines().enumerate() {
            rows += 1;
            for (col, freq) in line.chars().enumerate() {
                if row == 0 {
                    cols += 1;
                }
                if freq.is_ascii_alphanumeric() {
                    antennas.push(Antenna {
                        freq,
                        coord: (row, col),
                    });
                }
            }
        }
        Ok(Map {
            antennas,
            rows,
            cols,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_parse() {
        let map = Map::from_str(SAMPLE).unwrap();
        assert_eq!(map.antennas.len(), 7);
        assert_eq!(map.antennas[0].freq, '0');
        assert_eq!(map.antennas[0].coord, (1, 8));
        assert_eq!(map.rows, 12);
        assert_eq!(map.cols, 12);
    }

    #[test]
    fn test_antinodes() {
        let ca = |nodes| calc_antinodes(10, 10, nodes, false);
        assert_eq!(ca(&[(2, 2), (4, 4)]), hashset![(0, 0), (6, 6)]);
        assert_eq!(ca(&[(2, 4), (4, 2)]), hashset![(6, 0), (0, 6)]);
        assert_eq!(ca(&[(4, 2), (2, 4)]), hashset![(6, 0), (0, 6)]);
        assert_eq!(ca(&[(4, 4), (2, 2)]), hashset![(0, 0), (6, 6)]);
        assert_eq!(ca(&[(2, 2), (2, 5)]), hashset![(2, 8)]);
    }

    #[test]
    fn test_harmonics() {
        let ca = |nodes| calc_antinodes(10, 10, nodes, true);
        assert_eq!(
            ca(&[(2, 2), (4, 4)]),
            hashset![(0, 0), (2, 2), (4, 4), (6, 6), (8, 8)]
        );
        assert_eq!(
            ca(&[(2, 4), (4, 2)]),
            hashset![(6, 0), (4, 2), (2, 4), (0, 6)]
        );
        assert_eq!(ca(&[(2, 2), (2, 5)]), hashset![(2, 2), (2, 5), (2, 8)]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 14);
    }

    const BASIC: &str = indoc!(
        "
        T....#....
        ...T......
        .T....#...
        .........#
        ..#.......
        ..........
        ...#......
        ..........
        ....#.....
        ..........
    "
    );

    #[test]
    fn test_part2() {
        assert_eq!(part2(BASIC).unwrap(), 9);
        assert_eq!(part2(SAMPLE).unwrap(), 34);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(8);
}
