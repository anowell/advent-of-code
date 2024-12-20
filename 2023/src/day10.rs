//! [Advent of Code Day 10](https://adventofcode.com/2023/day/10)

use crate::prelude::*;
use grid::Grid;

pub fn part1(input: &str) -> Result<usize> {
    let field = Field::from_str(input)?;
    let cycle = field.find_loop_path();
    Ok((cycle.len() + 1) / 2)
}

pub fn part2(input: &str) -> Result<u32> {
    let mut field = Field::from_str(input)?;
    field.remove_unused_pipes();
    let count = field.count_enclosed_ground();
    Ok(count)
}

#[derive(Debug, Clone, Deref)]
struct Field {
    tiles: Grid<Tile>,
}

use parse_display::{Display, FromStr, ParseError};
#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq)]
enum Tile {
    #[display(".")]
    Ground,
    #[display("|")]
    PipeNS,
    #[display("-")]
    PipeWE,
    #[display("L")]
    PipeNE,
    #[display("J")]
    PipeNW,
    #[display("7")]
    PipeSW,
    #[display("F")]
    PipeSE,
    #[display("S")]
    Start,
}

impl TryFrom<char> for Tile {
    type Error = ParseError;
    fn try_from(c: char) -> std::result::Result<Tile, Self::Error> {
        let mut tmp = [0];
        let s = c.encode_utf8(&mut tmp);
        Tile::from_str(s)
    }
}

#[derive(Debug, Clone, Copy, Display, PartialEq)]
#[display("({x},{y})")]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Coord {
        Coord { x, y }
    }
}

impl Coord {
    fn offset(&self, dx: isize, dy: isize) -> Coord {
        let x = self.x as isize;
        let y = self.y as isize;
        assert!(x + dx >= 0);
        assert!(y + dy >= 0);
        Coord {
            x: (x + dx) as usize,
            y: (y + dy) as usize,
        }
    }

    fn diff(&self, other: &Coord) -> (isize, isize) {
        (
            other.x as isize - self.x as isize,
            other.y as isize - self.y as isize,
        )
    }
}

impl Field {
    pub fn find_start(&self) -> Coord {
        let i = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, t)| **t == Tile::Start)
            .unwrap()
            .0;
        self.coord_from_index(i)
    }

    fn coord_from_index(&self, i: usize) -> Coord {
        Coord::from((i % self.cols(), i / self.cols()))
    }

    fn tile<C: Into<Coord>>(&self, coord: C) -> Tile {
        let coord = coord.into();
        self.tiles[(coord.x, coord.y)]
    }

    fn connected_neighbors(&self, coord: Coord) -> Option<(Coord, Coord)> {
        let max_x = self.cols() - 1;
        let max_y = self.rows() - 1;
        match self.tile(coord) {
            Tile::PipeNS if coord.y > 0 && coord.y < max_y => {
                Some((coord.offset(0, 1), coord.offset(0, -1)))
            }
            Tile::PipeWE if coord.x > 0 && coord.x < max_x => {
                Some((coord.offset(-1, 0), coord.offset(1, 0)))
            }
            Tile::PipeNE if coord.x < max_x && coord.y > 0 => {
                Some((coord.offset(0, -1), coord.offset(1, 0)))
            }
            Tile::PipeSE if coord.x < max_x && coord.y < max_y => {
                Some((coord.offset(0, 1), coord.offset(1, 0)))
            }
            Tile::PipeSW if coord.x > 0 && coord.y < max_y => {
                Some((coord.offset(0, 1), coord.offset(-1, 0)))
            }
            Tile::PipeNW if coord.x > 0 && coord.y > 0 => {
                Some((coord.offset(0, -1), coord.offset(-1, 0)))
            }
            Tile::Start => {
                let mut offsets: Vec<(isize, isize)> = vec![];
                if coord.x > 0 {
                    offsets.push((-1, 0));
                }
                if coord.x < max_x {
                    offsets.push((1, 0));
                }
                if coord.y > 0 {
                    offsets.push((0, -1));
                }
                if coord.y < max_y {
                    offsets.push((0, 1));
                }
                let coords: Option<(Coord, Coord)> = offsets
                    .into_iter()
                    .flat_map(|(dx, dy)| {
                        let c = coord.offset(dx, dy);
                        match self.tile(c) {
                            Tile::Start => unreachable!("multiple start tiles?"),
                            Tile::Ground => None,
                            Tile::PipeNS if dy != 0 => Some(c),
                            Tile::PipeWE if dx != 0 => Some(c),
                            Tile::PipeNE if dy == 1 || dx == -1 => Some(c),
                            Tile::PipeNW if dy == 1 || dx == 1 => Some(c),
                            Tile::PipeSE if dy == -1 || dx == -1 => Some(c),
                            Tile::PipeSW if dy == -1 || dx == 1 => Some(c),
                            _ => None,
                        }
                    })
                    .collect_tuple();
                Some(coords.expect("Starting point doesn't connect to exactly 2 pipe tiles"))
            }
            Tile::Ground => None,
            _ => unreachable!("Found a tile in a location where it can't be traversed"),
        }
    }

    // Returns the path of the loop
    fn find_loop_path(&self) -> Vec<Coord> {
        let start = self.find_start();
        let mut path = vec![start];

        // Pick one of the start-connected neighbors
        let mut current = self.connected_neighbors(start).unwrap().0;
        let mut prev = start;
        let mut next;
        while current != start {
            path.push(current);
            // eprintln!("Iter: {}", path.iter().map(|c| c.to_string()).join(" -> "));
            let connections = self
                .connected_neighbors(current)
                .expect("Pipes should always return 2 neighbors");

            next = match connections {
                (a, _) if a != prev => a,
                (_, b) if b != prev => b,
                (_, _) => unreachable!(
                    "Neither of the connected neighbors is the previous traversed neighbor"
                ),
            };
            prev = current;
            current = next;
        }
        // eprintln!("Path: {}", path.iter().map(|c| c.to_string()).join(" -> "));
        path
    }

    fn remove_unused_pipes(&mut self) {
        let loop_coords = self.find_loop_path();
        let (a, start, b) = (
            *(loop_coords.last().unwrap()),
            loop_coords[0],
            loop_coords[1],
        );
        let (cols, rows) = self.tiles.size();
        let items = vec![Tile::Ground; cols * rows];
        let mut grid = Grid::from_vec_with_order(items, cols, grid::Order::ColumnMajor);
        for c in loop_coords {
            let tile = self.tiles[(c.x, c.y)];
            grid[(c.x, c.y)] = tile;
        }
        let a_delta = start.diff(&a);
        let b_delta = start.diff(&b);
        grid[(start.x, start.y)] = match (a_delta, b_delta) {
            ((0, _), (0, _)) => Tile::PipeNS,
            ((_, 0), (_, 0)) => Tile::PipeWE,
            ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => Tile::PipeSE,
            ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => Tile::PipeSW,
            ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => Tile::PipeNE,
            ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => Tile::PipeNW,
            _ => unreachable!("Start had neighbor deltas {:?} and {:?}", a_delta, b_delta),
        };
        self.tiles = grid
    }

    fn count_enclosed_ground(&self) -> u32 {
        let mut count = 0;
        let mut crossed_wall = false;
        let mut last_corner = None::<Tile>;

        for row in self.tiles.iter_rows() {
            crossed_wall = false;
            last_corner = None;
            for tile in row {
                match (*tile, last_corner) {
                    (Tile::Ground, _) if crossed_wall => count += 1,
                    (Tile::Ground, _) => (),
                    (Tile::PipeNS, _) => crossed_wall = !crossed_wall,

                    (Tile::PipeNE, None) => last_corner = Some(Tile::PipeNE),
                    (Tile::PipeNW, Some(Tile::PipeNE)) => last_corner = None,
                    (Tile::PipeSW, Some(Tile::PipeNE)) => crossed_wall = !crossed_wall,

                    (Tile::PipeSE, None) => last_corner = Some(Tile::PipeSE),
                    (Tile::PipeNW, Some(Tile::PipeSW)) => crossed_wall = !crossed_wall,
                    (Tile::PipeSW, Some(Tile::PipeSE)) => last_corner = None,

                    (found, Some(after)) => unreachable!("Found {} after {}", found, after),
                    (found, None) => unreachable!("Found {}", found),
                }
            }
        }
        count
    }
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tiles: Grid<Tile> = crate::parse::parse_2d::<Tile>(s)?;
        Ok(Field { tiles })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const SAMPLE2: &str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "};

    const SAMPLE3: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_parse() {
        let tile = |c: &str| c.parse::<Tile>().unwrap();
        assert_eq!(tile("|"), Tile::PipeNS);
        assert_eq!(tile("J"), Tile::PipeNW);
        assert_eq!(tile("S"), Tile::Start);
        assert_eq!(tile("."), Tile::Ground);
        let field = Field::from_str(SAMPLE).unwrap();
        assert_eq!(field.tile((0, 0)), Tile::Ground);
        assert_eq!(field.tile((1, 1)), Tile::Start);
        assert_eq!(field.tile((3, 1)), Tile::PipeSW);
        assert_eq!(field.tile((3, 3)), Tile::PipeNW);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 4);
        assert_eq!(part1(SAMPLE2).unwrap(), 4);
        assert_eq!(part1(SAMPLE3).unwrap(), 8);
    }

    const SAMPLE4: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const SAMPLE5: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 1);
        assert_eq!(part2(SAMPLE2).unwrap(), 1);
        assert_eq!(part2(SAMPLE4).unwrap(), 4);
        assert_eq!(part2(SAMPLE5).unwrap(), 8);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(10);
}
