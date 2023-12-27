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
        get_puzzle_input(2023, 13, session).context("Could not retrieve puzzle input!")?
    };

    let patterns: Vec<_> = input.split("\n\n").map(Pattern::parse_pattern).collect();

    let mut sum_part_one = 0;
    for p in patterns.iter() {
        let v = p.find_reflection();
        sum_part_one += if v.1 { v.0 } else { (v.0) * 100 }
    }
    println!("Part one: {}", sum_part_one);

    let mut sum_part_two = 0;
    for p in patterns.iter() {
        let v = p.find_reflection_part_two();
        sum_part_two += if v.1 { v.0 } else { (v.0) * 100 }
    }
    println!("Part two: {}", sum_part_two);

    Ok(())
}

/// A record is a line or column of a pattern
#[derive(Debug, Clone, PartialEq, Eq)]
struct Record(String);

/// A Pattern has two representations of the same data:
/// rows are the lines of a pattern
/// cols are the columns of a pattern
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    rows: Vec<Record>,
    cols: Vec<Record>,
}

impl Pattern {
    fn parse_pattern(pattern: &str) -> Pattern {
        Pattern {
            rows: Self::parse_rows(pattern),
            cols: Self::parse_columns(pattern),
        }
    }

    fn parse_rows(pattern: &str) -> Vec<Record> {
        pattern
            .lines()
            .map(str::chars)
            .map(|chars| Record(chars.collect()))
            .collect()
    }

    fn parse_columns(pattern: &str) -> Vec<Record> {
        let n = pattern.lines().next().unwrap().len();
        let mut v = vec![];
        for col in 0..n {
            let data = pattern
                .lines()
                .map(|l| l.chars().nth(col).unwrap())
                .collect();
            v.push(Record(data));
        }
        v
    }

    fn find_ref_part_one(v: &[Record]) -> Option<usize> {
        assert!(v.len() > 1);
        (1..v.len()).find(|i| {
            let left = v[0..*i].iter().rev();
            let right = v[*i..v.len()].iter();
            left.zip(right).all(|(a, b)| a == b)
        })
    }

    fn find_ref_part_two(v: &[Record]) -> Option<usize> {
        assert!(v.len() > 1);

        (1..v.len()).find(|i| {
            let left = v[0..*i].iter().rev();
            let right = v[*i..v.len()].iter();

            left.zip(right).map(|(a, b)| a.count_differences(b)).sum::<usize>() == 1
        })
    }

    /// Find the line of reflection
    /// Returns (x,true) when the reflection is vertical
    fn find_reflection(&self) -> (usize, bool) {
        let horizontal = Self::find_ref_part_one(&self.rows).map(|u| (u, false));
        let vertical = Self::find_ref_part_one(&self.cols).map(|u| (u, true));
        horizontal.or(vertical).unwrap()
    }

    /// Find the line of reflection
    /// Returns (x,true) when the reflection is vertical
    fn find_reflection_part_two(&self) -> (usize, bool) {
        let horizontal = Self::find_ref_part_two(&self.rows).map(|u| (u, false));
        let vertical = Self::find_ref_part_two(&self.cols).map(|u| (u, true));
        horizontal.or(vertical).unwrap()
    }
}

impl Record {
    fn count_differences(&self, other: &Record) -> usize {
        self.0
            .chars()
            .zip(other.0.chars())
            .map(|(a, b)| if a != b { 1 } else { 0 })
            .sum()
    }
}
