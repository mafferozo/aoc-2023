use std::env;

use anyhow::{anyhow, Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

use regex::Regex;
use Direction::*;

/// Lavaduct Lagoon
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
        get_puzzle_input(2023, 18, session).context("Could not retrieve puzzle input")?
    };
    println!("{input}");
    let instructions = DigInstruction::parse_input(&input)?;
    dbg!(instructions);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct DigInstruction {
    /// The direction to dig in
    direction: Direction,

    /// The trench length in meters
    trench_length: u32,

    /// color of the trench 0x00RRGGBB
    trench_color: u32,
}

impl DigInstruction {
    fn parse_input(input: &str) -> Result<Vec<Self>> {
        let re = Regex::new(r"^(\w) (\d+) \(#(.*)\)$").unwrap();

        let mut instructions = vec![];
        for line in input.lines() {
            let (_m, [dir, length, color]) = re.captures(line).context("should match")?.extract();
            instructions.push(DigInstruction {
                direction: Direction::parse(dir)?,
                trench_length: u32::from_str_radix(length, 10)?,
                trench_color: u32::from_str_radix(color, 16)?,
            });
        }

        Ok(instructions)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn parse(m: &str) -> Result<Self> {
        match m {
            "U" => Ok(Up),
            "R" => Ok(Right),
            "D" => Ok(Left),
            "L" => Ok(Down),
            _ => Err(anyhow!("Should match")),
        }
    }
}
