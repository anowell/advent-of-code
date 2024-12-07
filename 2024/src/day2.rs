//! [Advent of Code Day 2](https://adventofcode.com/2024/day/2)

use crate::parse;
use crate::prelude::*;

pub fn part1(input: &str) -> Result<u32> {
    let lines = input.trim().lines();
    let reports: Vec<Vec<i32>> = lines.map(parse::extract_nums).try_collect()?;
    let safe_count = reports.into_iter().filter(|report| is_safe(report)).count();
    Ok(safe_count as u32)
}

pub fn part2(input: &str) -> Result<u32> {
    let lines = input.trim().lines();
    let reports: Vec<Vec<i32>> = lines.map(parse::extract_nums).try_collect()?;
    let safe_count = reports
        .into_iter()
        .filter(|report| is_safe_ecc(report))
        .count();
    Ok(safe_count as u32)
}

fn is_decreasing(levels: &[i32]) -> bool {
    levels.iter().tuple_windows().all(|(a, b)| a > b)
}

fn is_increasing(levels: &[i32]) -> bool {
    levels.iter().tuple_windows().all(|(a, b)| a < b)
}

fn is_gradual(levels: &[i32]) -> bool {
    levels
        .iter()
        .tuple_windows()
        .all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3)
}

fn is_safe(levels: &[i32]) -> bool {
    is_gradual(levels) && (is_increasing(levels) || is_decreasing(levels))
}

/// ECC - a sort of Error Checking Code that can identify the bad level in the report
///
/// We could have implemented this by iterating over every possible report without one level, but chose a more mathematical approach
///
/// Instead of analyzing the level values, we analyze the deltas between subsequent level numbers.
/// - If all the deltas are between 1 and 3, we know it's gradually increasing.
/// - If all the deltas are between -1 and -3, we know it's gradually decreasing.
/// - When we have deltas out of that range, we determine if the delta can be applied to a previous or next delta (which would locate a removable bad level)
fn is_safe_ecc(levels: &[i32]) -> bool {
    let deltas = levels
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    // Determine if this is an increasing or decreasing report
    let pos_count = deltas.iter().filter(|n| **n > 0).count();
    let neg_count = deltas.iter().filter(|n| **n < 0).count();
    let good_range = match pos_count > neg_count {
        true => 1..=3,
        false => -3..=-1,
    };

    let good = |d: i32| good_range.contains(&d);
    let bad = |d: i32| !good(d);

    // Count bad entries in the report
    let bad_delta_count = deltas.iter().filter(|n| bad(**n)).count();
    match bad_delta_count {
        // Every delta is good, so the report is safe
        0 => return true,
        // 1 bad delta. It needs to be able to carry to the previous or next delta for the report to be safe
        1 => {
            // unless it's the first or last delta, in which case the first or last level can be removed
            if bad(*deltas.first().unwrap()) || bad(*deltas.last().unwrap()) {
                return true;
            }
            for (da, db) in deltas.iter().tuple_windows() {
                if (bad(*da) || bad(*db)) && good(da + db) {
                    return true;
                }
            }
            return false;
        }
        // 2 bad deltas. They must be consecutive and sum to a good delta for the report to be safe
        2 => {
            for (da, db) in deltas.into_iter().tuple_windows() {
                if bad(da) && bad(db) {
                    return good(da + db);
                }
            }
            return false;
        }
        // Too many deltas are bad for the report to be safe
        _ => return false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn test_helpers() {
        assert_eq!(true, is_increasing(&[1, 2, 3, 4, 5]));
        assert_eq!(false, is_increasing(&[5, 4, 3, 2, 1]));
        assert_eq!(false, is_decreasing(&[1, 2, 3, 4, 5]));
        assert_eq!(true, is_decreasing(&[5, 4, 3, 2, 1]));
        assert_eq!(true, is_gradual(&[1, 2, 4, 7, 9]));
        assert_eq!(true, is_gradual(&[9, 7, 4, 2, 1]));
        assert_eq!(false, is_gradual(&[9, 4, 3, 2, 1]));
        assert_eq!(false, is_gradual(&[1, 1]));
        assert_eq!(true, is_gradual(&[1, 2]));
        assert_eq!(true, is_gradual(&[1, 4]));
        assert_eq!(false, is_gradual(&[1, 5]));
    }

    #[test]
    fn test_is_safe() {
        // 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
        assert!(is_safe(&[7, 6, 4, 2, 1]));
        // 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
        assert!(!is_safe(&[1, 2, 7, 8, 9]));
        // 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
        assert!(!is_safe(&[9, 7, 6, 2, 1]));
        // 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
        assert!(!is_safe(&[1, 3, 2, 4, 5]));
        // 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
        assert!(!is_safe(&[8, 6, 4, 4, 1]));
        // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
        assert!(is_safe(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_is_safe_ecc() {
        // 7 6 4 2 1: Safe without removing any level.
        assert!(is_safe_ecc(&[7, 6, 4, 2, 1]));
        // 1 2 7 8 9: Unsafe regardless of which level is removed.
        assert!(!is_safe_ecc(&[1, 2, 7, 8, 9]));
        // 9 7 6 2 1: Unsafe regardless of which level is removed.
        assert!(!is_safe_ecc(&[9, 7, 6, 2, 1]));
        // 1 3 2 4 5: Safe by removing the second level, 3.
        assert!(is_safe_ecc(&[1, 3, 2, 4, 5]));
        // 8 6 4 4 1: Safe by removing the third level, 4.
        assert!(is_safe_ecc(&[8, 6, 4, 4, 1]));
        // 1 3 6 7 9: Safe without removing any level.
        assert!(is_safe_ecc(&[1, 3, 6, 7, 9]));

        // 1,3,2,4
        assert!(is_safe_ecc(&[1, 3, 2, 4]));
        // 2,3,1,4
        assert!(is_safe_ecc(&[2, 3, 1, 4]));

        // failing in original implementation
        assert!(is_safe_ecc(&[63, 61, 62, 61, 60]));

        // failing in v2 implementation
        assert!(is_safe_ecc(&[11, 12, 15, 18, 19, 18]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 4);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(2);
}
