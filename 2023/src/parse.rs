//! Utility functions for parsing

use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

/// Parses each line into any type implementing `FromStr`
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

/// Splits the input based on a Regex, then parses the splits
///
/// ```
/// # use regex::Regex;
/// # use aoc::parse::parse_regex_split;
///
/// let sample = "1, 2, 3\n4, 5, 6";
/// let re = Regex::new(r"[,\n]").unwrap();
/// let result = parse_regex_split::<u32>(sample, &re).unwrap();
/// assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn parse_regex_split<F>(input: &str, re: &Regex) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
{
    re.split(input.trim())
        .map(str::trim)
        .map(str::parse)
        .collect()
}

/// Regex for matching digits
pub static RE_NUMS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

/// Extracts digits from the input string (no parsing)
pub fn extract_digits(input: &str) -> Vec<&str> {
    RE_NUMS.find_iter(input).map(|m| m.as_str()).collect()
}

/// Extracts and parses digits from the input string into any type implementing `FromStr`
pub fn extract_nums<F>(input: &str) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
{
    RE_NUMS
        .find_iter(input)
        .map(|m| m.as_str())
        .map(F::from_str)
        .collect()
}
