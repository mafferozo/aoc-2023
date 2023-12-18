use std::env;

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;

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
        get_puzzle_input(2023, 6, session).context("Could not retrieve puzzle input!")?
    };
    dbg!(&input);

    let re = Regex::new(r"Time:\s+(.*)\nDistance:\s+(.*)")?;
    let (_m, [time, dist]) = re.captures(&input).context("should match")?.extract();
    let race_times: Vec<u32> = time
        .split_ascii_whitespace()
        .map(|w| u32::from_str_radix(w, 10).context("not a number"))
        .collect::<Result<Vec<_>>>()?;

    let record_dists: Vec<u32> = dist
        .split_ascii_whitespace()
        .map(|w| u32::from_str_radix(w, 10).context("not a number"))
        .collect::<Result<Vec<_>>>()?;

    let mut sum_part_one = 1;
    for (index, time) in race_times.iter().copied().enumerate() {
        let record_dist = *record_dists.get(index).unwrap();
        // distances recorded by simulation
        let mut dists = vec![];
        for i in 0..time {
            let time_left = time - i;
            let speed = i;
            let distance_traveled = speed * time_left;
            dists.push(distance_traveled);
        }
        // looking at this is interesting; the vector is symmetric
        dbg!(&dists);
        let margin: usize = dists.into_iter().filter(|d| *d > record_dist).count();
        sum_part_one *= margin;
    }
    println!("part one: {}", sum_part_one);

    // part two
    let time = u64::from_str_radix(&time.split_ascii_whitespace().collect::<String>(), 10)?;
    let record_dist = u64::from_str_radix(&dist.split_ascii_whitespace().collect::<String>(), 10)?;

    let mut dists = vec![];
    for i in 0..time {
        let time_left = time - i;
        let speed = i;
        let distance_traveled = speed * time_left;
        dists.push(distance_traveled);
    }
    let margin: usize = dists.into_iter().filter(|d| *d > record_dist).count();
    println!("part two: {}", margin);
    Ok(())
}
