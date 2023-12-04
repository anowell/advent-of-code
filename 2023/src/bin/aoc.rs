use anyhow::{format_err, Context, Result, bail};
use std::fs;

fn main() -> Result<()> {
    let mut args = ::std::env::args().skip(1);
    if args.len() < 1 {
        bail!("USAGE: aoc DAY[-PART] [FILE]");
    }

    let puzzle = args
        .next()
        .ok_or_else(|| format_err!("Must specifiy a puzzle (e.g. 2-1 for day 2 part 1)"))?;
    let mut split = puzzle.split('-');

    let day: u32 = split
        .next()
        .unwrap()
        .parse()
        .context("Failed to parse a day number from the puzzle")?;
    let part: u32 = split
        .next()
        .unwrap_or("1")
        .parse()
        .expect("Failed to parse a part number from the puzzle");

    let file_path = args.next().unwrap_or_else(|| format!("inputs/day{}", day));
    let input = fs::read_to_string(file_path).context("Unable to open input file")?;
    let output = aoc::apply(aoc::Input { day, part, input })?;

    println!("{}", output);
    Ok(())
}
