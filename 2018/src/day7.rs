use daggy::{Dag, NodeIndex, Walker};
use fxhash::FxHashMap;
use regex::Regex;
use crate::Error;

// 2018 AoC Day 7 puzzle
// https://adventofcode.com/2018/day/7

pub fn build_graph(input: &str) -> Result<(Dag<u8, ()>, FxHashMap<u8, NodeIndex>), Error> {
    let edges: Vec<Edge> = input
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Result<_, _>>()?;

    let mut g = Dag::<u8, ()>::new();
    let mut node_map = FxHashMap::default();

    for edge in edges {
        let prev = *node_map
            .entry(edge.prev)
            .or_insert_with(|| g.add_node(edge.prev));
        let next = *node_map
            .entry(edge.next)
            .or_insert_with(|| g.add_node(edge.next));

        g.add_edge(prev, next, ()).expect("Your graph has a cycle");
    }
    Ok((g, node_map))
}

pub fn part1(input: &str) -> Result<String, Error> {
    let mut step_tracker = StepTracker::new(input)?;

    let mut ordered_steps = Vec::new();
    while let Some(step) = step_tracker.get_next_step() {
        ordered_steps.push(step);
        step_tracker.complete_step(step);
    }

    Ok(String::from_utf8_lossy(&*ordered_steps).to_string())
}

#[derive(Debug)]
struct Worker {
    step: Option<u8>,
    remaining: u32,
}

// This function convers the ascii byte value of a capital letter to the cardinal
// e.g. 'A' == 1, 'B' ==2, etc...
fn ascii_cardinal(step: u8) -> u32 {
    step as u32 - 64
}

pub fn part2(input: &str, worker_count: u32, base_time: u32) -> Result<u32, Error> {
    let mut step_tracker = StepTracker::new(input)?;

    let mut second = 0;
    let mut workers = Vec::new();
    for _ in 0..worker_count {
        workers.push(Worker {
            step: None,
            remaining: 0,
        })
    }

    while !step_tracker.is_complete() {
        for worker in &mut workers {
            if let Some(step) = worker.step {
                if worker.remaining > 0 {
                    worker.remaining -= 1
                } else if worker.remaining == 0 {
                    step_tracker.complete_step(step);
                    worker.step = None;
                }
            }
        }
        for worker in &mut workers {
            if worker.step.is_none() {
                if let Some(step) = step_tracker.get_next_step() {
                    worker.step = Some(step);
                    worker.remaining = base_time + ascii_cardinal(step) - 1;
                }
            }
        }

        second += 1;
    }

    Ok(second - 1)
}

// Step tracker contains a DAG and walks it in the order of lowest node value of nodes already reached
struct StepTracker {
    // The graph itself
    dag: Dag<u8, ()>,
    // ready_map keeps track of all the nodes that are ready to be walked. nodes are only ready once all their parents have been walked
    ready_map: FxHashMap<u8, NodeIndex>,
    // pending_map is nodes that are being processed (i.e. returned by get_next_step), but not yet completed (i.e. complete_step)
    pending_map: FxHashMap<u8, NodeIndex>,
    // keeps track of how many parents each node has that have not yet been removed.
    parent_counts: FxHashMap<u8, usize>,
}

impl StepTracker {
    fn new(input: &str) -> Result<StepTracker, Error> {
        let (dag, node_map) = build_graph(&input)?;

        let mut heads = Vec::new();
        for node in node_map.values() {
            if dag.parents(*node).iter(&dag).count() == 0 {
                heads.push(*node);
            }
        }

        let parent_counts: FxHashMap<_, usize> = node_map
            .values()
            .map(|n| {
                (
                    *dag.node_weight(*n).expect("no weight"),
                    dag.parents(*n).iter(&dag).count(),
                )
            })
            .collect();

        let ready_map: FxHashMap<u8, _> = heads
            .into_iter()
            .map(|n| (*dag.node_weight(n).expect("no weight"), n))
            .collect();

        Ok(StepTracker {
            dag,
            ready_map,
            parent_counts,
            pending_map: FxHashMap::default(),
        })
    }

    fn is_complete(&self) -> bool {
        self.ready_map.is_empty() && self.pending_map.is_empty()
    }

    fn get_next_step(&mut self) -> Option<u8> {
        if let Some(step) = self.ready_map.iter().map(|(step, _)| *step).min() {
            let val = self.ready_map.remove(&step).unwrap();
            self.pending_map.insert(step, val);
            Some(step)
        } else {
            None
        }
    }

    fn complete_step(&mut self, step: u8) {
        let node = self.pending_map[&step];
        let children: FxHashMap<_, _> = self
            .dag
            .children(node)
            .iter(&self.dag)
            .map(|(_, n)| (*self.dag.node_weight(n).expect("no weight"), n))
            .collect();

        for (k, _n) in &children {
            if let Some(x) = self.parent_counts.get_mut(&k) {
                if x > &mut 0 {
                    *x -= 1;
                }
            }
        }
        let children: FxHashMap<_, _> = children
            .iter()
            .filter(|&(k, _)| self.parent_counts[k] == 0)
            .collect();
        self.pending_map.remove(&step);
        self.ready_map.extend(children);
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    prev: u8,
    next: u8,
}

fn parse_line(input: &str) -> Result<Edge, Error> {
    let caps = RE
        .captures(input)
        .ok_or_else(|| format!("Line could not be parsed: {}", input))?;

    Ok(Edge {
        prev: caps[1].as_bytes()[0],
        next: caps[2].as_bytes()[0],
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "bench")]
    use test::Bencher;

    const INPUT: &str = r#"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "CABDFE");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT, 2, 0).unwrap(), 15);
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part1(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-7.txt").expect("Unable to open file");
        b.iter(|| part1(&input).unwrap());
    }

    #[cfg_attr(feature = "bench", bench)]
    #[cfg(feature = "bench")]
    fn bench_part2(b: &mut Bencher) {
        let input = ::std::fs::read_to_string("inputs/day-7.txt").expect("Unable to open file");
        b.iter(|| part2(&input, 5, 60).unwrap());
    }
}
