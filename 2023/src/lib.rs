use std::{env, fmt::Display, fs};

use anyhow::{bail, Context, Result};

pub mod parse;

// Macro to setup main (to run or benchmark the example)
#[macro_export]
macro_rules! setup {
    ($input:literal) => {
        #[cfg(not(feature = "bench"))]
        fn main() -> Result<()> {
            aoc::run($input, part1, part2)
        }

        #[cfg(feature = "bench")]
        fn main() {
            divan::main();
        }
    };
}

pub fn input(fname: &str) -> Result<String> {
    let path = format!("inputs/{fname}");
    fs::read_to_string(path).context("Unable to open input file")
}

// Runner boilerplate
pub fn run<T>(
    fname: &str,
    fn1: impl Fn(&str) -> Result<T>,
    fn2: impl Fn(&str) -> Result<T>,
) -> Result<()>
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

    let input = input(fname)?;
    if part1 {
        println!("Part 1: {}", fn1(&input)?);
    }
    if part2 {
        println!("Part 2: {}", fn2(&input)?);
    }
    Ok(())
}
