//! [Advent of Code Day 7](https://adventofcode.com/2023/day/7)

use crate::prelude::*;
use std::collections::HashMap;

/// Calculate winnings for Camel Cards hands based on bid
pub fn part1(input: &str) -> Result<u32> {
    let camel_cards = CamelCards::parse(input)?;
    Ok(camel_cards.winnings())
}

/// Calculate winnings for Camel Cards hands based on bid using 'J' as Joker
pub fn part2(input: &str) -> Result<u32> {
    let camel_cards = CamelCards::parse_with_jokers(input)?;
    Ok(camel_cards.winnings())
}

#[derive(Debug, Clone, Deref)]
/// Represents all hands and bids from a Camel Cards game
pub struct CamelCards(Vec<HandBid>);

#[derive(Debug, Clone, Copy, Eq)]
/// A hand of 5 cards
///
/// Note: Hand comparison is based on rank
/// use `Hand::cards` to compare actual cards
pub struct Hand([Card; 5]);

impl Hand {
    /// Get the cards from a hand
    pub fn cards(&self) -> [Card; 5] {
        self.0
    }

    /// Calculates the rank of a hand
    pub fn rank(&self) -> Rank {
        let mut map = HashMap::new();
        let mut jokers = 0;
        for card in self.0 {
            match card {
                Card::Joker => jokers += 1,
                card => {
                    let entry = map.entry(card).or_insert(0);
                    *entry += 1;
                }
            }
        }
        let mut counts = map.into_values().collect_vec();
        counts.sort();
        counts.reverse();

        // This can happen if we have 5 jokers
        if counts.is_empty() {
            counts.push(0);
        }

        counts[0] += jokers;
        match (counts[0], counts.get(1)) {
            (5, _) => Rank::FiveOfKind,
            (4, _) => Rank::FourOfKind,
            (3, Some(2)) => Rank::FullHouse,
            (3, _) => Rank::ThreeOfKind,
            (2, Some(2)) => Rank::TwoPair,
            (2, _) => Rank::Pair,
            _ => Rank::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// Ordered ranking of Camel Cards hands
pub enum Rank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, Clone, Copy)]
/// Represents a single hand and its bid
pub struct HandBid {
    pub hand: Hand,
    pub bid: u32,
}

impl CamelCards {
    /// Calculates the winnings for a full list of hands and their bids
    pub fn winnings(&self) -> u32 {
        let mut hand_bids = self.0.clone();
        hand_bids.sort_by_key(|hb| hb.hand);
        hand_bids
            .iter()
            .enumerate()
            .map(|(i, hb)| (i as u32 + 1) * hb.bid)
            .sum()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank().cmp(&other.rank()) {
            Ordering::Equal => {
                for i in 0..5 {
                    match self.0[i].cmp(&other.0[i]) {
                        Ordering::Equal => continue,
                        ordering => {
                            return ordering;
                        }
                    }
                }
                Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
/// Single Card. Joker is low (when not wild)
pub enum Card {
    Joker,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w{5}) (\d+)").unwrap());
impl CamelCards {
    pub fn parse(s: &str) -> Result<Self> {
        CamelCards::parse_with(s, Hand::parse)
    }

    pub fn parse_with_jokers(s: &str) -> Result<Self> {
        CamelCards::parse_with(s, Hand::parse_with_jokers)
    }

    fn parse_with(s: &str, parser_fn: impl Fn(&str) -> Result<Hand>) -> Result<CamelCards> {
        let hands = RE
            .captures_iter(s)
            .map(|cap| {
                let hand = parser_fn(&cap[1]).unwrap();
                let bid = cap[2].parse::<u32>().unwrap();
                HandBid { hand, bid }
            })
            .collect_vec();
        Ok(CamelCards(hands))
    }
}

impl Hand {
    pub fn parse(s: &str) -> Result<Hand> {
        Hand::parse_with(s, Card::new)
    }

    pub fn parse_with_jokers(s: &str) -> Result<Hand> {
        Hand::parse_with(s, Card::new_with_joker)
    }

    fn parse_with(s: &str, parse_fn: impl Fn(char) -> Card) -> Result<Hand> {
        let cards = s.chars().map(parse_fn).collect_vec();
        let len = cards.len();
        let hand: [Card; 5] = cards
            .try_into()
            .map_err(|_| format_err!("Bad hand - {} cards", len))?;
        Ok(Hand(hand))
    }
}

impl Card {
    pub fn new(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            _ => panic!("Unsupported card character '{c}'"),
        }
    }

    pub fn new_with_joker(c: char) -> Card {
        match c {
            'J' => Card::Joker,
            c => Card::new(c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_parse() {
        let cc = CamelCards::parse(SAMPLE).unwrap();
        assert_eq!(cc.len(), 5);
        assert_eq!(
            cc[0].hand.cards(),
            [Card::_3, Card::_2, Card::T, Card::_3, Card::K]
        );
        assert_eq!(cc[0].bid, 765);
    }

    fn hand(cards: &str) -> Hand {
        Hand::parse(cards).unwrap()
    }

    #[test]
    fn test_rank() {
        assert_eq!(hand("AAAAA").rank(), Rank::FiveOfKind);
        assert_eq!(hand("AA8AA").rank(), Rank::FourOfKind);
        assert_eq!(hand("23332").rank(), Rank::FullHouse);
        assert_eq!(hand("TTT98").rank(), Rank::ThreeOfKind);
        assert_eq!(hand("23432").rank(), Rank::TwoPair);
        assert_eq!(hand("A23A4").rank(), Rank::Pair);
        assert_eq!(hand("23456").rank(), Rank::HighCard);
    }

    fn with_jokers(cards: &str) -> Hand {
        Hand::parse_with_jokers(cards).unwrap()
    }

    #[test]
    fn test_rank_with_jokers() {
        assert_eq!(with_jokers("32T3K").rank(), Rank::Pair);
        assert_eq!(with_jokers("T55J5").rank(), Rank::FourOfKind);
        assert_eq!(with_jokers("KK677").rank(), Rank::TwoPair);
        assert_eq!(with_jokers("KTJJT").rank(), Rank::FourOfKind);
        assert_eq!(with_jokers("QQQJA").rank(), Rank::FourOfKind);
        assert_eq!(with_jokers("JJJJJ").rank(), Rank::FiveOfKind);
    }

    #[test]
    fn test_hand_ordering() {
        let cc = CamelCards::parse(SAMPLE).unwrap();
        let mut hands = cc.iter().map(|hb| hb.hand).collect_vec();
        hands.sort();
        assert_eq!(hands[0].cards(), hand("32T3K").cards());
        assert_eq!(hands[1].cards(), hand("KTJJT").cards());
        assert_eq!(hands[2].cards(), hand("KK677").cards());
        assert_eq!(hands[3].cards(), hand("T55J5").cards());
        assert_eq!(hands[4].cards(), hand("QQQJA").cards());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 5905);
    }
}

#[cfg(feature = "bench")]
mod bench {
    bench_day!(7);
}
