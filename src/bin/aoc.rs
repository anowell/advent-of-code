extern crate algorithm;
extern crate algorithmia;
#[macro_use]
extern crate serde_json;

use algorithmia::algo::{AlgoInput, AlgoOutput, EntryPoint};
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::Read;

macro_rules! bail {
    ($e:expr) => {{
        eprintln!($e);
        ::std::process::exit(1)
    }};
    ($fmt:expr, $($arg:tt)+) => {{
        eprintln!($fmt, $($arg)+);
        ::std::process::exit(1)
    }};
}

fn main() {
    let algo = algorithm::Algo::default();
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
        .expect("Failed to parse a part number from the puzzle");;

    let file_path = args
        .next()
        .unwrap_or_else(|| format!("inputs/day-{}.txt", day));
    let mut input = String::new();
    let mut file =
        File::open(&file_path).unwrap_or_else(|e| bail!("Failed to open puzzle input: {}", e));
    file.read_to_string(&mut input)
        .expect("Failed reading file");

    let output = algo.apply(AlgoInput::Json(Cow::Owned(json!({
        "day": day,
        "part": part,
        "input": input.trim(),
    }))));
    match output {
        Ok(AlgoOutput::Json(res)) => println!("{}", res),
        Ok(_) => unimplemented!(),
        Err(err) => print_cause_chain(&*err),
    }
}

fn print_cause_chain(err: &Error) {
    let mut causes = vec![err.to_string()];
    let mut e = err;
    while let Some(cause) = e.cause() {
        causes.push(cause.to_string());
        e = cause;
    }
    eprintln!("Error: {}", causes.join("\ncaused by: "));
}
