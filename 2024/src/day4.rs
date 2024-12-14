//! [Advent of Code Day 4](https://adventofcode.com/2024/day/4)
//!
//! A bit of an ugly solution.
//! - Part 1 is generic word search solution
//! - Part 2 is hard-coded to the unusual X-shaped 'MAS' intersection solution

use crate::parse;
use crate::prelude::*;
use grid::Grid;

pub fn part1(input: &str) -> Result<u32> {
    let ws = WordSearch::from_str(input)?;
    let count = ws.count_occurrences("XMAS");
    Ok(count as u32)
}

pub fn part2(input: &str) -> Result<u32> {
    let ws = WordSearch::from_str(input)?;
    let count = ws.count_x_mas();
    Ok(count as u32)
}

#[derive(Debug, Clone, Deref)]
struct WordSearch {
    grid: Grid<char>,
}

impl WordSearch {
    fn count_x_mas(&self) -> usize {
        let w = self.grid.cols();
        let h = self.grid.rows();
        let mut count = 0;

        for row in 1..h - 1 {
            for col in 1..w - 1 {
                if self.check_x_mas((row, col)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_occurrences(&self, word: &str) -> usize {
        let word_rev: String = word.chars().rev().collect();
        let w = self.grid.cols();
        let h = self.grid.rows();
        let mut count = 0;

        // Forward
        for row in 0..h {
            for col in 0..w {
                // FORWARD / BACKWARD
                if col + word.len() <= w {
                    if self.check_forward(word, (row, col)) {
                        // eprintln!("FORWARD at ({row},{col})");
                        count += 1;
                    }
                    if self.check_forward(&word_rev, (row, col)) {
                        // eprintln!("BACKWARD to ({row},{col})");
                        count += 1;
                    }
                }

                // UP / DOWN
                if row + word.len() <= h {
                    if self.check_down(word, (row, col)) {
                        // eprintln!("DOWN at ({row},{col})");
                        count += 1;
                    }
                    if self.check_down(&word_rev, (row, col)) {
                        // eprintln!("UP to ({row},{col})");
                        count += 1;
                    }
                }

                // DIAGONAL
                if col + word.len() <= w && row + word.len() <= h {
                    if self.check_down_right(word, (row, col)) {
                        // eprintln!("DOWN RIGHT at ({row},{col})");
                        count += 1;
                    }
                    if self.check_down_right(&word_rev, (row, col)) {
                        // eprintln!("UP LEFT to ({row},{col})");
                        count += 1;
                    }
                }

                if col + word.len() <= w && row + 1 >= word.len() {
                    if self.check_up_right(word, (row, col)) {
                        // eprintln!("UP RIGHT at ({row},{col})");
                        count += 1;
                    }
                    if self.check_up_right(&word_rev, (row, col)) {
                        // eprintln!("DOWN LEFT to ({row},{col})");
                        count += 1;
                    }
                }
            }
        }

        count
    }

    // Checks for MAS diagonals intersecting on the A at the specified coordinate
    fn check_x_mas(&self, (row, col): (usize, usize)) -> bool {
        assert!(row > 0);
        assert!(col > 0);
        assert!(col + 1 < self.grid.cols());
        assert!(row + 1 < self.grid.rows());

        if self.grid[(row, col)] != 'A' {
            return false;
        }

        let nw = self.grid[(row - 1, col - 1)];
        let ne = self.grid[(row - 1, col + 1)];
        let sw = self.grid[(row + 1, col - 1)];
        let se = self.grid[(row + 1, col + 1)];

        matches!(
            (nw, se, ne, sw),
            ('M', 'S', 'M', 'S')
                | ('M', 'S', 'S', 'M')
                | ('S', 'M', 'M', 'S')
                | ('S', 'M', 'S', 'M')
        )
    }

    fn check_forward(&self, word: &str, (row, col): (usize, usize)) -> bool {
        assert!(col + word.len() <= self.grid.cols());

        for (idx, c) in word.chars().enumerate() {
            if self.grid[(row, col + idx)] != c {
                return false;
            }
        }
        true
    }

    fn check_down(&self, word: &str, (row, col): (usize, usize)) -> bool {
        assert!(row + word.len() <= self.grid.rows());

        for (idx, c) in word.chars().enumerate() {
            if self.grid[(row + idx, col)] != c {
                return false;
            }
        }
        true
    }

    fn check_down_right(&self, word: &str, (row, col): (usize, usize)) -> bool {
        assert!(col + word.len() <= self.grid.cols());
        assert!(row + word.len() <= self.grid.rows());

        for (idx, c) in word.chars().enumerate() {
            if self.grid[(row + idx, col + idx)] != c {
                return false;
            }
        }
        true
    }

    fn check_up_right(&self, word: &str, (row, col): (usize, usize)) -> bool {
        assert!(col + word.len() <= self.grid.cols());
        assert!(row + 1 >= word.len());

        for (idx, c) in word.chars().enumerate() {
            if self.grid[(row - idx, col + idx)] != c {
                return false;
            }
        }
        true
    }
}

impl FromStr for WordSearch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = parse::parse_2d(s)?;
        Ok(WordSearch { grid })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const BASIC: &str = indoc! {"
        H.XMAS.O
        O......H
    "};

    const SAMPLE: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn test_parse() {
        let ws = WordSearch::from_str(SAMPLE).unwrap();
        assert_eq!(ws.grid.cols(), 10);
        assert_eq!(ws.grid.rows(), 10);

        let ws = WordSearch::from_str(BASIC).unwrap();
        assert_eq!(ws.grid.cols(), 8);
        assert_eq!(ws.grid.rows(), 2);

        assert_eq!(ws.grid[(0, 0)], 'H');
        assert_eq!(ws.grid[(1, 7)], 'H');
    }

    #[test]
    fn test_check_word() {
        let ws = WordSearch::from_str(SAMPLE).unwrap();
        assert!(ws.check_forward("MMMS", (0, 0)));
        assert!(ws.check_forward("XMAS", (0, 5)));
        assert!(ws.check_down("MMAM", (0, 0)));
        assert!(ws.check_down_right("MSXM", (0, 0)));
        assert!(ws.check_up_right("MMAS", (3, 0)));
    }

    const CARDINAL: &str = indoc! {"
        XMASAMX
        M..A..M
        A..M..A
        SAMXMAS
    "};

    const DIAGONAL: &str = indoc! {"
        X.SS.X
        .MAAM.
        .MAAM.
        X.SS.X
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 18);
        assert_eq!(part1(BASIC).unwrap(), 1);
        assert_eq!(part1(CARDINAL).unwrap(), 7);
        assert_eq!(part1(DIAGONAL).unwrap(), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 9);
        assert_eq!(part2(DIAGONAL).unwrap(), 0);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(4);
}
