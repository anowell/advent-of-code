use std::{env, fmt::Display, fs};

use anyhow::{bail, Context, Result};

pub mod parse;

// Macro to setup main (to run or benchmark the example)
#[macro_export]
macro_rules! aoc {
    () => {
        #[cfg(not(feature = "bench"))]
        fn main() -> Result<()> {
            aoc::run(part1, part2)
        }

        #[cfg(feature = "bench")]
        fn main() {
            divan::main();
        }
    };
}

// Runner boilerplate
pub fn run<T>(fn1: impl Fn(&str) -> Result<T>, fn2: impl Fn(&str) -> Result<T>) -> Result<()>
where
    T: Display,
{
    let mut args = env::args();
    let _ = args.next();
    let (part1, part2) = match args.next().as_deref() {
        Some("1") => (true, false),
        Some("2") => (false, true),
        Some(val) => bail!("Don't understand arg: {val}"),
        None => (true, true),
    };

    let input = fs::read_to_string("inputs/day1").context("Unable to open input file")?;
    if part1 {
        println!("Part 1: {}", fn1(&input)?);
    }
    if part2 {
        println!("Part 2: {}", fn2(&input)?);
    }
    Ok(())
}

