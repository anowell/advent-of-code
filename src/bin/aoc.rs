extern crate algorithm;
extern crate algorithmia;
#[macro_use]
extern crate serde_json;

use algorithmia::algo::{AlgoInput, AlgoOutput, EntryPoint};
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() {
    let algo = algorithm::Algo::default();
    let mut args = ::std::env::args();
    let _ = args.next();
    let day = args.next().expect("Must specify a day number");
    let part = args.next().expect("Must specify a part number");

    let mut input = String::new();
    let mut file =
        File::open(&format!("inputs/day-{}.txt", day)).expect("Failed to open puzzle input");
    file.read_to_string(&mut input)
        .expect("Failed reading file");

    let output = algo.apply(AlgoInput::Json(Cow::Owned(json!({
        "day": str::parse::<u32>(&day).unwrap(),
        "part": str::parse::<u32>(&part).unwrap(),
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
