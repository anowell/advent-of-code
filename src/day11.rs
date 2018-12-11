use matrix::format::conventional::Conventional;
use rayon::prelude::*;
use Error;

// 2018 AoC Day 11 puzzle
// https://adventofcode.com/2018/day/11
const WIDTH: usize = 300;
const HEIGHT: usize = 300;

#[derive(Debug)]
struct SubGridPower {
    x: usize,
    y: usize,
    power: isize,
}

fn power_level(x: isize, y: isize, sn: isize) -> isize {
    let rack_id = x + 10;
    ((y * rack_id + sn) * rack_id) / 100 % 10 - 5
}

fn build_grid(sn: isize) -> Conventional<isize> {
    let mut grid = Conventional::new((WIDTH, HEIGHT));
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let level = power_level(1 + j as isize, 1 + i as isize, sn);
            grid[(j, i)] = level;
        }
    }
    grid
}

fn find_subgrid_power(grid: &Conventional<isize>, square_size: usize) -> SubGridPower {
    let mut i_max = 0;
    let mut j_max = 0;
    let mut val_max = 0;

    for i in 0..(HEIGHT - square_size + 1) {
        for j in 0..(WIDTH - square_size + 1) {
            let mut sum = 0;
            for a in 0..square_size {
                for b in 0..square_size {
                    sum += grid[(j + b, i + a)]
                }
            }
            if sum > val_max {
                val_max = sum;
                i_max = i;
                j_max = j;
            }
        }
    }

    SubGridPower {
        x: j_max + 1,
        y: i_max + 1,
        power: val_max,
    }
}

pub fn part1(input: &str) -> Result<String, Error> {
    let sn: isize = input.trim().parse()?;
    let grid = build_grid(sn);
    let SubGridPower { x, y, .. } = find_subgrid_power(&grid, 3);
    Ok(format!("{},{}", x, y))
}

pub fn part2(input: &str) -> Result<String, Error> {
    let sn: isize = input.trim().parse()?;
    let grid = build_grid(sn);
    let (max_subgrid, idx) = (1..(WIDTH + 1))
        .into_par_iter()
        .map(|i| {
            let subgrid = find_subgrid_power(&grid, i as usize);
            (subgrid, i)
        })
        .max_by(|a, b| a.0.power.cmp(&b.0.power))
        .unwrap();

    Ok(format!("{},{},{}", max_subgrid.x, max_subgrid.y, idx))
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("18").unwrap(), "33,45");
        assert_eq!(part1("42").unwrap(), "21,61");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("18").unwrap(), "90,269,16");
        assert_eq!(part2("42").unwrap(), "232,251,12");
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-11.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-11.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }

}
