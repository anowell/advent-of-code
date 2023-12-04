pub mod parse;
use anyhow::Context;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
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
            $(pub mod [<day $day>];)*
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
handle_days![1, 2, 3, 4];


pub fn input(fname: &str) -> anyhow::Result<String> {
    let path = format!("inputs/{fname}");
    std::fs::read_to_string(path).context("Unable to open input file")
}
