Advent of Code 2023
=================

### Setup

Install rust and [just](https://github.com/casey/just).

Login to AoC and retrieve your session cookie. Add it to `.env` as.
This allows us to fetch puzzle input.

```
SESSION="..."
```

Run tests, benchmark, fetch input, run puzzle:

```
# Scaffold new example file for Day 2
just scaffold 2

# Watch: recompile & retest Day 2 solution after every change
just watch 2

# Run tests for Day 2
just test 2

# Download puzzle input for day 2 (requires setting SESSION)
just fetch 2

# Run benchmarks for Day 2
just bench 2

# Run Day 2, Part 1 using the downloaded input
just run 2 1

# Run Day 2, Part 2 using the downloaded input
just run 1 2
```
