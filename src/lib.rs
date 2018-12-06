#![cfg_attr(feature = "bench", feature(test))]

#[cfg(feature = "bench")]
extern crate test;

#[macro_use]
extern crate algorithmia;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate fxhash;
extern crate linked_list;
extern crate matrix;
extern crate rayon;
extern crate regex;

use algorithmia::prelude::*;

type Error = Box<std::error::Error>;

mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(Deserialize)]
pub struct Input {
    day: u32,
    part: u32,
    input: String,
}

algo_entrypoint!(Input);
fn apply(input: Input) -> Result<Value, Error> {
    let Input { day, part, input } = input;
    match (day, part) {
        (0, 1) => Ok(json!(day0::part1(&input)?)),
        (0, 2) => Ok(json!(day0::part2(&input)?)),
        (1, 1) => Ok(json!(day1::part1(&input)?)),
        (1, 2) => Ok(json!(day1::part2(&input)?)),
        (2, 1) => Ok(json!(day2::part1(&input)?)),
        (2, 2) => Ok(json!(day2::part2(&input)?)),
        (3, 1) => Ok(json!(day3::part1(&input)?)),
        (3, 2) => Ok(json!(day3::part2(&input)?)),
        (4, 1) => Ok(json!(day4::part1(&input)?)),
        (4, 2) => Ok(json!(day4::part2(&input)?)),
        (5, 1) => Ok(json!(day5::part1(&input)?)),
        (5, 2) => Ok(json!(day5::part2(&input)?)),
        (6, 1) => Ok(json!(day6::part1(&input)?)),
        (6, 2) => Ok(json!(day6::part2(&input, 10_000)?)),
        _ => {
            return Err(format!("Puzzle '{}-{}' not supported", day, part).into());
        }
    }
}
