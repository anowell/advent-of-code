use regex::Regex;
use std::cmp::{max, min};
use std::u32::MAX as MAX_U32;
use Error;

// 2018 AoC Day 6 puzzle
// https://adventofcode.com/2018/day/6

fn manhattan_distance(a: Point, b: Point) -> u32 {
    max(a.x, b.x) - min(a.x, b.x) + max(a.y, b.y) - min(a.y, b.y)
}

// Finds the min and max points that serve as the bounding box that contains all other points
fn bounding_box(points: &[Point]) -> (Point, Point) {
    let mut iter = points.iter();
    let p = iter.next().unwrap();
    let mut pmin = *p;
    let mut pmax = *p;
    for p in iter {
        pmin.x = min(pmin.x, p.x);
        pmax.x = max(pmax.x, p.x);
        pmin.y = min(pmin.y, p.y);
        pmax.y = max(pmax.y, p.y);
    }
    (pmin, pmax)
}

pub fn part1(input: &str) -> Result<u32, Error> {
    let points = parse_input(input)?;
    let (pmin, pmax) = bounding_box(&points);

    let mut regions = vec![0; points.len()];

    for y in (pmin.y)..(pmax.y + 1) {
        for x in (pmin.x)..(pmax.x + 1) {
            let mut min_index = None;
            let mut min_distance = 0;
            let grid_point = Point { x, y };
            for (i, p) in points.iter().enumerate() {
                let distance = manhattan_distance(*p, grid_point);
                if i == 0 || distance < min_distance {
                    min_index = Some(i);
                    min_distance = distance;
                } else if distance == min_distance {
                    min_index = None;
                }
            }
            // print!("{}, ", min_index);
            if let Some(index) = min_index {
                if x == pmin.x || y == pmin.y || x == pmax.x || y == pmax.y {
                    regions[index] = MAX_U32;
                } else if regions[index] < MAX_U32 {
                    regions[index] += 1;
                }
            }
        }
        // println!("");
    }

    // println!("{:?}", regions);
    let largest = regions
        .into_iter()
        .filter(|x| *x != MAX_U32)
        .max()
        .expect("no max found");

    Ok(largest)
}

pub fn part2(input: &str, total_distance_bound: u32) -> Result<u32, Error> {
    let points = parse_input(input)?;
    let (pmin, pmax) = bounding_box(&points);

    let mut region_size = 0;
    for y in (pmin.y)..(pmax.y + 1) {
        for x in (pmin.x)..(pmax.x + 1) {
            // Add up all the distances
            let grid_point = Point { x, y };
            let total_distance: u32 = points
                .iter()
                .map(|p| manhattan_distance(*p, grid_point))
                .sum();

            if total_distance < total_distance_bound {
                region_size += 1;
            }
        }
    }

    Ok(region_size)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+), (\d+)$").unwrap();
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

fn parse_input(input: &str) -> Result<Vec<Point>, Error> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Result<Point, Error> {
    let caps = RE
        .captures(input)
        .ok_or_else(|| format!("Claim could not be parsed: {}", input))?;

    Ok(Point {
        x: caps[1].parse()?,
        y: caps[2].parse()?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    const INPUT: &str = r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT, 32).unwrap(), 16);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-6.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-6.txt").expect("Unable to open file");
        b.iter(|| part2(&input, 10000).unwrap());
    }

}
