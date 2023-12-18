use std::env;

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
        get_puzzle_input(2023, 9, session).context("Could not retrieve puzzle input!")?
    };

    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|word| i32::from_str_radix(word, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    for history in histories.into_iter() {
        let mut result = vec![history.clone()];
        let mut stack = vec![history];
        while let Some(history) = stack.pop() {
            let diff = history.windows(2).map(|n| n[1] - n[0]).collect::<Vec<_>>();
            result.push(diff.clone());

            if diff.iter().any(|n| n != &0) {
                stack.push(diff);
            }
        }
        sum_part_one += result
            .clone()
            .into_iter()
            .map(|v| *v.last().unwrap())
            .sum::<i32>();

        let mut d = 0;
        for n in result.iter().rev().map(|v| *v.first().unwrap()) {
            d = n-d;
        }
        sum_part_two += d;
    }
    println!("Part one: {}", sum_part_one);
    println!("Part two: {}", sum_part_two);
    Ok(())
}
