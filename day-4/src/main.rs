use std::{collections::HashSet, env};

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;

/// Scratchcards
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
        get_puzzle_input(2023, 4, session).context("Could not retrieve puzzle input")?
    };

    let re = Regex::new(r"Card\s+(\d+): (.*)\s+\|\s+(.*)").unwrap();
    let mut sum_part_one = 0;

    // array of (matches, instances) pairs
    let mut v = Vec::new();
    for line in input.lines() {
        let (_m, [_card_number, n1, n2]) = re.captures(line).context("should match")?.extract();
        let mut winning_set = HashSet::new();
        let mut count = 0;

        for number in n1.split_ascii_whitespace() {
            let n = u32::from_str_radix(number, 10)?;
            winning_set.insert(n);
        }
        for number in n2.split_ascii_whitespace() {
            let n = u32::from_str_radix(number, 10)?;
            if winning_set.contains(&n) {
                count += 1;
            }
        }
        if count > 0 {
            sum_part_one = sum_part_one + 2u32.pow(count - 1);
        }
        // for part two, add the count as the number of matches
        // and a single instance (the original copy)
        v.push((count as usize, 1usize));
    }
    println!("part one: {}", sum_part_one);

    for index in 0..v.len() {
        let (matches, instances) = v[index];
        for _copy in 0..instances {
            for i in 0..matches {
                let old = v
                    .get_mut(index + i + 1)
                    .context("Puzzle input should not exceed table length")?;
                old.1 += 1;
            }
        }
    }
    println!("part two: {}", v.iter().map(|x| x.1).sum::<usize>());
    Ok(())
}
