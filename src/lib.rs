#[macro_use]
extern crate algorithmia;
#[macro_use]
extern crate serde_derive;

use algorithmia::prelude::*;
use std::error::Error;
mod day0;

#[derive(Deserialize)]
pub struct Input {
    day: u32,
    part: u32,
    input: String,
}

algo_entrypoint!(Input);
fn apply(input: Input) -> Result<String, Box<Error>> {
    match (input.day, input.part) {
        (0, 1) => day0::part1(&input.input),
        (0, 2) => day0::part2(&input.input),
        _ => Err(format!("Puzzle '{}-{}' not supported", input.day, input.part).into()),
    }
}

