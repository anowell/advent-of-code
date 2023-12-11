//! [Advent of Code Day 11](https://adventofcode.com/2023/day/11)

use crate::math::manhattan_distance;
use crate::parse;
use crate::prelude::*;
use grid::Grid;

pub fn part1(input: &str) -> Result<usize> {
    let space = Universe::from_str(input)?;
    Ok(space.pairwise_distance_sum(2))
}

pub fn part2(input: &str) -> Result<usize> {
    let space = Universe::from_str(input)?;
    Ok(space.pairwise_distance_sum(1_000_000))
}

#[derive(Debug, Clone, Deref)]
struct Universe {
    grid: Grid<Space>,
}

impl Universe {
    fn empty_rows(&self) -> Vec<usize> {
        self.grid
            .iter_cols()
            .map(|col| col.map(|s| (*s as u32)).sum::<u32>() == 0)
            .enumerate()
            .filter(|(_, is_empty)| *is_empty)
            .map(|(i, _)| i)
            .collect_vec()
    }

    fn empty_cols(&self) -> Vec<usize> {
        self.grid
            .iter_rows()
            .map(|row| row.map(|s| (*s as u32)).sum::<u32>() == 0)
            .enumerate()
            .filter(|(_, is_empty)| *is_empty)
            .map(|(i, _)| i)
            .collect_vec()
    }

    fn galaxy_locations(&self, gap_distance: usize) -> Vec<(usize, usize)> {
        let empty_cols = self.empty_cols();
        let empty_rows = self.empty_rows();
        self.grid
            .indexed_iter()
            .filter(|(_, s)| **s == Space::Galaxy)
            .map(|((x, y), _)| {
                let x_gaps = empty_cols
                    .iter()
                    .find_position(|col| **col > x)
                    .map(|(pos, _)| pos)
                    .unwrap_or_else(|| empty_cols.len());
                let y_gaps = empty_rows
                    .iter()
                    .find_position(|row| **row > y)
                    .map(|(pos, _)| pos)
                    .unwrap_or_else(|| empty_rows.len());
                let dx = (gap_distance - 1) * x_gaps;
                let dy = (gap_distance - 1) * y_gaps;
                (x + dx, y + dy)
            })
            .collect()
    }

    fn pairwise_distance_sum(&self, gap_distance: usize) -> usize {
        self.galaxy_locations(gap_distance)
            .into_iter()
            .combinations(2)
            .map(|combo| manhattan_distance(combo[0], combo[1]))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Space {
    Empty = 0,
    Galaxy = 1,
}

impl From<char> for Space {
    fn from(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => unreachable!("Input contained unknown symbol"),
        }
    }
}

impl FromStr for Universe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = parse::parse_2d::<Space>(s)?;
        Ok(Universe { grid })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_parse() {
        let u = Universe::from_str(SAMPLE).unwrap();
        assert_eq!(u[(3, 0)], Space::Galaxy);
        assert_eq!(u[(0, 0)], Space::Empty);
    }

    #[test]
    fn test_empty_space() {
        let u = Universe::from_str(SAMPLE).unwrap();
        assert_eq!(u.empty_rows(), vec![3, 7]);
        assert_eq!(u.empty_cols(), vec![2, 5, 8]);
    }

    #[test]
    fn test_galaxy_locations() {
        let u = Universe::from_str(SAMPLE).unwrap();
        let g = u.galaxy_locations(2);
        assert_eq!(g[0], (4, 0));
        assert_eq!(g[1], (9, 1));
        assert_eq!(g[3], (8, 5));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 374);
    }

    #[test]
    fn test_part2() {
        let u = Universe::from_str(SAMPLE).unwrap();
        assert_eq!(u.pairwise_distance_sum(10), 1030);
        assert_eq!(u.pairwise_distance_sum(100), 8410);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(11);
}
