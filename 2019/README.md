## Overview

Advent of Code 2019 solutions in Rust.

## Usage

```
cargo build --release
target/release/aoc <day>-<puzzle> [input_file]
```

### Output

Outputs a JSON serialization of the solutions output, generally a number or a string.

## Developing

`just` is used to simplify several workflows

```
# Run a puzzle (e.g. day 2, part 1)
$ just run 2-1

# Run tests (e.g. day 2)
$ just test 2

# Run benchmarks (e.g. day 2)
$ just bench 2

# Watch for changes and rerun tests (e.g. day 2)
$ just tdd 2

# Fetch the puzzle input (e.g. day 2)
$ just fetch 2

# Generate a template daily template (e.g. day2.rs for day 2)
$ just template 2
```

