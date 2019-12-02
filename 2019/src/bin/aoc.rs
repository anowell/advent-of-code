use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let mut args = ::std::env::args();
    let _ = args.next();

    let puzzle = args
        .next()
        .expect("Must specifiy a puzzle (e.g. 2-1 for day 2 part 1)");
    let mut split = puzzle.split('-');
    let day: u32 = split
        .next()
        .unwrap()
        .parse()
        .expect("Failed to parse a day number from the puzzle");
    let part: u32 = split
        .next()
        .unwrap_or("1")
        .parse()
        .expect("Failed to parse a part number from the puzzle");

    let file_path = args
        .next()
        .unwrap_or_else(|| format!("inputs/day-{}.txt", day));
    let mut input = String::new();
    let mut file = File::open(&file_path)
        .with_context(|| format!("Failed to open puzzle input: {}", file_path))?;
    file.read_to_string(&mut input)
        .expect("Failed reading file");

    let output = aoc::apply(aoc::Input {
        day,
        part,
        input: input.trim().to_owned(),
    })?;

    println!("{}", output);
    Ok(())
}
