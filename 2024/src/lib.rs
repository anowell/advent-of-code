use anyhow::Context;
use serde_json::{json, Value};

pub mod math;
pub mod parse;

#[allow(unused)]
pub(crate) mod prelude {
    pub use anyhow::{bail, format_err, Chain, Context, Error, Result};
    pub use derive_deref::Deref;
    pub use itertools::Itertools;
    pub use once_cell::sync::Lazy;
    pub use regex::Regex;
    pub use std::cmp::{self, Ordering};
    pub use std::str::FromStr;
    pub use winnow::prelude::*;
}

#[derive(Debug, Clone)]
/// Specifies the puzzle and puzzle input to run
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

            /// Wrapper function that calls a particular puzzle and prints the result
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

#[cfg(feature = "bench")]
macro_rules! bench_day {
    ($day:literal) => {
        use super::*;

        #[divan::bench(max_time = std::time::Duration::from_secs(10))]
        fn bench_part1(bencher: divan::Bencher) {
            let input = crate::input(concat!("day", $day)).unwrap();
            bencher.bench(|| part1(&input).unwrap());
        }
        #[divan::bench(max_time = std::time::Duration::from_secs(10))]
        fn bench_part2(bencher: divan::Bencher) {
            let input = crate::input(concat!("day", $day)).unwrap();
            bencher.bench(|| part2(&input).unwrap());
        }
    };
}

// Simply specify the days that are implemented
handle_days![1, 2, 3, 4, 5, 6, 7, 8, 11];

/// Helper to read a given input file into a string
pub fn input(fname: &str) -> anyhow::Result<String> {
    let path = format!("inputs/{fname}");
    std::fs::read_to_string(path).context("Unable to open input file")
}
