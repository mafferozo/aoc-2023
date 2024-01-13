use std::env;

use anyhow::{Context, Result};
use aoc_input_lib::get_puzzle_input;
use clap::Parser;

/// Pulse Propagation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = Option::None)]
pub struct Args {
    /// The puzzle input
    #[arg()]
    pub input: Option<String>,

    /// Advent of code session token
    #[arg(short, long)]
    pub session: Option<String>,
}

pub fn get_input(year: u32, day: u32) -> Result<String> {
    let args = Args::parse();
    let session = args.session.or_else(|| env::var("SESSION").ok());

    if let Some(s) = args.input {
        Ok(s)
    } else {
        Ok(get_puzzle_input(year, day, session).context("Could not retrieve puzzle input")?)
    }
}
