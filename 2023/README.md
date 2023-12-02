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
just watch 1
just test 1
just fetch 1
just bench 1

# day1 part1
just run 1 1

# day1 part 2
just run 1 2
```
