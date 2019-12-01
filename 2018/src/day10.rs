use regex::Regex;
use std::cmp::{max, min};
use std::i32::MAX as MAX_I32;
use crate::Error;

// 2018 AoC Day 10 puzzle
// https://adventofcode.com/2018/day/10

// Renders grid in the smallest region possible, with '#' for points and '.' for empty coordinates
fn render_grid(points: &[Point], pmin: Coord, pmax: Coord) -> String {
    let mut render = String::new();

    let width = (pmax.x - pmin.x + 1) as usize;
    let height = (pmax.y - pmin.y + 1) as usize;
    let mut grid = vec![false; width * height];

    for p in points {
        let x = (p.x - pmin.x) as usize;
        let y = (p.y - pmin.y) as usize;
        grid[y * width + x] = true;
    }

    for (i, cell) in grid.into_iter().enumerate() {
        if cell {
            render.push('#');
        } else {
            render.push('.');
        }
        if i % width == width - 1 {
            render.push('\n')
        }
    }

    // Simple and slow
    // for y in (pmin.y)..(pmax.y+1) {
    //     for x in (pmin.x)..(pmax.x+1) {
    //         if points.iter().find(|p| p.x == x && p.y == y).is_some() {
    //             render.push('#');
    //         } else {
    //             render.push('.');
    //         }
    //     }
    //     render.push('\n');
    // }
    render
}

// Calculates the bounding points of the grid (min and max x and y values)
// This is a bit lazy, since Point has velocity fields too that we're ignoring
fn bounding_box(points: &[Point]) -> (Coord, Coord) {
    let mut iter = points.iter();
    let p = iter.next().unwrap();
    let mut pmin = Coord { x: p.x, y: p.y };
    let mut pmax = Coord { x: p.x, y: p.y };
    for p in iter {
        pmin.x = min(pmin.x, p.x);
        pmax.x = max(pmax.x, p.x);
        pmin.y = min(pmin.y, p.y);
        pmax.y = max(pmax.y, p.y);
    }
    (pmin, pmax)
}

// Advance points according to the veolocity fields
fn step_points(points: &mut [Point]) {
    for p in points {
        p.x += p.dx;
        p.y += p.dy;
    }
}

// We're going to assume the points start by moving inward, namely "shrinking the grid"
// and that the smallest grid is the one containing the message, after which point, the grid will expand indefinitely
// Also, I don't really have it in me to do ASCII-Art-to-String, so we're going to return the entire ASCII Art
pub fn part1(input: &str) -> Result<String, Error> {
    let mut points = parse_input(input)?;

    let mut last_height = MAX_I32;
    let mut grid = String::new();
    loop {
        let (pmin, pmax) = bounding_box(&points);
        let height = pmax.y - pmin.y;
        if height > last_height {
            break;
        }
        last_height = height;
        if height < 12 {
            grid = render_grid(&points, pmin, pmax);
        }

        step_points(&mut points);
    }

    if grid.is_empty() {
        Err("Failed to find small grid to identify as containing a message".into())
    } else {
        Ok(grid)
    }
}

// Same basic idea but just keep count of which iteration ends up having the smallest grid height
pub fn part2(input: &str) -> Result<u32, Error> {
    let mut points = parse_input(input)?;

    let mut i = 0;
    let mut last_height = MAX_I32;
    loop {
        let (pmin, pmax) = bounding_box(&points);
        let height = pmax.y - pmin.y;
        if height > last_height {
            break;
        }
        last_height = height;

        step_points(&mut points);
        i += 1;
    }

    Ok(i - 1)
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>$").unwrap();
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn parse_input(input: &str) -> Result<Vec<Point>, Error> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Result<Point, Error> {
    let caps = RE
        .captures(input)
        .ok_or_else(|| format!("Input could not be parsed: {}", input))?;

    Ok(Point {
        x: caps[1].parse()?,
        y: caps[2].parse()?,
        dx: caps[3].parse()?,
        dy: caps[4].parse()?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    const INPUT: &str = r#"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
"#;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(INPUT).unwrap().trim(),
            r#"
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
"#
            .trim()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 3);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-10.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-10.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
