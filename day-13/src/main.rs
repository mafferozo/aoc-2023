use std::{env, str::Chars};

use anyhow::{Context, Result, anyhow};
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
        get_puzzle_input(2023, 11, session).context("Could not retrieve puzzle input!")?
    };
    println!("{input}");

    Ok(())
}

/// A record is a line or column of a pattern
struct Record {
    data: String,
}

impl Record {
    fn parse_rows(pattern: &str) -> Vec<Record> {
        pattern
            .lines()
            .map(str::chars)
            .map(|chars| Record {
                data: chars.collect(),
            })
            .collect()
    }

    fn parse_columns(pattern: &str) -> Vec<Record> {
        let n = pattern.lines().next().unwrap().len();

        let mut v = vec![];
        for col in 0..n {
            let data = pattern.lines().map(|l| l.chars().nth(col).unwrap()).collect();
            v.push(Record {data});
        }
        v
    }
}

struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>
}
