## Overview

Advent of Code 2018 solutions in Rust that are [live](https://algorithmia.com/algorithms/anowell/RustyAoC2018) on the Algorithmia API.

[![](https://algorithmia.com/algorithms/anowell/RustyAoC2018/badge)](https://algorithmia.com/algorithms/anowell/RustyAoC2018)

## Usage

### Input

| Parameter | Description |
| --------- | ----------- |
| day     | Day number of the Advent of Code puzzle |
| part    | 1 or 2. Each AoC puzzle has 2 parts |
| input   | A string containing the entire puzzle input |


### Output

Outputs a JSON serialization of the solutions output, generally a number or a string.

## Examples

Running the 2018 Day 1 puzzle:

```json
{
  "day": 1,
  "part": 2,
  "input": "+7, +7, -2, -7, -4"
}
```

## Running Locally

Build it tested on both 1.22.1 and nightly rust, though you will need nightly to run benchmarks.

```
cargo build
cargo test
cargo test day4
```

Benchmarks and the `aoc` binary it use inputs from `inputs/day-{num}.txt`.

```
# Run all the benchmarks
cargo bench

# Run benchmarks for a specific day
cargo bench day4

cargo build --release
target/release/aoc 4-1
target/release/aoc 4-1 inputs/day-4.txt
```
