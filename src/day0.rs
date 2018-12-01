use Error;

// This is the 2017 AoC Day 1 puzzle

static ASCII_NUM_OFFSET: u8 = 48;

pub fn part1(input: &str) -> Result<u64, Error> {
    let bytes = input.as_bytes();
    let total = sum_matching_digits(bytes, 1);
    Ok(total)
}

pub fn part2(input: &str) -> Result<u64, Error> {
    let bytes = input.as_bytes();
    let total = sum_matching_digits(bytes, bytes.len() / 2);
    Ok(total)
}

fn sum_matching_digits(ascii: &[u8], offset: usize) -> u64 {
    let len = ascii.len();
    let mut total: u64 = 0;
    for i in 0..len {
        if ascii[i] == ascii[(i + offset) % len] {
            total += (ascii[i] - ASCII_NUM_OFFSET) as u64;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1122").unwrap(), 3);
        assert_eq!(part1("1111").unwrap(), 4);
        assert_eq!(part1("91212129").unwrap(), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1212").unwrap(), 6);
        assert_eq!(part2("1221").unwrap(), 0);
        assert_eq!(part2("123425").unwrap(), 4);
        assert_eq!(part2("123123").unwrap(), 12);
        assert_eq!(part2("12131415").unwrap(), 4);
    }
}
