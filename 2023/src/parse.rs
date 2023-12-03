use std::str::FromStr;

use regex::Regex;

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

// split_parse("foo, bar; baz", Regex::new(r"[,;\n]").unwrap())
pub fn parse_regex_split<F>(input: &str, re: &Regex) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
{
    re.split(input.trim())
        .map(str::trim)
        .map(str::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_split_parse() {
        let sample = "1, 2, 3\n4, 5, 6";
        let re = Regex::new(r"[,\n]").unwrap();
        let result = parse_regex_split::<u32>(sample, &re).unwrap();
        assert_eq!(result, vec![1,2,3,4,5,6]);
    }
}