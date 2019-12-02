use std::str::pattern::Pattern;
use std::str::FromStr;

pub fn parse_lines<F>(input: &str) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
{
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect()
}

// split_parse("foo, bar", |c| c == '.' || c == '\n')
pub fn split_parse<'a, F, P>(input: &'a str, pattern: P) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
    P: Pattern<'a>,
{
    input
        .trim()
        .split(pattern)
        .map(str::trim)
        .map(str::parse)
        .collect()
}
