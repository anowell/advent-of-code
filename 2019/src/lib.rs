#![feature(pattern)]
#![feature(concat_idents)]
#![cfg_attr(feature = "bench", feature(test))]
#[cfg(feature = "bench")]
extern crate test;

use serde_json::{json, Value};

mod util;

pub struct Input {
    pub day: u32,
    pub part: u32,
    pub input: String,
}

// Macro to avoid manually implementing the wrapper for every puzzle
// For each day specified in handle_days!(), this expects to find a
// dayN.rs file containing part1 and part2 functions
macro_rules! handle_days {
    ($($day:literal),*) => {
        paste::item! {
            $(mod [<day $day>];)*
            pub fn apply(input: Input) -> anyhow::Result<Value> {
                match (input.day, input.part) {
                    $(
                        ($day, 1) => Ok(json!([<day $day>]::part1(&input.input)?)),
                        ($day, 2) => Ok(json!([<day $day>]::part2(&input.input)?)),
                    )*
                    _ => anyhow::bail!("Puzzle '{}-{}' not supported", input.day, input.part),
                }
            }
        }

    }
}

// Simply specify the days that are implemented
handle_days!(1, 2);
