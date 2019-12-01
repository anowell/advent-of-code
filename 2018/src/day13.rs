use matrix::format::conventional::Conventional;
use matrix::Element;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Add;
use crate::Error;

// 2018 AoC Day 13 puzzle
// https://adventofcode.com/2018/day/13

// turns out the pattern starts repeating just before the 100th generation
pub fn part1(input: &str) -> Result<String, Error> {
    let mut map = parse_input(input);
    loop {
        if let Some(coord) = map.tick() {
            return Ok(format!("{},{}", coord.x, coord.y));
        }
    }
}

pub fn part2(input: &str) -> Result<String, Error> {
    let mut map = parse_input(input);
    // print(&map);
    println!("{} cars", map.carts.len());
    let mut prev;
    for _i in 0..1_000_000 {
        prev = map.clone();
        let crashes = map.tick_remove_crashes();
        // print(&map);
        if crashes > 0 {
            println!("Removed {} cars. {} remaining", crashes, map.carts.len());
        }
        if map.carts.len() <= 1 {
            let coord = map.carts[0].coord;
            println!("{:?}", prev.carts);
            println!("{:?}", map.carts[0]);
            map.tick();
            println!("{:?}", map.carts[0]);
            return Ok(format!("{},{}", coord.x, coord.y));
        }
    }
    unreachable!();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    None,
    NorthSouth,
    EastWest,
    CurveSwNe,
    CurveNwSe,
    Intersection,
}

impl Element for Cell {
    fn is_zero(&self) -> bool {
        *self == Cell::None
    }
    fn zero() -> Self {
        Cell::None
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: Direction) -> Coordinate {
        let (dx, dy): (i32, i32) = match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        Coordinate {
            x: (self.x as i32 + dx) as u32,
            y: (self.y as i32 + dy) as u32,
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        ((self.y as u64) << 32 + self.x as u64).cmp(&((other.y as u64) << 32 + other.x as u64))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cart {
    direction: Direction,
    coord: Coordinate,
    next_intersection: Turn,
    crashed: bool,
}

impl Cart {
    fn new(x: u32, y: u32, direction: Direction) -> Cart {
        Cart {
            direction,
            coord: Coordinate { x, y },
            next_intersection: Turn::Left,
            crashed: false,
        }
    }

    fn update_heading(&mut self, cell_path: Cell) {
        match cell_path {
            Cell::NorthSouth | Cell::EastWest => {}

            // Path:  \
            Cell::CurveNwSe => {
                self.direction = match self.direction {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                }
            }

            // Path: /
            Cell::CurveSwNe => {
                self.direction = match self.direction {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                }
            }

            // Path: +
            Cell::Intersection => {
                self.direction = match self.next_intersection {
                    Turn::Left => self.direction.turn_left(),
                    Turn::Straight => self.direction,
                    Turn::Right => self.direction.turn_right(),
                };
                self.next_intersection = match self.next_intersection {
                    Turn::Left => Turn::Straight,
                    Turn::Straight => Turn::Right,
                    Turn::Right => Turn::Left,
                };
            }
            Cell::None => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    grid: Conventional<Cell>,
    carts: Vec<Cart>,
    cart_lookup: HashSet<Coordinate>,
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        Map {
            grid: Conventional::new((width, height)),
            carts: Vec::new(),
            cart_lookup: HashSet::new(),
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.grid[(x, y)] = cell;
    }

    fn add_cart(&mut self, x: usize, y: usize, direction: Direction) {
        let cart = Cart::new(x as u32, y as u32, direction);
        self.cart_lookup.insert(cart.coord);
        self.carts.push(cart);
    }

    // fn remove_carts(&mut self, x: usize, y: usize) {
    //     let coord = Coordinate { x: x as u32, y: y as u32 };
    //     self.cart_lookup.remove(&coord);
    //     self.carts = self.carts.iter().filter(|c| c.coord != coord).map(|c| *c).collect();
    // }
}

impl Map {
    fn tick(&mut self) -> Option<Coordinate> {
        // sort to make sure we move them in the right order
        self.carts.sort_by(|a, b| a.coord.cmp(&b.coord));
        for cart in &mut self.carts {
            // vacate the current coord and move to the new coord
            self.cart_lookup.remove(&cart.coord);
            cart.coord = cart.coord + cart.direction;

            // Insert the carts new coord into the lookup set, if this fails, then a collision occured
            if !self.cart_lookup.insert(cart.coord) {
                cart.crashed = true;
                return Some(cart.coord);
            }

            // Update the cart's direction based on the cell's path and previous driving turning history
            let cell_path = self.grid[cart.coord.tuple()];
            cart.update_heading(cell_path);
        }
        None
    }

    // number of cars removed
    fn tick_remove_crashes(&mut self) -> usize {
        let mut crashes = HashSet::new();

        // sort to make sure we move them in the right order
        self.carts.sort_by(|a, b| a.coord.cmp(&b.coord));
        for (_i, cart) in self.carts.iter_mut().enumerate() {
            if crashes.contains(&cart.coord) {
                continue;
            }

            // vacate the current coord and move to the new coord
            self.cart_lookup.remove(&cart.coord);
            cart.coord = cart.coord + cart.direction;

            // Insert the carts new coord into the lookup set, if this fails, then a collision occured
            if self.cart_lookup.contains(&cart.coord) {
                crashes.insert(cart.coord);
                self.cart_lookup.remove(&cart.coord);
            } else {
                self.cart_lookup.insert(cart.coord);
            }

            // Update the cart's direction based on the cell's path and previous driving turning history
            let cell_path = self.grid[cart.coord.tuple()];
            cart.update_heading(cell_path);
        }

        if crashes.len() > 0 {
            println!("CRASHES: {:?}", crashes);
            println!(
                "Removing: cars {:?}",
                self.carts
                    .iter()
                    .filter(|c| crashes.contains(&c.coord))
                    .map(|c| *c)
                    .collect::<Vec<_>>()
            );
        }
        self.carts = self
            .carts
            .iter()
            .filter(|c| !crashes.contains(&c.coord))
            .map(|c| *c)
            .collect();

        crashes.len() * 2
    }
}

fn parse_input(input: &str) -> Map {
    let input = input.trim_matches('\n');
    let width = input.lines().map(|l| l.len()).max().unwrap();
    let height = input.lines().count();
    let mut map = Map::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            let cell = match c {
                b'-' | b'<' | b'>' => Cell::EastWest,
                b'|' | b'^' | b'v' => Cell::NorthSouth,
                b'\\' => Cell::CurveNwSe,
                b'/' => Cell::CurveSwNe,
                b'+' => Cell::Intersection,
                b' ' => Cell::None,
                _ => unimplemented!("Unexpected symbol in map"),
            };
            map.set_cell(x, y, cell);
            match c {
                b'<' => map.add_cart(x, y, Direction::West),
                b'v' => map.add_cart(x, y, Direction::South),
                b'>' => map.add_cart(x, y, Direction::East),
                b'^' => map.add_cart(x, y, Direction::North),
                _ => {}
            }
        }
    }

    map
}

// fn print(map: &Map) {
//     for y in 0..(map.grid.rows) {
//         for x in 0..(map.grid.columns) {
//             let coord = Coordinate {
//                 x: x as u32,
//                 y: y as u32,
//             };
//             if map.cart_lookup.contains(&coord) {
//                 let cart = map.carts.iter().find(|&&c| c.coord == coord).unwrap();
//                 print!(
//                     "{}",
//                     match cart.direction {
//                         Direction::East => '>',
//                         Direction::West => '<',
//                         Direction::North => '^',
//                         Direction::South => 'v',
//                     }
//                 )
//             } else {
//                 print!(
//                     "{}",
//                     match map.grid[coord.tuple()] {
//                         Cell::EastWest => '-',
//                         Cell::NorthSouth => '|',
//                         Cell::CurveNwSe => '\\',
//                         Cell::CurveSwNe => '/',
//                         Cell::Intersection => '+',
//                         Cell::None => ' ',
//                     }
//                 )
//             }
//         }
//         println!("");
//     }
//     println!("")
// }

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        let input = r#"
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
"#;
        assert_eq!(part1(input).unwrap(), "7,3");
    }

    #[test]
    fn test_part2() {
        let input = r#"
/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"#;

        assert_eq!(part2(input).unwrap(), "6,4");
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-13.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-13.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
