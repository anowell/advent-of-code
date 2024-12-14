//! [Advent of Code Day 9](https://adventofcode.com/2024/day/9)

use crate::prelude::*;

pub fn part1(input: &str) -> Result<usize> {
    let fs = Filesystem::from_str(input).unwrap();
    Ok(fs.block_compacted_checksum())
}

pub fn part2(input: &str) -> Result<usize> {
    let fs = Filesystem::from_str(input).unwrap();
    Ok(fs.file_compacted_checksum())
}

#[derive(Debug, Clone, Deref)]
struct Filesystem(Vec<Record>);

impl Filesystem {
    fn files(&self) -> Vec<File> {
        let mut files = Vec::new();
        let mut offset = 0;
        for (id, record) in self.0.iter().enumerate() {
            if let Record::Used(size) = *record {
                if size > 0 {
                    files.push(File { id, offset, size });
                } else {
                    panic!("Found a file with no length?");
                }
            }
            offset += record.blocks()
        }
        files
    }

    fn free_ranges(&self) -> Vec<Range> {
        let mut ranges = Vec::new();
        let mut offset = 0;
        for record in &self.0 {
            if let Record::Free(size) = *record {
                if size > 0 {
                    ranges.push(Range { offset, size });
                }
            }
            offset += record.blocks()
        }
        ranges
    }

    fn block_compacted_checksum(&self) -> usize {
        let mut checksum = 0;
        let mut free = self.free_ranges().into_iter();
        let mut free_range = free.next().expect("No free space available");
        // eprintln!("FREE: {} blocks @{}", free_range.size, free_range.offset);

        let mut free_range_used = 0;
        for (id, file) in self.files().iter().enumerate().rev() {
            // eprintln!("FILE {id}: {} blocks @{}", file.size, file.offset);

            for file_block in (0..file.size).rev() {
                if file.offset + file_block < free_range.offset + free_range_used {
                    let placement = file.offset + file_block;
                    // eprintln!("  LEAVE: {id} at {placement}");
                    checksum += id * placement;
                } else {
                    let placement = free_range.offset + free_range_used;
                    // eprintln!("  MOVE : {id} to {placement}");
                    checksum += id * placement;
                    free_range_used += 1;

                    if free_range_used == free_range.size {
                        free_range = free.next().expect("Compaction exceeds disk space");
                        // eprintln!("FREE: {} blocks @{}", free_range.size, free_range.offset);
                        free_range_used = 0;
                    }
                }
            }
        }
        checksum
    }

    fn file_compacted_checksum(&self) -> usize {
        let mut checksum = 0;
        let mut free = self.free_ranges();

        for (id, file) in self.files().iter().enumerate().rev() {
            // eprintln!("FILE {id}: {} blocks @{}", file.size, file.offset);
            let mut placement = file.offset;
            for free_range in free.iter_mut() {
                if file.offset < free_range.offset {
                    // eprintln!("  LEAVE: {id} at {placement}");
                    break;
                }
                if free_range.size >= file.size {
                    placement = free_range.offset;
                    // eprintln!("  MOVE : {id} to {placement}");
                    free_range.offset += file.size;
                    free_range.size -= file.size;
                    break;
                }
            }
            for file_block in 0..file.size {
                checksum += id * (placement + file_block);
            }
        }
        checksum
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct File {
    id: usize,
    offset: usize,
    size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range {
    offset: usize,
    size: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Record {
    Free(usize),
    Used(usize),
}
impl Record {
    fn blocks(&self) -> usize {
        match self {
            Record::Free(n) | Record::Used(n) => *n,
        }
    }
}

impl FromStr for Filesystem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut used = true;
        let mut layout = Vec::new();
        for c in s.trim().chars() {
            let n = c.to_digit(10).ok_or_else(|| Error::msg("Not a digit"))? as usize;
            if used {
                layout.push(Record::Used(n));
            } else {
                layout.push(Record::Free(n));
            }
            used = !used;
        }
        Ok(Filesystem(layout))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_parse() {
        let fs = Filesystem::from_str(SAMPLE).unwrap();
        assert_eq!(fs[0], Record::Used(2));
        assert_eq!(fs[1], Record::Free(3));
        assert_eq!(fs[2], Record::Used(3));
        assert_eq!(fs[3], Record::Free(3));
        assert_eq!(fs[4], Record::Used(1));
    }

    #[test]
    fn test_checksum() {
        let fs = Filesystem::from_str("232").unwrap(); // 0011.
        assert_eq!(fs.block_compacted_checksum(), 5);
        let fs = Filesystem::from_str("12345").unwrap(); // 022111222......
        assert_eq!(fs.block_compacted_checksum(), 60);
        let fs = Filesystem::from_str("2021").unwrap(); // 0011.
        assert_eq!(fs.block_compacted_checksum(), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 2858);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(9);
}
