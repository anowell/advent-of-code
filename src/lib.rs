#[macro_use]
extern crate algorithmia;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use algorithmia::prelude::*;

type Error = Box<std::error::Error>;

mod day0;
mod day1;

#[derive(Deserialize)]
pub struct Input {
    day: u32,
    part: u32,
    input: String,
}

algo_entrypoint!(Input);
fn apply(input: Input) -> Result<Value, Error> {
    let Input{ day, part, input } = input;
    match (day, part) {
        (0, 1) => Ok(json!(day0::part1(&input)?)),
        (0, 2) => Ok(json!(day0::part2(&input)?)),
        (1, 1) => Ok(json!(day1::part1(&input)?)),
        (1, 2) => Ok(json!(day1::part2(&input)?)),
        _ => {
            return Err(format!("Puzzle '{}-{}' not supported", day, part).into());
        }
    }
}

