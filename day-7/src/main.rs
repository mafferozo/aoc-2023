use std::{cmp::Ordering, collections::HashMap, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

/// Wait for it
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The puzzle input
    #[arg()]
    input: Option<String>,

    /// Advent of code session token
    #[arg(short, long)]
    session: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let session = args.session.or_else(|| env::var("SESSION").ok());
    let input = if let Some(s) = args.input {
        s
    } else {
        get_puzzle_input(2023, 7, session).context("Could not retrieve puzzle input!")?
    };

    let mut words = input.split_ascii_whitespace();
    let mut hands_and_bids = Vec::new();

    // build vec of hands and their bids
    while let Some(hand) = words.next() {
        let bids = u64::from_str_radix(words.next().unwrap(), 10)?;
        hands_and_bids.push((Hand::new(hand), bids));
    }

    // look ma, sort by hands!
    hands_and_bids.sort_by(|a, b| a.0.order_part_one(&b.0));

    // compute winnings
    let winnings: u64 = hands_and_bids
        .iter()
        .clone()
        .enumerate()
        // rank = index + 1
        .map(|(index, (_hand, points))| (index as u64 + 1) * points)
        .sum();

    println!("Part one: {}", winnings);

    // part two
    hands_and_bids.sort_by(|a, b| a.0.order_part_two(&b.0));

    // compute winnings
    let winnings: u64 = hands_and_bids
        .iter()
        .clone()
        .enumerate()
        // rank = index + 1
        .map(|(index, (_hand, points))| (index as u64 + 1) * points)
        .sum();

    println!("Part two: {}", winnings);

    Ok(())
}

// NOTE: The lexographic ordering of this struct is important in order to
// correctly derive PartialOrd and Ord for this puzzle. i.e., keep `kind` at the top of the struct..
// Also note: Vec already derives PartialOrd for us like we expect it to:
// It runs two iterators in parallel and stop at non-equal Ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    // The count of each unique character in the hand
    kind_part_one: u32,
    // Card has a different Ord impl than Card2
    cards_part_one: Vec<Card>,

    kind_part_two: u32,
    cards_part_two: Vec<Card2>,
}

impl Hand {
    fn new(value: &str) -> Hand {
        assert!(value.len() == 5);

        let mut frequency_map = HashMap::new();
        for ch in value.chars() {
            frequency_map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }
        // sort the values of the frequency map, and match on the first and optionally second count
        let mut k: Vec<_> = frequency_map.drain().map(|(_k, v)| v).collect();
        k.sort();


        let kind_part_one = match (k.pop().unwrap(), k.pop()) {
            (5, _) => 7,
            (4, _) => 6,
            (3, Some(2)) => 5,
            (3, _) => 4,
            (2, Some(2)) => 3,
            (2, _) => 2,
            _ => 1,
        };

        let mut frequency_map = HashMap::new();
        for ch in value.chars() {
            frequency_map.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        // count the number of jokers,
        // update the best card in the map; add the joker count
        // remove the jokers.
        let joker_count = frequency_map.remove_entry(&'J').map(|(k,v)| v);

        // sort the values of the frequency map, and match on the first and second count
        let mut k: Vec<_> = frequency_map.drain().map(|(_k, v)| v).collect();
        k.sort();

        let mut best = k.pop();
        if let Some(joker_count) = joker_count {
            best = best.map(|count| count+joker_count)
        }

        let kind_part_two = match (best, k.pop()) {
            // one more edge case here;
            // if we have no elements left in the map, we must have had 5 jokers
            (None, _) => 7,
            (Some(5), _) => 7,
            (Some(4), _) => 6,
            (Some(3), Some(2)) => 5,
            (Some(3), _) => 4,
            (Some(2), Some(2)) => 3,
            (Some(2), _) => 2,
            _ => 1,
        };

        Hand {
            kind_part_one,
            cards_part_one: value.chars().map(|ch| Card::new(ch)).collect(),

            kind_part_two,
            cards_part_two: value.chars().map(|ch| Card2::new(ch)).collect(),
        }
    }

    fn order_part_one(&self, other: &Hand) -> Ordering {
        match self.kind_part_one.cmp(&other.kind_part_one) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
        self.cards_part_one.cmp(&other.cards_part_one)
    }

    fn order_part_two(&self, other: &Hand) -> Ordering {
        match self.kind_part_two.cmp(&other.kind_part_two) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
        self.cards_part_two.cmp(&other.cards_part_two)
    }

    fn to_string(&self) -> String {
        self.cards_part_two.iter().map(|c| c.0).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card(char);

const ALLOWED: &'static [char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

impl Card {
    fn new(value: char) -> Card {
        assert!(ALLOWED.contains(&value));
        Card(value)
    }

    fn get_rank(&self) -> i32 {
        let mut rank = 0;

        for ch in ALLOWED.iter().rev() {
            rank += 1;
            if self.0 == *ch {
                return rank;
            }
        }
        0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_rank().partial_cmp(&other.get_rank())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card2(char);

impl Card2 {
    fn new(value: char) -> Card2 {
        assert!(ALLOWED.contains(&value));
        Card2(value)
    }

    fn get_rank(&self) -> i32 {
        let mut rank = 0;
        if self.0 == 'J' {
            return 0;
        }
        for ch in ALLOWED.iter().rev() {
            rank += 1;
            if self.0 == *ch {
                return rank;
            }
        }
        0
    }
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_rank().partial_cmp(&other.get_rank())
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
