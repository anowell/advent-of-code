#![cfg_attr(feature = "bench", feature(test))]

#[cfg(feature = "bench")]
extern crate test;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use serde_json::Value;

type Error = Box<dyn std::error::Error>;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Deserialize)]
pub struct Input {
    pub day: u32,
    pub part: u32,
    pub input: String,
}

pub fn apply(input: Input) -> Result<Value, Error> {
    let Input {
        day,
        part,
        input,
    } = input;
    match (day, part) {
        (01, 1) => Ok(json!(day1::part1(&input)?)),
        (01, 2) => Ok(json!(day1::part2(&input)?)),
        (02, 1) => Ok(json!(day2::part1(&input)?)),
        (02, 2) => Ok(json!(day2::part2(&input)?)),
        (03, 1) => Ok(json!(day3::part1(&input)?)),
        (03, 2) => Ok(json!(day3::part2(&input)?)),
        (04, 1) => Ok(json!(day4::part1(&input)?)),
        (04, 2) => Ok(json!(day4::part2(&input)?)),
        (05, 1) => Ok(json!(day5::part1(&input)?)),
        (05, 2) => Ok(json!(day5::part2(&input)?)),
        (06, 1) => Ok(json!(day6::part1(&input)?)),
        (06, 2) => Ok(json!(day6::part2(&input, 10_000)?)),
        (07, 1) => Ok(json!(day7::part1(&input)?)),
        (07, 2) => Ok(json!(day7::part2(&input, 5, 60)?)),
        (08, 1) => Ok(json!(day8::part1(&input)?)),
        (08, 2) => Ok(json!(day8::part2(&input)?)),
        (09, 1) => Ok(json!(day9::part1(&input)?)),
        (09, 2) => Ok(json!(day9::part2(&input)?)),
        (10, 1) => Ok(json!(day10::part1(&input)?)),
        (10, 2) => Ok(json!(day10::part2(&input)?)),
        (11, 1) => Ok(json!(day11::part1(&input)?)),
        (11, 2) => Ok(json!(day11::part2(&input)?)),
        (12, 1) => Ok(json!(day12::part1(&input)?)),
        (12, 2) => Ok(json!(day12::part2(&input)?)),
        (13, 1) => Ok(json!(day13::part1(&input)?)),
        (13, 2) => Ok(json!(day13::part2(&input)?)),
        _ => {
            return Err(format!("Puzzle '{}-{}' not supported", day, part).into());
        }
    }
}
