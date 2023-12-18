use std::env;

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;
use regex::Regex;

/// Seeds
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
        get_puzzle_input(2023, 5, session).context("Could not retrieve puzzle input")?
    };

    let re = Regex::new(r"seeds: (.*)\n")?;

    let (_full_match, [seeds_match]) = re
        .captures(&input)
        .context("puzzle should contain seeds array!")?
        .extract();

    let mut seeds = vec![];
    for seed in seeds_match.split_ascii_whitespace() {
        let seed = u64::from_str_radix(seed, 10).context("Seeds can only be numbers!")?;
        seeds.push(seed);
    }

    let maps: Vec<Vec<(u64, u64, u64)>> = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    .iter()
    .map(|map_name| parse_map(&input, *map_name))
    .collect::<Result<Vec<_>>>()?;

    let mut location = u64::MAX;
    dbg!(&seeds);
    for seed in seeds.iter() {
        let mut next = *seed;
        for map in maps.iter() {
            for (dest,source,len) in map.iter() {
                if (*source..(source+len)).contains(&next) {
                    // println!("contains next: {}, dest: {}, source: {}, len: {}",next,dest,source,len);
                    next = dest + (next - source);
                    break;
                }
            }
        }
        location = location.min(next);
    }
    println!("part one: {}",location);
    Ok(())
}

fn parse_map(input: &str, start_of_map: &str) -> Result<Vec<(u64, u64, u64)>, anyhow::Error> {
    let mut nums = input
        .split_ascii_whitespace()
        // skip until map name
        .skip_while(|word| *word != start_of_map)
        // skip map name and "map:" word
        .skip(2)
        // take every number
        .take_while(|word| word.chars().all(|ch| ch.is_ascii_digit()));

    let mut map = vec![];
    while let Some(dest) = nums.next() {
        let dest = u64::from_str_radix(dest, 10)?;
        let source = u64::from_str_radix(nums.next().unwrap(), 10)?;
        let len = u64::from_str_radix(nums.next().unwrap(), 10)?;
        map.push((dest, source, len))
    }
    Ok(map)
}
