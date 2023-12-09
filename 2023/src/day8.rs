//! [Advent of Code Day 8](https://adventofcode.com/2023/day/8)

use crate::prelude::*;
use std::collections::{BTreeSet, HashMap};

/// Count the steps traversing a map from 'AAA' to 'ZZZ'
pub fn part1(input: &str) -> Result<u32> {
    let map = Map::from_str(input)?;
    let count = map.steps_to_zzz();
    Ok(count)
}

/// Calculate the steps for parallel map traversal from '**A' ending on '**Z' nodes at the same time
///
/// This implementation is limited to inputs with loops where a solution node loops back to the head's first node
pub fn part2(input: &str) -> Result<usize> {
    let map = Map::from_str(input)?;
    let count = map.ghost_steps();
    Ok(count)
}

#[derive(Debug, Clone)]
/// Represents the entire map: direction list and network of nodes
pub struct Map {
    network: Network,
    dirs: Vec<Dir>,
}

impl Map {
    pub fn steps_to_zzz(&self) -> u32 {
        let mut node = Node::new("AAA");
        let mut count = 0;
        let end = Node::new("ZZZ");
        while node != end {
            for d in &self.dirs {
                node = self.network.follow(node, *d);
                count += 1;
            }
            assert!(count < 1_000_000);
        }
        count
    }

    /// This solution naively assumes the paths cycle with solutions at the ends of the cycle
    /// This holds true for the provided input, but without it, the solution would be more complicated
    /// With offset end nodes, it would require using the Chinese Remainder Theorem.
    /// And with multiple end nodes in a cycle, it would require implementing that over the cartesian product
    /// of the end nodes per cycle
    ///
    /// Instead, we panic if the cycle length doesn't match one of the end nodes
    /// And we just log a warning if extra end nodes are found (since they could result in a smaller LCM)
    pub fn ghost_steps(&self) -> usize {
        let cycles = self.find_all_cycles();

        let mut lcm = 1;
        for cycle in cycles {
            if cycle.ends.len() > 1 {
                eprintln!(
                    "WARN: found cycle with multiple end states {:?}",
                    cycle.ends
                );
            }
            let end = *cycle.ends.last().unwrap();

            assert_eq!(cycle.length, end);
            lcm = num::integer::lcm(lcm, end);
        }
        lcm
    }

    pub fn find_all_cycles(&self) -> Vec<Cycle> {
        self.network
            .starting_points()
            .into_iter()
            .map(|start| self.find_cycle(start))
            .collect()
    }

    pub fn find_cycle(&self, start: Node) -> Cycle {
        let last_dir_i = self.dirs.len() - 1;
        let mut visited = HashMap::from([(NodeHash(start, last_dir_i), 0)]);
        let mut ends = BTreeSet::new();
        let mut current = start;
        let mut i = 0;
        loop {
            for (dir_i, dir) in self.dirs.iter().enumerate() {
                let next = self.network.follow(current, *dir);
                i += 1;
                if next[2] == 'Z' {
                    ends.insert(i);
                }

                if let Some(offset) = visited.insert(NodeHash(next, dir_i), i) {
                    let length = visited.len() - offset;
                    let cycle = Cycle {
                        offset,
                        length,
                        ends,
                    };
                    // eprintln!("Cycle at {}: {:?}", next, cycle);
                    return cycle;
                }
                current = next;
            }
            assert!(i < 100_000_000);
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct NodeHash(Node, usize);

#[derive(Debug, Clone, Deref)]
pub struct Network(HashMap<Node, (Node, Node)>);

#[derive(Debug, Clone, Deref, Copy, PartialEq, Eq, Hash)]
pub struct Node([char; 3]);

use std::fmt::{self, Formatter};
impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}{}{}", self[0], self[1], self[2])
    }
}

impl Node {
    pub fn new(s: &str) -> Node {
        Node(s.chars().collect_vec().try_into().unwrap())
    }
}

impl Network {
    pub fn follow(&self, node: Node, dir: Dir) -> Node {
        let node_dest = self[&node];
        match dir {
            Dir::Left => node_dest.0,
            Dir::Right => node_dest.1,
        }
    }
    pub fn starting_points(&self) -> Vec<Node> {
        self.keys().filter(|n| n[2] == 'A').cloned().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cycle {
    offset: usize,
    length: usize,
    ends: BTreeSet<usize>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Dir {
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = Error;
    fn try_from(c: char) -> Result<Dir> {
        match c {
            'R' => Ok(Dir::Right),
            'L' => Ok(Dir::Left),
            _ => bail!("Unsupported direction: '{c}'"),
        }
    }
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap());

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let dirs: Vec<Dir> = lines
            .next()
            .context("Missing direction list")?
            .chars()
            .map(Dir::try_from)
            .try_collect()?;

        let mut network_map = HashMap::new();
        for cap in RE.captures_iter(s) {
            let node = Node::new(&cap[1]);
            let left = Node::new(&cap[2]);
            let right = Node::new(&cap[3]);

            network_map.insert(node, (left, right));
        }
        let network = Network(network_map);
        Ok(Map { dirs, network })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"};

    const SAMPLE2: &str = indoc! {"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"};

    const SAMPLE3: &str = indoc! {"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"};

    fn c(s: &str) -> Node {
        Node::new(s)
    }

    #[test]
    fn test_parse_map() {
        let map = Map::from_str(SAMPLE).unwrap();
        assert_eq!(map.dirs, vec![Dir::Right, Dir::Left]);
        assert_eq!(map.network.len(), 7);
        assert_eq!(map.network[&c("CCC")], (c("ZZZ"), c("GGG")));
    }

    #[test]
    fn test_follow() {
        let map = Map::from_str(SAMPLE).unwrap();
        assert_eq!(map.network.follow(c("CCC"), Dir::Left), c("ZZZ"));
        assert_eq!(map.network.follow(c("CCC"), Dir::Right), c("GGG"));
    }

    #[test]
    fn test_cycles() {
        let map = Map::from_str(SAMPLE3).unwrap();
        assert_eq!(
            map.find_cycle(c("11A")),
            Cycle {
                offset: 1,
                length: 2,
                ends: maplit::btreeset!(2)
            }
        );
        assert_eq!(
            map.find_cycle(c("22A")),
            // Arguably this is a cycle of length 3, but it just happens to follow the same nodes
            // using opposite path directions
            Cycle {
                offset: 1,
                length: 6,
                ends: maplit::btreeset!(3, 6)
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 2);
        assert_eq!(part1(SAMPLE2).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE3).unwrap(), 6);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(8);
}
