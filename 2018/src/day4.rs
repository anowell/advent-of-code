use fxhash::FxHashMap;
use regex::Regex;
use crate::Error;

// 2018 AoC Day 4 puzzle
// https://adventofcode.com/2018/day/4

// Builds a Map
fn guard_map(input: &str) -> Result<FxHashMap<u32, Vec<u64>>, Error> {
    // Sort the lines since order is how we determine which guard a log entry refers to
    let mut lines: Vec<_> = input.trim().lines().collect();
    lines.sort();

    // Parse the lines
    let mut entries = Vec::new();
    for line in lines {
        let entry = parse_input(&line)?;
        entries.push(entry);
    }

    // Find each Sleep->Wake cycle and turn it into a bitmap of the minutes asleep
    // 60 minutes worth of bits fits nicely into a u64
    let mut guard_map = FxHashMap::default();
    let mut current_guard = 0_u32; // Guard 0 isn't really a valid starting point, but works as long as first record is Event::Wake
    let mut sleep_min = 0;
    for entry in entries {
        match entry.evt {
            Event::Start(id) => {
                current_guard = id;
                sleep_min = if entry.hr == 0 { entry.min } else { 0 };
            }
            Event::Sleep => {
                sleep_min = if entry.hr == 0 { entry.min } else { 0 };
            }
            Event::Wake => {
                let schedules = guard_map.entry(current_guard).or_insert_with(Vec::new);
                let wake_min = if entry.hr == 0 { entry.min } else { 0 };
                // println!("Guard {}: sleeping {}..{}", current_guard, sleep_min, wake_min);
                let mut schedule = 0_u64;
                for i in sleep_min..wake_min {
                    schedule |= 1_u64 << i;
                }
                schedules.push(schedule);
            }
        }
    }
    Ok(guard_map)
}

pub fn part1(input: &str) -> Result<u32, Error> {
    let guard_map = guard_map(input)?;

    // Find the guard that sleeps the most
    let sleepy_guard = guard_map
        .iter()
        .max_by_key(|&(_k, v)| v.iter().map(|n| n.count_ones()).sum::<u32>())
        .unwrap();

    // Find the sleepiest minute for the sleepy guard
    let mut sleepiest_minute = 0;
    let mut sleepiest_count = 0;
    for i in 0..60 {
        let mut sum = 0;
        for schedule in sleepy_guard.1 {
            if schedule & (1_u64 << i) != 0 {
                sum += 1
            }
        }
        if sum > sleepiest_count {
            sleepiest_count = sum;
            sleepiest_minute = i;
        }
    }

    Ok(sleepy_guard.0 * sleepiest_minute)
}

pub fn part2(input: &str) -> Result<u32, Error> {
    let guard_map = guard_map(input)?;

    // Find the sleepiest minute among all guards
    let mut sleepiest_minute = 0;
    let mut sleepiest_count = 0;
    let mut sleepiest_guard = 0;
    for (guard, schedules) in guard_map {
        for i in 0..60 {
            let mut sum = 0;
            for schedule in &schedules {
                if schedule & (1_u64 << i) != 0 {
                    sum += 1
                }
            }
            if sum > sleepiest_count {
                sleepiest_count = sum;
                sleepiest_minute = i;
                sleepiest_guard = guard
            }
        }
    }

    Ok(sleepiest_guard * sleepiest_minute)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\[\d+-(\d+)-(\d+) (\d+):(\d+)\] (.+)$").unwrap();
}

#[derive(Debug)]
enum Event {
    Start(u32),
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Entry {
    mo: u8,
    day: u8,
    hr: u8,
    min: u8,
    evt: Event,
}

fn parse_input(input: &str) -> Result<Entry, Error> {
    let caps = RE
        .captures(input)
        .ok_or_else(|| format!("Claim could not be parsed: {}", input))?;

    let evt = match caps[5].trim() {
        "falls asleep" => Event::Sleep,
        "wakes up" => Event::Wake,
        msg => {
            let guard = msg.split(' ').nth(1).unwrap()[1..].parse()?;
            Event::Start(guard)
        }
    };

    Ok(Entry {
        mo: caps[1].parse()?,
        day: caps[2].parse()?,
        hr: caps[3].parse()?,
        min: caps[4].parse()?,
        evt: evt,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    const INPUT: &str = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 4455);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-4.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-4.txt").expect("Unable to open file");
        b.iter(|| part2(&input).unwrap());
    }
}
